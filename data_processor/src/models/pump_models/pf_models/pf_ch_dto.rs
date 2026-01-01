use crate::models::pump_models::pf_models::pf_kafka_event::KPfChTrade;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfChTradeDto {
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
    pub timestamp: DateTime<Utc>,
}

impl From<KPfChTrade> for PfChTradeDto {
    fn from(src: KPfChTrade) -> Self {
        Self {
            /* ========= Transaction identity ========= */
            signature: src.signature,
            slot: src.slot,
            blockhash: src.blockhash,

            /* ========= Actors ========= */
            signer: src.signer,
            fee_payer: src.fee_payer,
            user: src.user,
            creator: src.creator,
            fee_recipient: src.fee_recipient,

            /* ========= Token / market ========= */
            mint: src.mint,
            bonding_curve: src.bonding_curve,
            is_pump_pool: src.is_pump_pool,

            /* ========= Instruction semantics ========= */
            ix_name: src.ix_name,
            is_buy: src.is_buy,

            /* ========= Trade amounts ========= */
            sol_amount: src.sol_amount,
            token_amount: src.token_amount,
            trade_size_lamports: src.trade_size_lamports,

            /* ========= Fees ========= */
            transaction_fee: src.transaction_fee,
            fee_lamports: src.fee_lamports,
            fee_basis_points: src.fee_basis_points,
            creator_fee_lamports: src.creator_fee_lamports,
            creator_fee_basis_points: src.creator_fee_basis_points,

            /* ========= Market / bonding curve state ========= */
            decimals: src.decimals,
            virtual_sol_reserves: src.virtual_sol_reserves,
            virtual_token_reserves: src.virtual_token_reserves,
            real_sol_reserves: src.real_sol_reserves,
            real_token_reserves: src.real_token_reserves,
            market_cap_lamports: src.market_cap_lamports,

            /* ========= Volume & tracking ========= */
            track_volume: src.track_volume,
            total_unclaimed_tokens: src.total_unclaimed_tokens,
            total_claimed_tokens: src.total_claimed_tokens,
            current_sol_volume: src.current_sol_volume,
            last_update_timestamp: src.last_update_timestamp,

            /* ========= Timestamp ========= */
            timestamp: src.ts,
        }
    }
}
