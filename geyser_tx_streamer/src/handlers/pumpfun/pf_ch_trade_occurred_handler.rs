use crate::models::pump_models::pf_ch_kafka_event::PfChTradeUnified;
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::Utc;
use solana_sdk::bs58;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use std::convert::TryFrom;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_pf_ch_trade(
    tx_info: &SubscribeUpdateTransactionInfo,
    slot: u64,
) -> Result<PfChTradeUnified, String> {
    /* ========= Transaction identity ========= */
    let signature: String = extract_pf_signature(tx_info)?;
    let slot: u64 = slot;
    let blockhash: String = extract_blockhash(tx_info)?;

    /* ========= Actors ========= */
    let signer = pf_signer(tx_info)?;
    let fee_payer = pf_fee_payer(tx_info)?;
    let user = pf_user(tx_info)?;
    let creator = extract_pumpfun_creator(tx_info)?;
    let fee_recipient = extract_pumpfun_fee_recipient(tx_info)?;

    /* ========= Token / market ========= */
    let mint: String = pf_mint(tx_info)?;
    let bonding_curve: String = pf_bonding_curve(tx_info)?;
    let is_pump_pool: bool = extract_is_pump_pool(tx_info)?;

    /* ========= Instruction semantics ========= */
    let ix_name: String = pf_ix_name(tx_info)?; // buy | sell | buy_exact_sol_in
    let is_buy: bool = pf_is_buy(tx_info)?; // Direction flag

    /* ========= Trade amounts ========= */
    let sol_amount: u64 = pf_sol_amount(tx_info)?; // SOL exchanged (lamports)
    let token_amount: u64 = pf_token_amount(tx_info)?; // Tokens exchanged (raw units)
    let trade_size_lamports: u64 = extract_trade_size_lamports(tx_info)?; // Trade size used for fee calculation

    /* ========= Fees ========= */
    let transaction_fee: u64 = extract_transaction_fee(tx_info)?; // SOL fee paid for the transaction (lamports)
    let fee_lamports: u64 = extract_protocol_fee_lamports(tx_info)?; // Protocol fee paid
    let fee_basis_points: u64 = extract_protocol_fee_bps(tx_info)?; // Protocol fee bps
    let creator_fee_lamports: u64 = extract_creator_fee_lamports(tx_info)?; // Creator fee paid
    let creator_fee_basis_points: u64 = extract_creator_fee_bps(tx_info)?; // Creator fee bps

    /* ========= Market / bonding curve state (post-trade) ========= */
    let (
        decimals,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
    ) = build_cc_pumpfun_trade_1(tx_info)?;
    let market_cap_lamports = extract_market_cap_lamports(tx_info)?;

    /* ========= Volume & tracking ========= */
    let track_volume: bool = extract_track_volume(tx_info)?;
    let total_unclaimed_tokens: u64 = extract_total_unclaimed_tokens(tx_info)?;
    let total_claimed_tokens: u64 = extract_total_claimed_tokens(tx_info)?;
    let current_sol_volume: u64 = extract_current_sol_volume(tx_info)?;
    let last_update_timestamp: i64 = extract_last_update_timestamp(tx_info)?;

    let ts = Utc::now();

    let pf_ch_trade_unified = PfChTradeUnified {
        /* ========= Transaction identity ========= */
        signature, // Transaction signature (base58)
        slot,      // Solana slot
        blockhash,

        /* ========= Actors ========= */
        signer,        // Primary transaction signer
        fee_payer,     // Fee payer (account_keys[0])
        user,          // Trader (from trade event)
        creator,       // Token / bonding curve creator
        fee_recipient, // Protocol fee recipient

        /* ========= Token / market ========= */
        mint,          // Token mint
        bonding_curve, // Bonding curve account
        is_pump_pool,

        /* ========= Instruction semantics ========= */
        ix_name, // buy | sell | buy_exact_sol_in
        is_buy,  // Direction flag

        /* ========= Trade amounts ========= */
        sol_amount,          // SOL exchanged (lamports)
        token_amount,        // Tokens exchanged (raw units)
        trade_size_lamports, // Trade size used for fee calculation

        /* ========= Fees ========= */
        transaction_fee,          // SOL fee paid for the transaction (lamports)
        fee_lamports,             // Protocol fee paid
        fee_basis_points,         // Protocol fee bps
        creator_fee_lamports,     // Creator fee paid
        creator_fee_basis_points, // Creator fee bps

        /* ========= Market / bonding curve state (post-trade) ========= */
        decimals,
        virtual_sol_reserves,   // Virtual SOL reserves
        virtual_token_reserves, // Virtual token reserves
        real_sol_reserves,      // Real SOL reserves
        real_token_reserves,    // Real token reserves

        /* ========= Derived metrics ========= */
        market_cap_lamports, // Market cap estimate

        /* ========= Volume & tracking ========= */
        track_volume,           // Whether volume is tracked
        total_unclaimed_tokens, // Total unclaimed tokens
        total_claimed_tokens,   // Total claimed tokens
        current_sol_volume,     // Current SOL volume
        last_update_timestamp,  // Last update timestamp

        /* ========= Timestamp ========= */
        ts,
    };

    Ok(pf_ch_trade_unified)
}

