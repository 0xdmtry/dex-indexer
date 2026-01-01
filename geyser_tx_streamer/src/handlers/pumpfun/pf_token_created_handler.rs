use crate::models::pump_models::pf_kafka_event::KPfToken;
use chrono::{DateTime, Utc};
use solana_sdk::bs58;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn extract_name(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;
    let ix = msg.instructions.get(2).ok_or("No create instruction")?;
    let data = ix.data.as_slice();

    // Need at least 8 bytes for discriminator
    if data.len() < 8 {
        return Err("Data too short for discriminator".into());
    }

    let mut cursor = 8;

    // Need 4 bytes for name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }

    let name_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read name_len")?;
    let name_len = u32::from_le_bytes(name_len_bytes) as usize;
    cursor += 4;

    // Need name_len bytes for name
    if data.len() < cursor + name_len {
        return Err("Data too short for name field".into());
    }

    let name_bytes = &data[cursor..cursor + name_len];
    let name = String::from_utf8(name_bytes.to_vec()).map_err(|_| "Invalid UTF-8 in name")?;

    Ok(name)
}

pub fn extract_symbol(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;
    let ix = msg.instructions.get(2).ok_or("No create instruction")?;
    let data = ix.data.as_slice();

    if data.len() < 8 {
        return Err("Data too short for discriminator".into());
    }

    let mut cursor = 8;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read name_len")?;
    let name_len = u32::from_le_bytes(name_len_bytes) as usize;
    cursor += 4;

    // skip name
    if data.len() < cursor + name_len {
        return Err("Data too short for name field".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read symbol_len")?;
    let symbol_len = u32::from_le_bytes(symbol_len_bytes) as usize;
    cursor += 4;

    // symbol
    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol field".into());
    }
    let symbol_bytes = &data[cursor..cursor + symbol_len];
    let symbol = String::from_utf8(symbol_bytes.to_vec()).map_err(|_| "Invalid UTF-8 in symbol")?;

    Ok(symbol)
}

pub fn extract_uri(tx_info: &SubscribeUpdateTransactionInfo) -> Result<Option<String>, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;
    let ix = msg.instructions.get(2).ok_or("No create instruction")?;
    let data = ix.data.as_slice();

    if data.len() < 8 {
        return Err("Data too short for discriminator".into());
    }

    let mut cursor = 8;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read name_len")?;
    let name_len = u32::from_le_bytes(name_len_bytes) as usize;
    cursor += 4;

    // skip name
    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read symbol_len")?;
    let symbol_len = u32::from_le_bytes(symbol_len_bytes) as usize;
    cursor += 4;

    // skip symbol
    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len_bytes: [u8; 4] = data[cursor..cursor + 4]
        .try_into()
        .map_err(|_| "Failed to read uri_len")?;
    let uri_len = u32::from_le_bytes(uri_len_bytes) as usize;
    cursor += 4;

    if uri_len == 0 {
        return Ok(None);
    }

    // uri
    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    let uri_bytes = &data[cursor..cursor + uri_len];
    let uri = String::from_utf8(uri_bytes.to_vec()).map_err(|_| "Invalid UTF-8 in uri")?;

    Ok(Some(uri))
}

pub fn extract_creator(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;
    let ix = msg.instructions.get(2).ok_or("No create instruction")?;
    let data = ix.data.as_slice();

    if data.len() < 8 {
        return Err("Data too short for discriminator".into());
    }

    let mut cursor = 8;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read name_len")?,
    ) as usize;
    cursor += 4;

    // skip name
    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read symbol_len")?,
    ) as usize;
    cursor += 4;

    // skip symbol
    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read uri_len")?,
    ) as usize;
    cursor += 4;

    // skip uri
    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // creator (32 bytes)
    if data.len() < cursor + 32 {
        return Err("Data too short for creator pubkey".into());
    }
    let creator_bytes = &data[cursor..cursor + 32];

    Ok(bs58::encode(creator_bytes).into_string())
}

pub fn extract_mint(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;

    let mint_bytes = msg
        .account_keys
        .get(1)
        .ok_or("No mint account at index 1")?;

    if mint_bytes.len() != 32 {
        return Err("Mint pubkey has invalid length".into());
    }

    Ok(bs58::encode(mint_bytes).into_string())
}

pub fn extract_bonding_curve(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;

    let curve_bytes = msg
        .account_keys
        .get(2)
        .ok_or("No bonding curve account at index 2")?;

    if curve_bytes.len() != 32 {
        return Err("Bonding curve pubkey has invalid length".into());
    }

    Ok(bs58::encode(curve_bytes).into_string())
}

