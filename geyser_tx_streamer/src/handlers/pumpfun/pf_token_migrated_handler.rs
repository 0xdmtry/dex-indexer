use crate::models::enums::Platform;
use crate::models::kafka_event::TokenStatus;
use crate::models::pump_models::pf_kafka_event::KPfTokenLifecycle;
use chrono::{DateTime, Utc};
use solana_sdk::bs58;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn extract_mint(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // user pubkey (32 bytes)
    if data.len() < cursor + 32 {
        return Err("Data too short for user pubkey".into());
    }
    cursor += 32;

    // mint pubkey (32 bytes)
    if data.len() < cursor + 32 {
        return Err("Data too short for mint pubkey".into());
    }
    let mint_bytes = &data[cursor..cursor + 32];

    Ok(bs58::encode(mint_bytes).into_string())
}

pub fn extract_token_amount_migrated(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // skip user (32 bytes)
    cursor += 32;

    // skip mint (32 bytes)
    cursor += 32;

    // mintAmount (u64, 8 bytes)
    if data.len() < cursor + 8 {
        return Err("Data too short for mintAmount".into());
    }
    let amount = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading mintAmount")?,
    );

    Ok(amount as i64)
}

pub fn extract_sol_amount_migrated(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // skip user (32 bytes)
    cursor += 32;

    // skip mint (32 bytes)
    cursor += 32;

    // skip mintAmount (8 bytes)
    cursor += 8;

    // solAmount (u64, 8 bytes)
    if data.len() < cursor + 8 {
        return Err("Data too short for solAmount".into());
    }
    let amount = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading solAmount")?,
    );

    Ok(amount as i64)
}

pub fn extract_timestamp(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<DateTime<Utc>, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // skip user (32 bytes)
    cursor += 32;

    // skip mint (32 bytes)
    cursor += 32;

    // skip mintAmount (8 bytes)
    cursor += 8;

    // skip solAmount (8 bytes)
    cursor += 8;

    // skip poolMigrationFee (8 bytes)
    cursor += 8;

    // skip bondingCurve (32 bytes)
    cursor += 32;

    // timestamp (i64, 8 bytes)
    if data.len() < cursor + 8 {
        return Err("Data too short for timestamp".into());
    }
    let ts_value = i64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading timestamp")?,
    );

    let ts = DateTime::from_timestamp(ts_value, 0)
        .ok_or_else(|| "Invalid timestamp value".to_string())?;

    Ok(ts)
}

pub fn extract_pool_address(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // skip user (32 bytes)
    cursor += 32;

    // skip mint (32 bytes)
    cursor += 32;

    // skip mintAmount (8 bytes)
    cursor += 8;

    // skip solAmount (8 bytes)
    cursor += 8;

    // skip poolMigrationFee (8 bytes)
    cursor += 8;

    // skip bondingCurve (32 bytes)
    cursor += 32;

    // skip timestamp (8 bytes)
    cursor += 8;

    // pool address (32 bytes)
    if data.len() < cursor + 32 {
        return Err("Data too short for pool address".into());
    }
    let pool_bytes = &data[cursor..cursor + 32];

    Ok(bs58::encode(pool_bytes).into_string())
}

pub fn extract_bonding_curve(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions")?;

    let event_ix = inner_ix.instructions.last().ok_or("No event instruction")?;

    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // skip user (32 bytes)
    cursor += 32;

    // skip mint (32 bytes)
    cursor += 32;

    // skip mintAmount (8 bytes)
    cursor += 8;

    // skip solAmount (8 bytes)
    cursor += 8;

    // skip poolMigrationFee (8 bytes)
    cursor += 8;

    // bondingCurve (32 bytes)
    if data.len() < cursor + 32 {
        return Err("Data too short for bonding curve".into());
    }
    let bonding_curve_bytes = &data[cursor..cursor + 32];

    Ok(bs58::encode(bonding_curve_bytes).into_string())
}

pub fn handle_pf_token_migration(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<KPfTokenLifecycle, String> {
    let mint = extract_mint(tx_info).map_err(|e| format!("extract_mint: {e}"))?;

    let token_amount_migrated = extract_token_amount_migrated(tx_info)
        .map_err(|e| format!("extract_token_amount_migrated: {e}"))?;

    let sol_amount_migrated = extract_sol_amount_migrated(tx_info)
        .map_err(|e| format!("extract_sol_amount_migrated: {e}"))?;

    let ts_migrated = extract_timestamp(tx_info).map_err(|e| format!("extract_timestamp: {e}"))?;

    let pool_address =
        extract_pool_address(tx_info).map_err(|e| format!("extract_pool_address: {e}"))?;

    let bonding_curve =
        extract_bonding_curve(tx_info).map_err(|e| format!("extract_bonding_curve: {e}"))?;

    Ok(KPfTokenLifecycle {
        mint,
        status: TokenStatus::Migrated,
        platform: Platform::PumpSwap,
        ts_created: None,
        ts_migrated: Some(ts_migrated),
        bonding_curve: Some(bonding_curve),
        pool: Some(pool_address),
        sol_amount_migrated: Some(sol_amount_migrated),
        token_amount_migrated: Some(token_amount_migrated),
    })
}