/* ========= Transaction identity ========= */

pub fn extract_pf_signature(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    if tx_info.signature.len() != 64 {
        return Err("extract_pf_signature: Invalid PF signature length".into());
    }
    Ok(bs58::encode(&tx_info.signature).into_string())
}

pub fn extract_blockhash(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let transaction = tx
        .transaction
        .as_ref()
        .ok_or("extract_blockhash: transaction missing")?;

    let message = transaction
        .message
        .as_ref()
        .ok_or("extract_blockhash: message missing")?;

    let bytes = &message.recent_blockhash;

    if bytes.len() != 32 {
        return Err("extract_blockhash: invalid blockhash length".into());
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);

    Ok(Hash::from(arr).to_string())
}

/* ========= Actors ========= */

pub fn pf_signer(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx
        .transaction
        .as_ref()
        .ok_or("pf_signer: missing transaction")?;

    let msg = tx.message.as_ref().ok_or("pf_signer: missing message")?;

    let header = msg.header.as_ref().ok_or("pf_signer: missing header")?;

    let _n = header.num_required_signatures as usize; // TODO review usage

    let key = msg
        .account_keys
        .first()
        .ok_or("pf_signer: missing signer key")?;

    Ok(bs58::encode(key).into_string())
}

pub fn pf_fee_payer(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx
        .transaction
        .as_ref()
        .ok_or("pf_fee_payer: missing transaction")?;

    let msg = tx.message.as_ref().ok_or("pf_fee_payer: missing message")?;

    let key = msg
        .account_keys
        .first()
        .ok_or("pf_fee_payer: missing account_keys[0]")?;

    Ok(bs58::encode(key).into_string())
}

pub fn pf_user(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let meta = tx.meta.as_ref().ok_or("meta missing")?;
    let tb = meta
        .post_token_balances
        .iter()
        .max_by_key(|b| b.ui_token_amount.as_ref()?.amount.parse::<u64>().ok()) // user ATA
        .ok_or("user token balance not found")?;
    Ok(tb.owner.clone())
}

/// Extract `creator` pubkey from Pump.fun `anchor Self CPI Log` (TradeEvent)
pub fn extract_pumpfun_creator(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    // 1) find and decode Anchor event bytes
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    let mut decoded: Option<Vec<u8>> = None;

    for log in logs {
        if let Some(data) = log.strip_prefix("Program data: ") {
            decoded = Some(
                STANDARD
                    .decode(data)
                    .map_err(|_| "extract_pumpfun_creator: base64 decode failed")?,
            );
            break;
        }
    }

    let buf = decoded.ok_or("extract_pumpfun_creator: Program data not found")?;

    // 2) walk Anchor TradeEvent layout up to `creator`
    const DISC: usize = 8;
    const PUBKEY: usize = 32;
    const U64: usize = 8;
    const BOOL: usize = 1;

    let mut o = DISC;

    o += PUBKEY; // mint
    o += U64; // sol_amount
    o += U64; // token_amount
    o += BOOL; // is_buy
    o += PUBKEY; // user
    o += U64; // timestamp
    o += U64 * 4; // virtual_sol, virtual_token, real_sol, real_token
    o += PUBKEY; // fee_recipient
    o += U64; // fee_basis_points
    o += U64; // fee

    if buf.len() < o + PUBKEY {
        return Err("extract_pumpfun_creator: buffer too short".into());
    }

    // 3) read creator pubkey
    let creator = Pubkey::try_from(&buf[o..o + PUBKEY])
        .map_err(|_| "extract_pumpfun_creator: invalid pubkey bytes")?;

    Ok(creator.to_string())
}

