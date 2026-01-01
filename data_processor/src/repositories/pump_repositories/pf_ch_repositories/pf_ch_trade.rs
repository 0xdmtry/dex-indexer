use crate::models::pump_models::pf_models::pf_ch_dto::PfChTradeDto;
use crate::state::AppState;
use log::info;

pub async fn insert_pf_ch_trade(state: &AppState, trade_dto: &PfChTradeDto) -> anyhow::Result<()> {
    let ts_millis = trade_dto.timestamp.timestamp_millis();

    state
        .clickhouse
        .query(
            "INSERT INTO pf_ch_trades (
                signature, slot, blockhash,
                signer, fee_payer, user, creator, fee_recipient,
                mint, bonding_curve, is_pump_pool,
                ix_name, is_buy,
                sol_amount, token_amount, trade_size_lamports,
                transaction_fee, fee_lamports, fee_basis_points,
                creator_fee_lamports, creator_fee_basis_points,
                decimals,
                virtual_sol_reserves, virtual_token_reserves,
                real_sol_reserves, real_token_reserves,
                market_cap_lamports,
                track_volume,
                total_unclaimed_tokens, total_claimed_tokens,
                current_sol_volume, last_update_timestamp,
                timestamp
            )
            VALUES (
                ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?,
                ?,
                ?, ?,
                ?, ?,
                ?,
                ?,
                ?, ?,
                ?, ?,
                ?
            )",
        )
        .bind(&trade_dto.signature)
        .bind(trade_dto.slot)
        .bind(&trade_dto.blockhash)
        .bind(&trade_dto.signer)
        .bind(&trade_dto.fee_payer)
        .bind(&trade_dto.user)
        .bind(&trade_dto.creator)
        .bind(&trade_dto.fee_recipient)
        .bind(&trade_dto.mint)
        .bind(&trade_dto.bonding_curve)
        .bind(trade_dto.is_pump_pool as u8)
        .bind(&trade_dto.ix_name)
        .bind(trade_dto.is_buy as u8)
        .bind(trade_dto.sol_amount)
        .bind(trade_dto.token_amount)
        .bind(trade_dto.trade_size_lamports)
        .bind(trade_dto.transaction_fee)
        .bind(trade_dto.fee_lamports)
        .bind(trade_dto.fee_basis_points)
        .bind(trade_dto.creator_fee_lamports)
        .bind(trade_dto.creator_fee_basis_points)
        .bind(trade_dto.decimals)
        .bind(trade_dto.virtual_sol_reserves)
        .bind(trade_dto.virtual_token_reserves)
        .bind(trade_dto.real_sol_reserves)
        .bind(trade_dto.real_token_reserves)
        .bind(trade_dto.market_cap_lamports)
        .bind(trade_dto.track_volume as u8)
        .bind(trade_dto.total_unclaimed_tokens)
        .bind(trade_dto.total_claimed_tokens)
        .bind(trade_dto.current_sol_volume)
        .bind(trade_dto.last_update_timestamp)
        .bind(ts_millis)
        .execute()
        .await?;

    info!("Inserted pf_ch_trade: {}", trade_dto.signature);

    Ok(())
}
