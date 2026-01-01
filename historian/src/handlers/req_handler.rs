use crate::handlers::rpc_handler::{
    extract_decimals_from_account, extract_mint_from_account, fetch_extended_metadata,
    get_account_by_address, get_account_by_bonding_curve, get_bonding_curve_price,
    get_mint_decimals, get_pool_price, get_price_from_pool_state, get_token_metadata,
};
use crate::handlers::{cache_reader, db_reader, pda_deriver, req_classifier, rpc_handler};
use crate::models::classifier::StringType;
use crate::models::consts::{
    PUMPFUN_PROGRAM_ID, RAYDIUM_LAUNCHLAB_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID,
};
use crate::models::enums::{Platform, RpcAccountData, RpcAccountType};
use crate::models::resolver::{EnrichedResolvedToken, ResolveError};
use log::{error, warn};
use redis::aio::ConnectionManager;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::timeout;

pub async fn resolve_token(
    input: &str,
    cache: &mut ConnectionManager,
    db_pool: &PgPool,
    rpc_client: Arc<RpcClient>,
) -> Result<Option<EnrichedResolvedToken>, ResolveError> {
    timeout(
        Duration::from_secs(3),
        resolve_token_inner(input, cache, db_pool, rpc_client),
    )
    .await
    .map_err(|_| ResolveError::Timeout)?
}

