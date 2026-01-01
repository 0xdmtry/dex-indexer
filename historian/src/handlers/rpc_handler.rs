use crate::models::consts::{PUMPSWAP_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID};
use crate::models::enums::RpcAccountData;
use crate::models::enums::RpcAccountType;
use crate::models::resolver::{ExtendedMetadata, ResolvedTokenMetadata};
use crate::models::rpc::{
    BondingCurveAccount, BondingCurveAccountSpl, BondingCurveAccountToken2022, ExtractMintError,
    PoolAccount, PoolState, PriceError, RpcAccount,
};
use borsh::BorshDeserialize;
use log::error;
use mpl_token_metadata::accounts::Metadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_response::RpcKeyedAccount;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::Mint,
};
use spl_token_metadata_interface::state::TokenMetadata;
use std::convert::TryInto;
use std::path::Path;
use std::str::FromStr;

pub async fn get_account_by_bonding_curve(
    rpc_client: &RpcClient,
    bonding_curve: &str,
    program_ids: Vec<&str>,
) -> Result<RpcKeyedAccount, ExtractMintError> {
    let bonding_curve_pubkey = Pubkey::from_str(bonding_curve)?;

    for program_id in program_ids {
        let program_id_pubkey = Pubkey::from_str(program_id)?;

        let accounts = rpc_client
            .get_token_accounts_by_owner(
                &bonding_curve_pubkey,
                solana_client::rpc_request::TokenAccountsFilter::ProgramId(program_id_pubkey),
            )
            .await;

        if let Ok(accounts) = accounts {
            if let Some(account) = accounts.first() {
                return Ok(account.clone());
            }
        }
    }

    Err(ExtractMintError::NoTokenAccounts)
}

pub fn extract_mint_from_account(account: &RpcKeyedAccount) -> Result<Pubkey, ExtractMintError> {
    let parsed = match &account.account.data {
        solana_account_decoder::UiAccountData::Json(parsed) => parsed,
        _ => return Err(ExtractMintError::DataNotParsed),
    };

    let info = parsed
        .parsed
        .get("info")
        .and_then(|v| v.as_object())
        .ok_or(ExtractMintError::InvalidJson)?;

    let mint_str = info
        .get("mint")
        .and_then(|v| v.as_str())
        .ok_or(ExtractMintError::MintNotFound)?;

    Ok(Pubkey::from_str(mint_str)?)
}

pub async fn get_account_by_address(
    rpc_client: &RpcClient,
    address: &str,
    program_ids: Vec<&str>,
) -> Option<RpcAccount> {
    let pubkey = Pubkey::from_str(address).unwrap(); // TODO re-do unwrap()

    match rpc_client.get_account(&pubkey).await {
        Ok(account) => {
            for program_id in program_ids {
                let program_pubkey = Pubkey::from_str(program_id).unwrap();

                if account.owner == program_pubkey {
                    match program_id {
                        PUMPSWAP_PROGRAM_ID | SPL_TOKEN_PROGRAM_ID | TOKEN_2022_PROGRAM_ID => {
                            if let Ok(bc) =
                                BondingCurveAccountToken2022::try_from_slice(&account.data[8..])
                            {
                                return Some(RpcAccount {
                                    account_type: RpcAccountType::PumpFunToken2022,
                                    account_data: RpcAccountData::PumpFunToken2022(
                                        BondingCurveAccount {
                                            virtual_token_reserves: bc.virtual_token_reserves,
                                            virtual_sol_reserves: bc.virtual_sol_reserves,
                                            real_token_reserves: bc.real_token_reserves,
                                            real_sol_reserves: bc.real_sol_reserves,
                                            token_total_supply: bc.token_total_supply,
                                            complete: bc.complete,
                                            creator: bc.creator,
                                        },
                                    ),
                                });
                            }

                            if let Ok(bc) =
                                BondingCurveAccountSpl::try_from_slice(&account.data[8..])
                            {
                                return Some(RpcAccount {
                                    account_type: RpcAccountType::PumpFunSpl,
                                    account_data: RpcAccountData::PumpFunSpl(BondingCurveAccount {
                                        virtual_token_reserves: bc.virtual_token_reserves,
                                        virtual_sol_reserves: bc.virtual_sol_reserves,
                                        real_token_reserves: bc.real_token_reserves,
                                        real_sol_reserves: bc.real_sol_reserves,
                                        token_total_supply: bc.token_total_supply,
                                        complete: bc.complete,
                                        creator: bc.creator,
                                    }),
                                });
                            }
                        }
                        _ => {
                            return match parse_pool_state(&account.data) {
                                Ok(pool_state) => Some(RpcAccount {
                                    account_type: RpcAccountType::RaydiumLaunchLab,
                                    account_data: RpcAccountData::RaydiumLaunchLab(pool_state),
                                }),
                                Err(e) => {
                                    error!("Failed to parse pool state: {e}");
                                    None
                                }
                            };
                        }
                    }
                }
            }
            None
        }
        Err(e) => {
            error!("Failed to get bonding curve: {e}");
            None
        }
    }
}