/// Extract `feeRecipient` pubkey (base58 String) from Pump.fun Anchor TradeEvent
pub fn extract_pumpfun_fee_recipient(
    tx: &SubscribeUpdateTransactionInfo,
) -> Result<String, String> {
    // decode Anchor event bytes
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    let mut buf: Option<Vec<u8>> = None;

    for log in logs {
        if let Some(data) = log.strip_prefix("Program data: ") {
            buf = Some(
                STANDARD
                    .decode(data)
                    .map_err(|_| "extract_pumpfun_fee_recipient: base64 decode failed")?,
            );
            break;
        }
    }

    let buf = buf.ok_or("extract_pumpfun_fee_recipient: Program data not found")?;

    // walk Anchor TradeEvent layout up to `feeRecipient`
    const DISC: usize = 8;
    const PUBKEY: usize = 32;
    const U64: usize = 8;
    const BOOL: usize = 1;

    let mut o = DISC;

    o += PUBKEY; // mint
    o += U64; // sol_amount
    o += U64; // token_amount
    o += BOOL; // is_buy
    o += PUBKEY; // user
    o += U64; // timestamp
    o += U64 * 4; // virtual_sol, virtual_token, real_sol, real_token

    // feeRecipient starts here
    if buf.len() < o + PUBKEY {
        return Err("extract_pumpfun_fee_recipient: buffer too short".into());
    }

    let fee_recipient = Pubkey::try_from(&buf[o..o + PUBKEY])
        .map_err(|_| "extract_pumpfun_fee_recipient: invalid pubkey bytes")?;

    Ok(fee_recipient.to_string())
}

/* ========= Token / market ========= */

pub fn pf_mint(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    tx.meta
        .as_ref()
        .and_then(|m| m.pre_token_balances.first())
        .map(|b| b.mint.clone())
        .ok_or("mint not found".into())
}

pub fn pf_bonding_curve(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let meta = tx.meta.as_ref().ok_or("meta missing")?;
    let bc = meta
        .post_token_balances
        .iter()
        .max_by_key(|b| b.ui_token_amount.as_ref()?.amount.parse::<u64>().ok())
        .ok_or("bonding curve not found")?;
    Ok(bc.owner.clone())
}

/// Extract `is_pump_pool` from
/// `Pump Fees Program: get_fees` program log
pub fn extract_is_pump_pool(tx: &SubscribeUpdateTransactionInfo) -> Result<bool, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log.strip_prefix("Program log: Pump Fees Program: get_fees is_pump_pool=")
        {
            return match v {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err("extract_is_pump_pool: invalid boolean".into()),
            };
        }
    }

    Err("extract_is_pump_pool: not found".into())
}

/* ========= Instruction semantics ========= */

pub fn pf_ix_name(tx: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let logs = &tx.meta.as_ref().ok_or("meta missing")?.log_messages;
    for l in logs {
        if l.contains("Instruction: BuyExactSolIn") {
            return Ok("buy_exact_sol_in".into());
        }
        if l.contains("Instruction: Buy") {
            return Ok("buy".into());
        }
        if l.contains("Instruction: Sell") {
            return Ok("sell".into());
        }
    }
    Err("ix name not found".into())
}

pub fn pf_is_buy(tx: &SubscribeUpdateTransactionInfo) -> Result<bool, String> {
    Ok(pf_ix_name(tx)?.starts_with("buy"))
}

/* ========= Trade amounts ========= */

pub fn pf_sol_amount(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let meta = tx.meta.as_ref().ok_or("meta missing")?;
    let pre = meta.pre_balances[0];
    let post = meta.post_balances[0];
    Ok(((pre as i64 - post as i64) - meta.fee as i64).abs() as u64)
}

pub fn pf_token_amount(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let meta = tx.meta.as_ref().ok_or("pf_token_amount: meta missing")?;

    let ii = meta
        .inner_instructions
        .first()
        .ok_or("pf_token_amount: no inner instructions")?;

    for ix in &ii.instructions {
        if ix.data.first() == Some(&12) {
            let bytes: [u8; 8] = ix
                .data
                .get(1..9)
                .ok_or("pf_token_amount: TransferChecked data too short")?
                .try_into()
                .map_err(|_| "pf_token_amount: invalid amount slice")?;

            return Ok(u64::from_le_bytes(bytes));
        }
    }

    Err("pf_token_amount: TransferChecked not found".into())
}