async fn resolve_token_inner(
    input: &str,
    cache: &mut ConnectionManager,
    db_pool: &PgPool,
    rpc_client: Arc<RpcClient>,
) -> Result<Option<EnrichedResolvedToken>, ResolveError> {
    let _classification = req_classifier::classify_string(input); // TODO prior of using it with the resolve_name_or_symbol, review logic of finding multiple results

    match resolve_address(input, cache, db_pool, rpc_client.clone()).await {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => {
            warn!("Address match empty");
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

async fn resolve_address(
    input: &str,
    cache: &mut ConnectionManager,
    db_pool: &PgPool,
    rpc_client: Arc<RpcClient>,
) -> Result<Option<EnrichedResolvedToken>, ResolveError> {
    let cache_task = cache_reader::fetch_from_cache(cache, input, &StringType::Address);
    let db_task = db_reader::fetch_from_db(db_pool, input, &StringType::Address);

    let (cache_result, db_result) = tokio::join!(cache_task, db_task);

    // TODO review necessity in the _mint
    let _mint = if let Ok(tokens) = cache_result {
        tokens.first().map(|t| t.mint.clone())
    } else if let Ok(tokens) = db_result {
        tokens.first().map(|t| t.token.mint.clone())
    } else {
        None
    };

    let mut rpc_checks: Vec<JoinHandle<Option<EnrichedResolvedToken>>> = vec![];

    // Direct address checks
    rpc_checks.push(tokio::spawn({
        let rpc = rpc_client.clone();
        let addr = input.to_string();
        async move {
            if let Some(enriched_token) = resolve_by_bonding_curve(&rpc, &addr).await {
                return Some(enriched_token);
            }

            if let Some(enriched_token) = resolve_by_pool(&rpc.clone(), &addr).await {
                return Some(enriched_token);
            }

            None
        }
    }));

    if let Ok(bc_pda) = pda_deriver::derive_bonding_curve_pda(input) {
        rpc_checks.push(tokio::spawn({
            let rpc = rpc_client.clone();
            let _m = input; // TODO review necessity of _m
            let _bc = bc_pda.to_string(); // TODO review necessity of _bc

            async move {
                if let Some(enriched_token) =
                    resolve_by_bonding_curve(&rpc, &bc_pda.to_string()).await
                {
                    return Some(enriched_token);
                }

                None
            }
        }));
    }

    // As if pool
    if let Ok(pool_pda) = pda_deriver::derive_pool_pda(input) {
        rpc_checks.push(tokio::spawn({
            let rpc = rpc_client.clone();
            let _m = input; // TODO review necessity of _m
            let _p = pool_pda.to_string(); // TODO review necessity of _p

            async move {
                if let Some(enriched_token) = resolve_by_pool(&rpc, &pool_pda.to_string()).await {
                    return Some(enriched_token);
                }

                None
            }
        }));
    }

    for task in rpc_checks {
        if let Ok(Some(enriched_token)) = task.await {
            return Ok(Some(enriched_token));
        }
    }

    Ok(None)
}

// TODO prior of re-introducing review logic of handling multiple findings
async fn _resolve_name_or_symbol(
    input: &str,
    cache: &mut ConnectionManager,
    db_pool: &PgPool,
    rpc_client: Arc<RpcClient>,
) -> Result<Option<EnrichedResolvedToken>, ResolveError> {
    for string_type in [StringType::Symbol, StringType::Name] {
        let cache_task = cache_reader::fetch_from_cache(cache, input, &string_type);
        let db_task = db_reader::fetch_from_db(db_pool, input, &string_type);

        let (cache_result, db_result) = tokio::join!(cache_task, db_task);

        let tokens = if let Ok(tokens) = cache_result {
            tokens
                .into_iter()
                .map(|t| (t.mint, t.bonding_curve, t.pool))
                .collect::<Vec<_>>()
        } else if let Ok(tokens) = db_result {
            tokens
                .into_iter()
                .map(|t| {
                    let bc = t.token.bonding_curve;
                    let pool = t.pools.first().map(|p| p.pool.clone());
                    (t.token.mint, bc, pool)
                })
                .collect()
        } else {
            continue;
        };

        let mut rpc_checks: Vec<JoinHandle<Option<EnrichedResolvedToken>>> = vec![];

        // TODO review necessity in mint
        for (_mint, bonding_curve, pool) in tokens {
            if let Some(bc) = bonding_curve {
                rpc_checks.push(tokio::spawn({
                    let rpc = rpc_client.clone();
                    async move { resolve_by_bonding_curve(&rpc.clone(), &bc).await }
                }));
            }

            if let Some(p) = pool {
                rpc_checks.push(tokio::spawn({
                    let rpc = rpc_client.clone();
                    async move { resolve_by_pool(&rpc.clone(), &p).await }
                }));
            }
        }

        for task in rpc_checks {
            if let Ok(Some(enriched_token)) = task.await {
                return Ok(Some(enriched_token));
            }
        }
    }

    Ok(None)
}

async fn resolve_by_bonding_curve(rpc: &RpcClient, input: &str) -> Option<EnrichedResolvedToken> {
    match get_account_by_address(
        rpc,
        input,
        vec![
            PUMPFUN_PROGRAM_ID,
            RAYDIUM_LAUNCHLAB_PROGRAM_ID,
            TOKEN_2022_PROGRAM_ID,
            SPL_TOKEN_PROGRAM_ID,
        ],
    )
    .await
    {
        Some(rpc_account) => {
            match rpc_account.account_type {
                RpcAccountType::PumpFunToken2022 | RpcAccountType::PumpFunSpl => {
                    // TODO optimize get_account_by_bonding_curve: if we receive bonding curve from get_bonding_curve — maybe there is no reason to pass all programs IDs to get_account_by_bonding_curve
                    match get_account_by_bonding_curve(
                        rpc,
                        input,
                        vec![
                            PUMPFUN_PROGRAM_ID,
                            RAYDIUM_LAUNCHLAB_PROGRAM_ID,
                            SPL_TOKEN_PROGRAM_ID,
                            TOKEN_2022_PROGRAM_ID,
                        ],
                    )
                    .await
                    {
                        Ok(account) => {
                            let bonding_curve = match rpc_account.account_data {
                                RpcAccountData::PumpFunToken2022(bc)
                                | RpcAccountData::PumpFunSpl(bc) => bc,
                                _ => return None,
                            };
                            let decimal = extract_decimals_from_account(&account.clone()).unwrap();
                            let price = get_bonding_curve_price(bonding_curve, decimal).unwrap();
                            let mint = extract_mint_from_account(&account.clone()).unwrap();

                            let toke_metadata =
                                // get_token_metadata(rpc, &mint).await.unwrap().unwrap();
                            get_token_metadata(rpc, &mint)
                                .await
                                .ok()
                                .flatten()
                                .unwrap_or_default();

                            let extended_metadata =
                                /*fetch_extended_metadata(&toke_metadata.uri)
                                .await
                                .unwrap()
                                .unwrap();*/
                            fetch_extended_metadata(&toke_metadata.uri)
                                .await
                                .ok()
                                .flatten()
                                .unwrap_or_default();

                            let enriched_token = EnrichedResolvedToken {
                                mint: mint.to_string(),
                                platform: Platform::PumpFun,

                                bonding_curve: Some(input.to_string()),
                                pool: None,
                                pool_state: None,
                                price,

                                decimal,

                                // ResolvedTokenMetadata
                                name: toke_metadata.name,
                                symbol: toke_metadata.symbol,
                                uri: toke_metadata.uri,

                                // ExtendedMetadata
                                description: extended_metadata.description,

                                twitter: extended_metadata.twitter,
                                telegram: extended_metadata.telegram,
                                website: extended_metadata.website,
                                image: extended_metadata.image,
                            };

                            Some(enriched_token)
                        }
                        Err(e) => {
                            error!("Failed to fetch bonding curve price: {e}");
                            None
                        }
                    }
                }
                RpcAccountType::RaydiumLaunchLab => {
                    let pool_state = match rpc_account.account_data {
                        RpcAccountData::RaydiumLaunchLab(ps) => ps,
                        _ => return None,
                    };

                    // TODO that code is repeated in other patrts — so, there is an opportunity to remove duplicates
                    let toke_metadata = /*get_token_metadata(rpc, &pool_state.base_mint)
                        .await
                        .unwrap()
                        .unwrap(); // TODO too many unwrap — potential runtime panic
                        */
                        get_token_metadata(rpc, &pool_state.base_mint)
                            .await
                            .ok()
                            .flatten()
                            .unwrap_or_default();

                    let extended_metadata = /*fetch_extended_metadata(&toke_metadata.uri)
                        .await
                        .unwrap()
                        .unwrap();*/
                        fetch_extended_metadata(&toke_metadata.uri)
                            .await
                            .ok()
                            .flatten()
                            .unwrap_or_default();

                    let price = get_price_from_pool_state(&pool_state);

                    let launchlab_program_id =
                        Pubkey::from_str(RAYDIUM_LAUNCHLAB_PROGRAM_ID).unwrap();
                    let (pool_state_address, _bump) = Pubkey::find_program_address(
                        &[
                            b"pool",
                            pool_state.base_mint.as_ref(),
                            pool_state.quote_mint.as_ref(),
                        ],
                        &launchlab_program_id,
                    );

                    let enriched_token = EnrichedResolvedToken {
                        mint: pool_state.base_mint.to_string(),
                        platform: Platform::RaydiumLaunchLab,

                        bonding_curve: None,
                        pool: None,
                        pool_state: Some(pool_state_address.to_string()),

                        price,

                        decimal: pool_state.base_decimals,

                        // ResolvedTokenMetadata
                        name: toke_metadata.name,
                        symbol: toke_metadata.symbol,
                        uri: toke_metadata.uri,

                        // ExtendedMetadata
                        description: extended_metadata.description,

                        twitter: extended_metadata.twitter,
                        telegram: extended_metadata.telegram,
                        website: extended_metadata.website,
                        image: extended_metadata.image,
                    };

                    Some(enriched_token)
                }
                _ => None,
            }
        }
        None => None,
    }
}

async fn resolve_by_pool(rpc: &RpcClient, input: &str) -> Option<EnrichedResolvedToken> {
    match rpc_handler::get_pool(rpc, input).await {
        Some(pool) => {
            let decimal = get_mint_decimals(rpc, &pool.base_mint).await.unwrap(); // TODO resolve the unwrap(), i.e. potential runtime issue

            let price = get_pool_price(rpc, pool.clone(), decimal)
                .await
                .unwrap()
                .unwrap(); // TODO review unwrap()

            let toke_metadata = /* get_token_metadata(rpc, &pool.base_mint)
                .await
                .unwrap()
                .unwrap(); // TODO review unwrap()
            */
                get_token_metadata(rpc, &pool.base_mint)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_default();

            let extended_metadata = /* fetch_extended_metadata(&toke_metadata.uri)
                .await
                .unwrap()
                .unwrap();*/
                fetch_extended_metadata(&toke_metadata.uri)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_default();

            let enriched_token = EnrichedResolvedToken {
                mint: pool.base_mint.to_string(),
                platform: Platform::PumpSwap,

                bonding_curve: None,
                pool: Some(input.to_string()),
                pool_state: None,
                price,

                decimal: decimal?, // TODO review unwrap()

                // ResolvedTokenMetadata
                name: toke_metadata.name,
                symbol: toke_metadata.symbol,
                uri: toke_metadata.uri,

                // ExtendedMetadata
                description: extended_metadata.description,

                twitter: extended_metadata.twitter,
                telegram: extended_metadata.telegram,
                website: extended_metadata.website,
                image: extended_metadata.image,
            };

            Some(enriched_token)
        }
        None => None,
    }
}