pub fn get_bonding_curve_price(bc: BondingCurveAccount, decimal: u8) -> Option<i64> {
    if bc.virtual_token_reserves == 0 {
        return None;
    }

    let price = (bc.virtual_sol_reserves as f64 / bc.virtual_token_reserves as f64
        * 10_f64.powi(decimal as i32)) as i64;

    Some(price)
}

pub fn extract_decimals_from_account(account: &RpcKeyedAccount) -> Result<u8, ExtractMintError> {
    let parsed = match &account.account.data {
        solana_account_decoder::UiAccountData::Json(parsed) => parsed,
        _ => return Err(ExtractMintError::DataNotParsed),
    };

    let info = parsed
        .parsed
        .get("info")
        .and_then(|v| v.as_object())
        .ok_or(ExtractMintError::InvalidJson)?;

    let decimals = info
        .get("tokenAmount")
        .and_then(|v| v.get("decimals"))
        .and_then(|v| v.as_u64())
        .ok_or(ExtractMintError::DecimalsNotFound)?;

    Ok(decimals as u8)
}

pub async fn get_pool(rpc_client: &RpcClient, address: &str) -> Option<PoolAccount> {
    let pubkey = Pubkey::from_str(address).unwrap();
    let pumpswap_program = Pubkey::from_str(PUMPSWAP_PROGRAM_ID).unwrap();

    match rpc_client.get_account(&pubkey).await {
        Ok(account) => {
            if account.owner != pumpswap_program {
                return None;
            }

            if account.data.len() < 8 {
                return None;
            }

            match PoolAccount::try_from_slice(&account.data[8..]) {
                Ok(pool) => Some(pool),
                Err(e) => {
                    error!("Failed to get pool account 1: {e}");
                    None
                }
            }
        }
        Err(e) => {
            error!("Failed to get pool account 2: {e}");
            None
        }
    }
}

pub async fn get_pool_price(
    rpc_client: &RpcClient,
    pool: PoolAccount,
    decimal: Option<u8>,
) -> Result<Option<i64>, PriceError> {
    let base_amount: Option<u64> = get_token_account_balance(
        rpc_client,
        &pool.pool_base_token_account,
        TOKEN_2022_PROGRAM_ID,
    )
    .await
    .unwrap();

    let quote_amount: Option<u64> = get_token_account_balance(
        rpc_client,
        &pool.pool_quote_token_account,
        SPL_TOKEN_PROGRAM_ID,
    )
    .await
    .unwrap();

    let price = quote_amount
        .zip(base_amount)
        .zip(decimal)
        .map(|((q, b), d)| {
            let quote_adjusted = q as f64 / 1_000_000_000.0; // SOL has 9 decimals
            let base_adjusted = b as f64 / 10_f64.powi(d as i32); // Token has d decimals
            (quote_adjusted / base_adjusted * 1_000_000_000.0) as i64
        });

    let _toke_metadata = get_token_metadata(rpc_client, &pool.base_mint)
        .await
        .unwrap(); // TODO review necessity of _toke_metadata

    match price {
        Some(p) => Ok(Some(p)),
        None => Ok(None),
    }
}