pub fn extract_user_address(tx_info: &SubscribeUpdateTransactionInfo) -> Result<String, String> {
    let tx = tx_info.transaction.as_ref().ok_or("No transaction")?;
    let msg = tx.message.as_ref().ok_or("No message")?;

    let user_bytes = msg
        .account_keys
        .first()
        .ok_or("No user account at index 0")?;

    if user_bytes.len() != 32 {
        return Err("User pubkey has invalid length".into());
    }

    Ok(bs58::encode(user_bytes).into_string())
}

pub fn extract_timestamp(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<DateTime<Utc>, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;
    let event_ix = inner_ix
        .instructions
        .get(14)
        .ok_or("No event instruction at index 14")?;
    let data = event_ix.data.as_slice();

    // Need at least 16 bytes to skip discriminators
    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading name_len")?,
    ) as usize;
    cursor += 4;

    // skip name
    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading symbol_len")?,
    ) as usize;
    cursor += 4;

    // skip symbol
    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading uri_len")?,
    ) as usize;
    cursor += 4;

    // skip uri
    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // skip 4 pubkeys (32 bytes each)
    let skip_pubkeys = 32 * 4;
    if data.len() < cursor + skip_pubkeys {
        return Err("Data too short for pubkeys".into());
    }
    cursor += skip_pubkeys;

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

pub fn extract_virtual_token_reserves(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;
    let event_ix = inner_ix
        .instructions
        .get(14)
        .ok_or("No event instruction at index 14")?;
    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read name_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read symbol_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed to read uri_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // skip 4 pubkeys + timestamp = 32*4 + 8
    let skip_fixed = 32 * 4 + 8;
    if data.len() < cursor + skip_fixed {
        return Err("Data too short for pubkeys + timestamp".into());
    }
    cursor += skip_fixed;

    // virtual_token_reserves (u64)
    if data.len() < cursor + 8 {
        return Err("Data too short for virtual_token_reserves".into());
    }
    let reserves = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed to read virtual_token_reserves")?,
    );

    Ok(reserves as i64)
}

// Extract virtual_sol_reserves from event log
pub fn extract_virtual_sol_reserves(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;
    let event_ix = inner_ix
        .instructions
        .get(14)
        .ok_or("No event instruction at index 14")?;
    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading name_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading symbol_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading uri_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // skip mint, bonding_curve, user, creator, timestamp, virtual_token_reserves
    let skip = 32 * 4 + 8 + 8;
    if data.len() < cursor + skip {
        return Err("Data too short for fixed fields".into());
    }
    cursor += skip;

    // virtual_sol_reserves (u64)
    if data.len() < cursor + 8 {
        return Err("Data too short for virtual_sol_reserves".into());
    }
    let reserves = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading virtual_sol_reserves")?,
    );

    Ok(reserves as i64)
}

// Extract real_token_reserves from event log
pub fn extract_real_token_reserves(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;
    let event_ix = inner_ix
        .instructions
        .get(14)
        .ok_or("No event instruction at index 14")?;
    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading name_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading symbol_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading uri_len")?,
    ) as usize;
    cursor += 4;

    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // skip mint, bonding_curve, user, creator, timestamp, virtual_token_reserves, virtual_sol_reserves
    let skip = 32 * 4 + 8 + 8 + 8;
    if data.len() < cursor + skip {
        return Err("Data too short for fixed fields".into());
    }
    cursor += skip;

    // real_token_reserves (u64)
    if data.len() < cursor + 8 {
        return Err("Data too short for real_token_reserves".into());
    }

    let reserves = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading real_token_reserves")?,
    );

    Ok(reserves as i64)
}