/// Extract `trade_size_lamports` from
/// `Pump Fees Program: get_fees` program log
pub fn extract_trade_size_lamports(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        // Example log format (as observed on-chain):
        // "Program log: Pump Fees Program: get_fees trade_size_lamports=123456789"
        if let Some(rest) = log.strip_prefix("Program log: Pump Fees Program: get_fees ") {
            if let Some(value) = rest
                .strip_prefix("trade_size_lamports=")
                .and_then(|v| v.parse::<u64>().ok())
            {
                return Ok(value);
            }
        }
    }

    Err("extract_trade_size_lamports: trade_size_lamports not found".into())
}

/* ========= Fees ========= */
/* 1. SOL transaction fee (lamports) */
pub fn extract_transaction_fee(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    tx.meta
        .as_ref()
        .and_then(|m| Some(m.fee))
        .ok_or("extract_transaction_fee: fee missing".into())
}

/* 2. protocol fee (lamports) */
pub fn extract_protocol_fee_lamports(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log
            .strip_prefix("Program log: Pump Fees Program: get_fees fee_lamports=")
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Ok(v);
        }
    }

    Err("extract_protocol_fee_lamports: not found".into())
}

/* 3. protocol fee bps */
pub fn extract_protocol_fee_bps(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log
            .strip_prefix("Program log: Pump Fees Program: get_fees fee_basis_points=")
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Ok(v);
        }
    }

    Err("extract_protocol_fee_bps: not found".into())
}

/* 4. creator fee (lamports) */
pub fn extract_creator_fee_lamports(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log
            .strip_prefix("Program log: Pump Fees Program: get_fees creator_fee_lamports=")
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Ok(v);
        }
    }

    Err("extract_creator_fee_lamports: not found".into())
}

/* 5. creator fee bps */
pub fn extract_creator_fee_bps(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log
            .strip_prefix("Program log: Pump Fees Program: get_fees creator_fee_basis_points=")
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Ok(v);
        }
    }

    Err("extract_creator_fee_bps: not found".into())
}

/* ========= Market / bonding curve state (post-trade) ========= */

pub fn build_cc_pumpfun_trade_1(
    tx: &SubscribeUpdateTransactionInfo,
) -> Result<(u32, u64, u64, u64, u64), String> {
    let buf = pf_anchor_event_bytes_1(tx)?;
    let (vsol, vtok, rsol, rtok) = trade_event_offsets_1();
    let decimals = extract_token_decimals(tx)?;
    let virtual_sol_reserves = read_u64_1(&buf, vsol)?;
    let virtual_token_reserves = read_u64_1(&buf, vtok)?;
    let real_sol_reserves = read_u64_1(&buf, rsol)?;
    let real_token_reserves = read_u64_1(&buf, rtok)?;

    Ok((
        decimals,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
    ))
}

fn trade_event_offsets_1() -> (usize, usize, usize, usize) {
    const DISC: usize = 8;
    const PUBKEY: usize = 32;
    const U64: usize = 8;
    const BOOL: usize = 1;

    let mut o = DISC;

    o += PUBKEY; // mint
    o += U64; // sol_amount
    o += U64; // token_amount
    o += BOOL; // is_buy
    o += PUBKEY; // user
    o += U64; // timestamp

    let virtual_sol = o;
    o += U64;
    let virtual_token = o;
    o += U64;
    let real_sol = o;
    o += U64;
    let real_token = o;
    o += U64; // TODO review usage

    (virtual_sol, virtual_token, real_sol, real_token)
}

fn pf_anchor_event_bytes_1(tx: &SubscribeUpdateTransactionInfo) -> Result<Vec<u8>, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(data) = log.strip_prefix("Program data: ") {
            let decoded = STANDARD
                .decode(data)
                .map_err(|_| "pf_anchor_event_bytes: base64 decode failed")?;
            return Ok(decoded);
        }
    }

    Err("pf_anchor_event_bytes: Program data not found".into())
}

fn read_u64_1(buf: &[u8], offset: usize) -> Result<u64, String> {
    let bytes: [u8; 8] = buf
        .get(offset..offset + 8)
        .ok_or("read_u64: out of bounds")?
        .try_into()
        .map_err(|_| "read_u64: invalid slice")?;
    Ok(u64::from_le_bytes(bytes))
}

