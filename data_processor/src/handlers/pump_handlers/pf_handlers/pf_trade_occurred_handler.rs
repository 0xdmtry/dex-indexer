use crate::models::pump_models::pf_models::pf_ch_dto::PfChTradeDto;
use crate::models::pump_models::pf_models::pf_kafka_event::KPfChTrade;
use crate::models::pump_models::pf_models::pf_pgsql_dto::{
    build_pf_pgsql_bonding_curve_progress, build_pf_pgsql_price,
};
use crate::models::pump_models::pf_models::pf_redis::{PfRedisBondingCurveProgress, PfRedisPrice};
use crate::repositories::pump_repositories::pf_ch_repositories::pf_ch_trade::insert_pf_ch_trade;
use crate::repositories::pump_repositories::pf_pgsql_repositories::pf_pgsql_prices::{
    upsert_pf_pgsql_bonding_curve_progress, upsert_pf_pgsql_price,
};
use crate::repositories::pump_repositories::pf_redis_repositories::pf_redis_prices::{
    upsert_pf_redis_bonding_curve_progress, upsert_pf_redis_price,
};
use crate::state::AppState;

pub async fn handle_pf_trade(state: &AppState, k_pf_ch_trade: KPfChTrade) -> anyhow::Result<()> {
    let trade_ch = PfChTradeDto::from(k_pf_ch_trade.clone());
    insert_pf_ch_trade(state, &trade_ch).await?;

    let pf_pgsql_price_dto = build_pf_pgsql_price(k_pf_ch_trade.clone());

    if let Ok(p) = pf_pgsql_price_dto {
        let pf_redis_price = PfRedisPrice::from(p.clone());
        upsert_pf_pgsql_price(&state.pg_pool, p).await?;
        upsert_pf_redis_price(state, pf_redis_price).await?;
    }

    let pf_pgsql_bonding_curve_progress_dto =
        build_pf_pgsql_bonding_curve_progress(k_pf_ch_trade.clone());

    if let Ok(bcp) = pf_pgsql_bonding_curve_progress_dto {
        let pf_redis_bcp = PfRedisBondingCurveProgress::from(bcp.clone());
        upsert_pf_redis_bonding_curve_progress(state, pf_redis_bcp).await?;
        upsert_pf_pgsql_bonding_curve_progress(&state.pg_pool, bcp).await?;
    }

    Ok(())
}