// Extract token_total_supply from event log
pub fn extract_token_total_supply(tx_info: &SubscribeUpdateTransactionInfo) -> Result<i64, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;
    let event_ix = inner_ix
        .instructions
        .get(14)
        .ok_or("No event instruction at index 14")?;
    let data = event_ix.data.as_slice();

    if data.len() < 16 {
        return Err("Data too short for event discriminators".into());
    }

    let mut cursor = 16;

    // name_len
    if data.len() < cursor + 4 {
        return Err("Data too short for name_len".into());
    }
    let name_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading name_len")?,
    ) as usize;
    cursor += 4;
    if data.len() < cursor + name_len {
        return Err("Data too short for name".into());
    }
    cursor += name_len;

    // symbol_len
    if data.len() < cursor + 4 {
        return Err("Data too short for symbol_len".into());
    }
    let symbol_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading symbol_len")?,
    ) as usize;
    cursor += 4;
    if data.len() < cursor + symbol_len {
        return Err("Data too short for symbol".into());
    }
    cursor += symbol_len;

    // uri_len
    if data.len() < cursor + 4 {
        return Err("Data too short for uri_len".into());
    }
    let uri_len = u32::from_le_bytes(
        data[cursor..cursor + 4]
            .try_into()
            .map_err(|_| "Failed reading uri_len")?,
    ) as usize;
    cursor += 4;
    if data.len() < cursor + uri_len {
        return Err("Data too short for uri".into());
    }
    cursor += uri_len;

    // skip mint, bonding_curve, user, creator, timestamp, virtual_token_reserves,
    // virtual_sol_reserves, real_token_reserves
    let skip = 32 * 4 + 8 + 8 + 8 + 8;
    if data.len() < cursor + skip {
        return Err("Data too short for fixed fields".into());
    }
    cursor += skip;

    // total_supply (u64)
    if data.len() < cursor + 8 {
        return Err("Data too short for token_total_supply".into());
    }
    let supply = u64::from_le_bytes(
        data[cursor..cursor + 8]
            .try_into()
            .map_err(|_| "Failed reading token_total_supply")?,
    );

    Ok(supply as i64)
}

pub fn extract_decimal(tx_info: &SubscribeUpdateTransactionInfo) -> Result<u8, String> {
    let meta = tx_info.meta.as_ref().ok_or("No meta")?;
    let inner_ix = meta
        .inner_instructions
        .first()
        .ok_or("No inner instructions at index 0")?;

    // Find InitializeMint2 instruction (discriminator = 20)
    let mint_ix = inner_ix
        .instructions
        .iter()
        .find(|ix| ix.data.first() == Some(&20))
        .ok_or("No InitializeMint2 instruction found")?;

    let data = mint_ix.data.as_slice();

    if data.len() < 2 {
        return Err("Data too short for decimals".into());
    }

    // decimals is at byte 1
    let decimals = data[1];

    Ok(decimals)
}

// Main handler
pub fn handle_pf_token_creation(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<KPfToken, String> {
    let name = extract_name(tx_info).map_err(|e| format!("extract_name: {e}"))?;

    let symbol = extract_symbol(tx_info).map_err(|e| format!("extract_symbol: {e}"))?;

    let uri = extract_uri(tx_info).map_err(|e| format!("extract_uri: {e}"))?;

    let creator = extract_creator(tx_info).map_err(|e| format!("extract_creator: {e}"))?;

    let mint = extract_mint(tx_info).map_err(|e| format!("extract_mint: {e}"))?;

    let bonding_curve =
        extract_bonding_curve(tx_info).map_err(|e| format!("extract_bonding_curve: {e}"))?;

    let user_address =
        extract_user_address(tx_info).map_err(|e| format!("extract_user_address: {e}"))?;

    let ts = extract_timestamp(tx_info).map_err(|e| format!("extract_timestamp: {e}"))?;

    let virtual_token_reserves = extract_virtual_token_reserves(tx_info)
        .map_err(|e| format!("extract_virtual_token_reserves: {e}"))?;

    let virtual_sol_reserves = extract_virtual_sol_reserves(tx_info)
        .map_err(|e| format!("extract_virtual_sol_reserves: {e}"))?;

    let real_token_reserves = extract_real_token_reserves(tx_info)
        .map_err(|e| format!("extract_real_token_reserves: {e}"))?;

    let token_total_supply = extract_token_total_supply(tx_info)
        .map_err(|e| format!("extract_token_total_supply: {e}"))?;

    let decimals =
        extract_decimal(tx_info).map_err(|e| format!("extract_token_total_supply: {e}"))?;

    let description = "".to_string();
    let twitter = "".to_string();
    let telegram = "".to_string();
    let website = "".to_string();
    let image = "".to_string();

    Ok(KPfToken {
        mint,
        bonding_curve,
        name,
        symbol,
        creator,
        user_address,
        decimals: (decimals as i16),
        ts,
        uri,
        description,
        twitter,
        telegram,
        website,
        image,
        virtual_token_reserves,
        virtual_sol_reserves,
        real_token_reserves,
        token_total_supply,
    })
}