pub async fn get_pool_price_with_rpc(
    rpc_client: &RpcClient,
    address: &str,
) -> Result<Option<i64>, PriceError> {
    let pubkey = Pubkey::from_str(address)?;
    let pumpswap_program = Pubkey::from_str(PUMPSWAP_PROGRAM_ID)?;

    match rpc_client.get_account(&pubkey).await {
        Ok(account) => {
            if account.owner != pumpswap_program {
                return Ok(None);
            }

            if account.data.len() < 8 {
                return Ok(None);
            }

            let pool = match PoolAccount::try_from_slice(&account.data[8..]) {
                Ok(pool) => pool,
                Err(e) => {
                    error!("Failed to get pool account 3: {e}");
                    return Ok(None);
                }
            };

            let base_amount: Option<u64> = get_token_account_balance(
                rpc_client,
                &pool.pool_base_token_account,
                TOKEN_2022_PROGRAM_ID,
            )
            .await
            .unwrap();
            let quote_amount: Option<u64> = get_token_account_balance(
                rpc_client,
                &pool.pool_quote_token_account,
                SPL_TOKEN_PROGRAM_ID,
            )
            .await
            .unwrap();

            let decimal = get_mint_decimals(rpc_client, &pool.base_mint)
                .await
                .unwrap();

            let price = quote_amount
                .zip(base_amount)
                .zip(decimal)
                .map(|((q, b), d)| {
                    let quote_adjusted = q as f64 / 1_000_000_000.0; // SOL has 9 decimals
                    let base_adjusted = b as f64 / 10_f64.powi(d as i32); // Token has d decimals
                    (quote_adjusted / base_adjusted * 1_000_000_000.0) as i64
                });

            let _toke_metadata = get_token_metadata(rpc_client, &pool.base_mint)
                .await
                .unwrap(); // TODO review necessity of _toke_metadata

            match price {
                Some(p) => Ok(Some(p)),
                None => Ok(None),
            }
        }
        Err(e) => {
            error!("Failed to get pool account 4: {e}");
            Ok(None)
        }
    }
}

//////////////////////////////////////////
/////////////////////////////////////////

pub async fn get_token_account_balance(
    rpc_client: &RpcClient,
    token_account: &Pubkey,
    program_id: &str,
) -> Result<Option<u64>, PriceError> {
    let token_program = Pubkey::from_str(program_id)?;

    match rpc_client.get_account(token_account).await {
        Ok(account) => {
            if account.owner != token_program {
                return Ok(None);
            }

            if account.data.len() < 72 {
                return Ok(None);
            }

            // SPL Token Account: amount is at bytes 64-72
            let amount_bytes: [u8; 8] = account.data[64..72].try_into().unwrap();
            let amount = u64::from_le_bytes(amount_bytes);

            Ok(Some(amount))
        }
        Err(_) => Ok(None),
    }
}

pub async fn get_mint_decimals(
    rpc_client: &RpcClient,
    mint: &Pubkey,
) -> Result<Option<u8>, PriceError> {
    let spl_token = Pubkey::from_str(SPL_TOKEN_PROGRAM_ID)?;
    let token_2022 = Pubkey::from_str(TOKEN_2022_PROGRAM_ID)?;

    match rpc_client.get_account(mint).await {
        Ok(account) => {
            if account.owner != spl_token && account.owner != token_2022 {
                return Ok(None);
            }

            if account.data.len() < 45 {
                return Ok(None);
            }

            // Mint account: decimals at byte 44
            let decimals = account.data[44];

            Ok(Some(decimals))
        }
        Err(_) => Ok(None),
    }
}

//////////////////////////////////////////
/////////////////////////////////////////
//////////////////////////////////////////
/////////////////////////////////////////

pub async fn get_token_metadata(
    rpc_client: &RpcClient,
    mint: &Pubkey,
) -> Result<Option<ResolvedTokenMetadata>, PriceError> {
    if let Some(metadata) = get_token2022_token_metadata(rpc_client, mint).await? {
        return Ok(Some(metadata));
    }

    get_spl_token_metadata(rpc_client, mint).await
}

