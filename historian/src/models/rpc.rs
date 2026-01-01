use crate::models::enums::{RpcAccountData, RpcAccountType};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PriceError {
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] solana_sdk::pubkey::ParsePubkeyError),
    #[error("RPC error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
    #[error("Deserialization error: {0}")]
    Deserialization(std::io::Error),
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Invalid pubkey")]
    InvalidPubkey,
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct BondingCurveAccount {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct BondingCurveAccountToken2022 {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub _padding: [u8; 69], // Remaining bytes
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct BondingCurveAccountSpl {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub _padding: [u8; 68], // Remaining bytes
}

#[derive(BorshDeserialize, Debug, Clone)]
pub struct PoolAccount {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub lp_supply: u64,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub _padding: [u8; 57],
}

#[derive(Debug, Error)]
pub enum ExtractMintError {
    #[error("Invalid pubkey: {0}")]
    InvalidPubkey(#[from] solana_sdk::pubkey::ParsePubkeyError),
    #[error("RPC error: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError),
    #[error("No token accounts found")]
    NoTokenAccounts,
    #[error("Invalid token account data")]
    InvalidTokenAccountData,
    #[error("Data not parsed")]
    DataNotParsed,
    #[error("Invalid JSON structure")]
    InvalidJson,
    #[error("Mint field not found")]
    MintNotFound,
    #[error("Decimals field not found")]
    DecimalsNotFound,
}

// Raydium
// Raydium  PoolState

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct PoolState {
    pub epoch: u64,
    pub auth_bump: u8,
    pub status: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub migrate_type: u8,
    pub supply: u64,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base: u64,
    pub real_quote: u64,
    pub total_quote_fund_raising: u64,
    pub quote_protocol_fee: u64,
    pub platform_fee: u64,
    pub migrate_fee: u64,
    // Vesting fields omitted for price calculation
    pub _vesting: [u8; 40],
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub creator: Pubkey,
    pub token_program_flag: u8,
    pub _padding: [u8; 63],
}

///////////////////////////////////

#[derive(Debug, Clone)]
pub struct RpcAccount {
    pub account_type: RpcAccountType,
    pub account_data: RpcAccountData,
}
