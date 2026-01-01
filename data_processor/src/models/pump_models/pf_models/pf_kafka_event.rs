use crate::models::pump_models::pf_models::pf_enums::{PfPriceSource, PfTradeDirection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfChTrade {
    /* ========= Transaction identity ========= */
    pub signature: String, // Transaction signature (base58)
    pub slot: u64,         // Solana slot
    pub blockhash: String,

    /* ========= Actors ========= */
    pub signer: String,        // Primary transaction signer
    pub fee_payer: String,     // Fee payer (account_keys[0])
    pub user: String,          // Trader (from trade event)
    pub creator: String,       // Token / bonding curve creator
    pub fee_recipient: String, // Protocol fee recipient

    /* ========= Token / market ========= */
    pub mint: String,          // Token mint
    pub bonding_curve: String, // Bonding curve account
    pub is_pump_pool: bool,

    /* ========= Instruction semantics ========= */
    pub ix_name: String, // buy | sell | buy_exact_sol_in
    pub is_buy: bool,    // Direction flag

    /* ========= Trade amounts ========= */
    pub sol_amount: u64,          // SOL exchanged (lamports)
    pub token_amount: u64,        // Tokens exchanged (raw units)
    pub trade_size_lamports: u64, // Trade size used for fee calculation

    /* ========= Fees ========= */
    pub transaction_fee: u64,  // SOL fee paid for the transaction (lamports)
    pub fee_lamports: u64,     // Protocol fee paid
    pub fee_basis_points: u64, // Protocol fee bps
    pub creator_fee_lamports: u64, // Creator fee paid
    pub creator_fee_basis_points: u64, // Creator fee bps

    /* ========= Market / bonding curve state (post-trade) ========= */
    pub decimals: i16,
    pub virtual_sol_reserves: u64,   // Virtual SOL reserves
    pub virtual_token_reserves: u64, // Virtual token reserves
    pub real_sol_reserves: u64,      // Real SOL reserves
    pub real_token_reserves: u64,    // Real token reserves
    pub market_cap_lamports: u64,    // Market cap estimate

    /* ========= Volume & tracking ========= */
    pub track_volume: bool,          // Whether volume is tracked
    pub total_unclaimed_tokens: u64, // Total unclaimed tokens
    pub total_claimed_tokens: u64,   // Total claimed tokens
    pub current_sol_volume: u64,     // Current SOL volume
    pub last_update_timestamp: i64,  // Last update timestamp

    /* ========= Timestamp ========= */
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfToken {
    pub mint: String,          // token mint
    pub bonding_curve: String, // bonding curve PDA
    pub name: String,          // token name
    pub symbol: String,        // token symbol
    pub creator: String,       // wallet that deployed the token
    pub user_address: String,  // wallet that executed the creator
    pub decimals: i16,
    pub ts: DateTime<Utc>,   // block timestamp
    pub uri: Option<String>, // metadata URI
    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
    pub image: String,
    pub virtual_token_reserves: i64, // bonding curve virtual token reserves
    pub virtual_sol_reserves: i64,   // bonding curve virtual SOL reserves
    pub real_token_reserves: i64,    // actual token reserves
    pub token_total_supply: i64,     // total token supply
}

/// Kafka payload for an individual swap event (a buy or sell).
/// Purpose: To log a single trade, which is used to feed all downstream aggregations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfTrade {
    pub signature: String,           // transaction signature
    pub mint: String,                // token mint being traded
    pub bonding_curve: String,       // bonding curve PDA
    pub direction: PfTradeDirection, //`Buy` or `Sell`
    pub sol_amount: i64,             // amount of SOL (in lamports) exchanged
    pub token_amount: i64,           // amount of token (in smallest unit) exchanged
    pub user_pubkey: String,         // wallet that executed the swap
    pub ts: DateTime<Utc>,           // transaction time
}

/// Kafka payload representing a change in a token's lifecycle state.
/// Purpose: To either log the initial creation or (more often) update a token to "Migrated".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfTokenLifecycle {
    pub mint: String,                       // wallet that created token
    pub bonding_curve: String,              // bonding_curve
    pub status: PfTradeDirection,           // CREATED / MIGRATED
    pub ts_created: Option<DateTime<Utc>>,  // time token was created
    pub ts_migrated: Option<DateTime<Utc>>, // time token migrated
    pub sol_amount_migrated: Option<i64>, // SOL transferred during migration (lamports, from Migrate event)
    pub token_amount_migrated: Option<i64>, // tokens transferred during migration (from Migrate event)
}

/// Kafka payload for a single, discrete price event.
/// Purpose: To log a new price point for a token from any monitored source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfPrice {
    pub bonding_curve: String, // token bonding curve
    pub source: PfPriceSource, // source of this specific price event
    pub ts: DateTime<Utc>,     // on-chain timestamp (block time) of the event
    pub virtual_token_reserves: i64,
    pub virtual_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub real_sol_reserves: i64,
}
