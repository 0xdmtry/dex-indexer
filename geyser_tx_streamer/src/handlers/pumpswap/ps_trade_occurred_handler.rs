use crate::models::kafka_event::TradeDirection;
use crate::models::pump_models::ps_kafka_event::KPsTrade;
use chrono::{DateTime, Utc};
use solana_sdk::bs58;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_ps_trade(tx_info: &SubscribeUpdateTransactionInfo) -> Result<KPsTrade, String> {
    let signature = extract_ps_signature(tx_info)?;
    let mint = extract_ps_mint_from_trade(tx_info)?;
    let pool = extract_ps_pool_from_trade(tx_info)?;
    let direction = extract_ps_direction(tx_info)?;
    let token_amount = extract_ps_token_amount(tx_info, 1)?;
    let sol_amount = extract_ps_sol_amount(tx_info, direction)?;
    let user_pubkey = extract_ps_user_pubkey(tx_info)?;
    let ts = extract_ps_trade_timestamp()?;

    Ok(KPsTrade {
        signature,
        mint,
        pool,
        direction,
        sol_amount,
        token_amount,
        user_pubkey,
        ts,
    })
}

pub fn extract_ps_signature(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    if tx_info.signature.len() != 64 {
        return Err("extract_ps_signature: Invalid PS signature length".into());
    }
    Ok(bs58::encode(&tx_info.signature).into_string())
}

pub fn extract_ps_mint_from_trade(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<String, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_ps_mint_from_trade: No PS meta")?;
    let token_balance = meta
        .pre_token_balances
        .first()
        .ok_or("extract_ps_mint_from_trade: No PS token balances")?;
    Ok(token_balance.mint.clone())
}

pub fn extract_ps_pool_from_trade(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<String, String> {
    let tx = tx_info
        .transaction
        .as_ref()
        .ok_or("extract_ps_pool_from_trade: No PS transaction")?;
    let msg = tx
        .message
        .as_ref()
        .ok_or("extract_ps_pool_from_trade: No PS message")?;

    // Pool is account_keys[3] for PumpSwap
    let pool_bytes = msg
        .account_keys
        .get(3)
        .ok_or("extract_ps_pool_from_trade: No PS pool account")?;

    if pool_bytes.len() != 32 {
        return Err("extract_ps_pool_from_trade: Invalid PS pool pubkey length".into());
    }

    Ok(bs58::encode(pool_bytes).into_string())
}

pub fn extract_ps_direction(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<TradeDirection, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;

    for log in &meta.log_messages {
        if log.contains("Instruction: Sell") {
            return Ok(TradeDirection::Sell);
        } else if log.contains("Instruction: Buy") {
            return Ok(TradeDirection::Buy);
        }
    }

    Err("extract_ps_direction: Could not determine PS trade direction".into())
}

pub fn extract_ps_token_amount(
    tx_info: &SubscribeUpdateTransactionInfo,
    trade_index: usize,
) -> Result<i64, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_ps_token_amount: No PS meta")?;
    let inner_ix = meta
        .inner_instructions
        .get(trade_index)
        .ok_or("extract_ps_token_amount: No PS inner instruction at index")?;

    for ix in &inner_ix.instructions {
        if !ix.data.is_empty() && ix.data[0] == 12 {
            if ix.data.len() < 9 {
                return Err("extract_ps_token_amount: TransferChecked PS data too short".into());
            }
            let amount = u64::from_le_bytes(
                ix.data[1..9]
                    .try_into()
                    .map_err(|_| "extract_ps_token_amount: Failed to read PS token amount")?,
            );
            return Ok(amount as i64);
        }
    }

    Err("extract_ps_token_amount: TransferChecked PS instruction not found".into())
}

pub fn extract_ps_sol_amount(
    tx_info: &SubscribeUpdateTransactionInfo,
    direction: TradeDirection,
) -> Result<i64, String> {
    let meta = tx_info
        .meta
        .as_ref()
        .ok_or("extract_ps_sol_amount: No PS meta")?;

    let pre_balance = *meta
        .pre_balances
        .first()
        .ok_or("extract_ps_sol_amount: No PS pre_balance")?;
    let post_balance = *meta
        .post_balances
        .first()
        .ok_or("extract_ps_sol_amount: No PS post_balance")?;
    let fee = meta.fee;

    let sol_change = match direction {
        TradeDirection::Buy => (pre_balance as i64) - (post_balance as i64) - (fee as i64),
        TradeDirection::Sell => (post_balance as i64) - (pre_balance as i64) + (fee as i64),
    };

    Ok(sol_change.abs())
}

pub fn extract_ps_user_pubkey(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info
        .transaction
        .as_ref()
        .ok_or("extract_ps_user_pubkey: No PS transaction")?;
    let msg = tx
        .message
        .as_ref()
        .ok_or("extract_ps_user_pubkey: No PS message")?;
    let user_bytes = msg
        .account_keys
        .first()
        .ok_or("extract_ps_user_pubkey: No PS user account")?;

    if user_bytes.len() != 32 {
        return Err("extract_ps_user_pubkey: Invalid PS user pubkey length".into());
    }

    Ok(bs58::encode(user_bytes).into_string())
}

pub fn extract_ps_trade_timestamp() -> Result<DateTime<Utc>, String> {
    Ok(Utc::now())
}