pub fn extract_token_decimals(tx: &SubscribeUpdateTransactionInfo) -> Result<u32, String> {
    let meta = tx
        .meta
        .as_ref()
        .ok_or("extract_token_decimals: meta missing")?;

    let first_balance = meta
        .pre_token_balances
        .first()
        .ok_or("extract_token_decimals: pre_token_balances empty")?;

    let ui_amount = first_balance
        .ui_token_amount
        .as_ref()
        .ok_or("extract_token_decimals: ui_token_amount missing")?;

    Ok(ui_amount.decimals)
}

pub fn extract_market_cap_lamports(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(v) = log
            .strip_prefix("Program log: Pump Fees Program: get_fees market_cap_lamports=")
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Ok(v);
        }
    }

    Err("extract_market_cap_lamports: not found".into())
}

/* ========= Volume & tracking ========= */

fn pf_anchor_trade_event_bytes(tx: &SubscribeUpdateTransactionInfo) -> Result<Vec<u8>, String> {
    let logs = &tx
        .meta
        .as_ref()
        .ok_or("pf_anchor_event_bytes: meta missing")?
        .log_messages;

    for log in logs {
        if let Some(data) = log.strip_prefix("Program data: ") {
            return STANDARD
                .decode(data)
                .map_err(|_| "pf_anchor_trade_event_bytes: base64 decode failed".into());
        }
    }

    Err("pf_anchor_trade_event_bytes: Program data not found".into())
}

fn volume_tracking_base_offset() -> usize {
    const DISC: usize = 8;
    const PUBKEY: usize = 32;
    const U64: usize = 8;
    const BOOL: usize = 1;

    let mut o = DISC;

    o += PUBKEY; // mint
    o += U64; // sol_amount
    o += U64; // token_amount
    o += BOOL; // is_buy
    o += PUBKEY; // user
    o += U64; // timestamp
    o += U64 * 4; // virtual_sol, virtual_token, real_sol, real_token
    o += PUBKEY; // fee_recipient
    o += U64; // fee_basis_points
    o += U64; // fee
    o += PUBKEY; // creator
    o += U64; // creator_fee_basis_points
    o += U64; // creator_fee

    o
}

pub fn extract_track_volume(tx: &SubscribeUpdateTransactionInfo) -> Result<bool, String> {
    let buf = pf_anchor_trade_event_bytes(tx)?;
    let o = volume_tracking_base_offset();

    if buf.len() <= o {
        return Err("extract_track_volume: buffer too short".into());
    }

    Ok(buf[o] != 0)
}

pub fn extract_total_unclaimed_tokens(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let buf = pf_anchor_trade_event_bytes(tx)?;
    let o = volume_tracking_base_offset() + 1;

    if buf.len() < o + 8 {
        return Err("extract_total_unclaimed_tokens: buffer too short".into());
    }

    Ok(u64::from_le_bytes(buf[o..o + 8].try_into().unwrap()))
}

pub fn extract_total_claimed_tokens(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let buf = pf_anchor_trade_event_bytes(tx)?;
    let o = volume_tracking_base_offset() + 1 + 8;

    if buf.len() < o + 8 {
        return Err("extract_total_claimed_tokens: buffer too short".into());
    }

    Ok(u64::from_le_bytes(buf[o..o + 8].try_into().unwrap()))
}

pub fn extract_current_sol_volume(tx: &SubscribeUpdateTransactionInfo) -> Result<u64, String> {
    let buf = pf_anchor_trade_event_bytes(tx)?;
    let o = volume_tracking_base_offset() + 1 + 8 + 8;

    if buf.len() < o + 8 {
        return Err("extract_current_sol_volume: buffer too short".into());
    }

    Ok(u64::from_le_bytes(buf[o..o + 8].try_into().unwrap()))
}

pub fn extract_last_update_timestamp(tx: &SubscribeUpdateTransactionInfo) -> Result<i64, String> {
    let buf = pf_anchor_trade_event_bytes(tx)?;
    let o = volume_tracking_base_offset() + 1 + 8 + 8 + 8;

    if buf.len() < o + 8 {
        return Err("extract_last_update_timestamp: buffer too short".into());
    }

    Ok(i64::from_le_bytes(buf[o..o + 8].try_into().unwrap()))
}
