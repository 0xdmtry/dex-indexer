use crate::models::enums::Platform;
use crate::models::kafka_event::TradeDirection;
use crate::models::pump_models::pf_kafka_event::KPfTrade;
use chrono::{DateTime, Utc};
use solana_sdk::bs58;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_pf_trade(tx_info: &SubscribeUpdateTransactionInfo) -> Result<KPfTrade, String> {
    let signature = extract_pf_signature(tx_info)?;
    let mint = extract_pf_mint_from_trade(tx_info)?;
    let bonding_curve = extract_pf_bonding_curve_from_trade(tx_info)?;
    let direction = extract_pf_direction(tx_info)?;
    let token_amount = extract_pf_token_amount(tx_info)?;
    let sol_amount = extract_pf_sol_amount(tx_info)?;
    let user_pubkey = extract_pf_user_pubkey(tx_info)?;
    let ts = extract_pf_trade_timestamp(tx_info)?;

    let k_pf_trade = KPfTrade {
        signature,
        mint,
        bonding_curve,
        platform: Platform::PumpFun,
        direction,
        sol_amount,
        token_amount,
        user_pubkey,
        ts,
    };

    Ok(k_pf_trade)
}

pub fn extract_pf_signature(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    if tx_info.signature.len() != 64 {
        return Err("extract_pf_signature: Invalid PF signature length".into());
    }
    Ok(bs58::encode(&tx_info.signature).into_string())
}

pub fn extract_pf_mint_from_trade(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<String, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_pf_mint_from_trade: No PF meta")?;

    let token_balance = meta
        .pre_token_balances
        .first()
        .ok_or("extract_pf_mint_from_trade: No PF token balances")?;

    Ok(token_balance.mint.clone())
}

pub fn extract_pf_bonding_curve_from_trade(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<String, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_pf_bonding_curve_from_trade: No PF meta")?;

    let bonding_token_balance = meta
        .post_token_balances
        .iter()
        .max_by_key(|b| {
            b.ui_token_amount
                .as_ref()
                .and_then(|a| a.amount.parse::<u64>().ok())
                .unwrap_or(0)
        })
        .ok_or("extract_pf_bonding_curve_from_trade: No PF token balances found")?;

    Ok(bonding_token_balance.owner.clone())
}

pub fn extract_pf_direction(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<TradeDirection, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_pf_direction: No PF meta")?;

    for log in &meta.log_messages {
        if log.contains("Instruction: Sell") {
            return Ok(TradeDirection::Sell);
        } else if log.contains("Instruction: Buy") {
            return Ok(TradeDirection::Buy);
        }
    }

    Err("extract_pf_direction: Could not determine PF trade direction".into())
}

pub fn extract_pf_token_amount(tx_info: &SubscribeUpdateTransactionInfo) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No PF meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("extract_pf_token_amount: No inner PF instructions")?;

    for ix in &inner_ix.instructions {
        if !ix.data.is_empty() && ix.data[0] == 12 {
            if ix.data.len() < 9 {
                return Err("extract_pf_token_amount: TransferChecked PF data too short".into());
            }
            let amount = u64::from_le_bytes(
                ix.data[1..9]
                    .try_into()
                    .map_err(|_| "extract_pf_token_amount: Failed to read PF token amount")?,
            );
            return Ok(amount as i64);
        }
    }

    Err("extract_pf_token_amount: TransferChecked PF instruction not found".into())
}

pub fn extract_pf_sol_amount(tx_info: &SubscribeUpdateTransactionInfo) -> Result<i64, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_pf_sol_amount: No PF meta")?;

    let pre_balance = *meta
        .pre_balances
        .first()
        .ok_or("extract_pf_sol_amount: No PF pre_balance")?;
    let post_balance = *meta
        .post_balances
        .first()
        .ok_or("extract_pf_sol_amount: No PF post_balance")?;
    let fee = meta.fee;

    let sol_change = (post_balance as i64) - (pre_balance as i64) + (fee as i64);

    Ok(sol_change.abs())
}

pub fn extract_pf_user_pubkey(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info
        .transaction
        .as_ref()
        .ok_or("extract_pf_user_pubkey: No PF transaction")?;
    let msg = tx
        .message
        .as_ref()
        .ok_or("extract_pf_user_pubkey: No PF message")?;

    let user_bytes = msg
        .account_keys
        .first()
        .ok_or("extract_pf_user_pubkey: No PF user account")?;

    if user_bytes.len() != 32 {
        return Err("Invalid user pubkey length".into());
    }

    Ok(bs58::encode(user_bytes).into_string())
}

pub fn extract_pf_trade_timestamp(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<DateTime<Utc>, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_pf_user_pubkey: No PF meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("extract_pf_user_pubkey: No inner instructions")?;

    for ix in &inner_ix.instructions {
        if ix.data.len() > 100 {
            // Event data starts at offset 16 (discriminators)
            // After pubkeys and other fields, timestamp is at a specific offset
            // For a sell event, parse similarly to token creation
            let _data = ix.data.as_slice();

            // Skip to timestamp location (varies by event type)
            // Simplified: using current time as fallback
            if let Some(ts) = DateTime::from_timestamp(chrono::Utc::now().timestamp(), 0) {
                return Ok(ts);
            }
        }
    }

    Ok(chrono::Utc::now())
}