// TODO review proper failing
pub async fn get_token2022_token_metadata(
    rpc_client: &RpcClient,
    mint: &Pubkey,
) -> Result<Option<ResolvedTokenMetadata>, PriceError> {
    let account = match rpc_client.get_account(mint).await {
        Ok(acc) => acc,
        Err(e) => {
            error!("Failed to get token2022 token metadata 1: {e}");
            return Ok(None);
        }
    };

    let mint_data = match StateWithExtensions::<Mint>::unpack(&account.data) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to unpack token2022 token metadata: {e}");
            return Ok(None);
        }
    };

    let metadata = match mint_data.get_variable_len_extension::<TokenMetadata>() {
        Ok(meta) => meta,
        Err(e) => {
            error!("Failed to get token2022 token metadata 2: {e}");
            return Ok(None);
        }
    };

    Ok(Some(ResolvedTokenMetadata {
        name: metadata.name,
        symbol: metadata.symbol,
        uri: metadata.uri,
    }))
}

// TODO rename function — spl is not representative enough. it works for Raydium as well
pub async fn get_spl_token_metadata(
    rpc_client: &RpcClient,
    mint: &Pubkey,
) -> Result<Option<ResolvedTokenMetadata>, PriceError> {
    let metaplex_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s") // TODO remove hardcoded value
        .map_err(|_| PriceError::InvalidPubkey)?;

    let (metadata_pda, _) = Pubkey::find_program_address(
        &[b"metadata", metaplex_program_id.as_ref(), mint.as_ref()],
        &metaplex_program_id,
    );

    let account = match rpc_client.get_account(&metadata_pda).await {
        Ok(acc) => acc,
        Err(_) => return Ok(None),
    };

    let metadata = match Metadata::safe_deserialize(&account.data) {
        Ok(meta) => meta,
        Err(_) => return Ok(None),
    };

    Ok(Some(ResolvedTokenMetadata {
        name: metadata.name.trim_end_matches('\0').to_string(),
        symbol: metadata.symbol.trim_end_matches('\0').to_string(),
        uri: metadata.uri.trim_end_matches('\0').to_string(),
    }))
}

pub async fn fetch_extended_metadata(
    uri: &str,
) -> Result<Option<ExtendedMetadata>, Box<dyn std::error::Error>> {
    let response = reqwest::get(uri).await?;
    if !response.status().is_success() {
        return Ok(None);
    }
    let metadata = response.json::<ExtendedMetadata>().await?;
    Ok(Some(metadata))
}

pub async fn download_and_save_metadata(
    uri: &str,
    save_dir: &Path,
    token_mint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(save_dir)?;

    // Download metadata JSON
    let metadata_response = reqwest::get(uri).await?;
    if metadata_response.status().is_success() {
        let metadata: ExtendedMetadata = metadata_response.json().await?;
        let json_path = save_dir.join(format!("{token_mint}_metadata.json"));
        std::fs::write(json_path, serde_json::to_string_pretty(&metadata)?)?;

        // Download image if available
        if !metadata.image.is_empty() {
            let image_response = reqwest::get(&metadata.image).await?;
            if image_response.status().is_success() {
                let ext = metadata.image.split('.').next_back().unwrap_or("webp"); // TODO improve formating handling
                let image_path = save_dir.join(format!("{token_mint}_image.{ext}"));
                let bytes = image_response.bytes().await?;
                std::fs::write(image_path, bytes)?;
            }
        }
    }

    Ok(())
}

// Raydium

pub fn parse_pool_state(data: &[u8]) -> Result<PoolState, std::io::Error> {
    PoolState::try_from_slice(&data[8..])
}

pub fn get_price_from_pool_state(pool_state: &PoolState) -> i64 {
    let scale = 10_i64.pow(9); // Price scaled by 10^9
    let decimal_adjustment =
        10_i64.pow(pool_state.base_decimals as u32) / 10_i64.pow(pool_state.quote_decimals as u32);

    ((pool_state.virtual_quote as i128 * scale as i128 * decimal_adjustment as i128)
        / pool_state.virtual_base as i128) as i64
}
