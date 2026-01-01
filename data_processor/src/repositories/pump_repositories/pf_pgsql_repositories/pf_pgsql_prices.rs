use crate::models::pump_models::pf_models::pf_pgsql_dto::{
    PfPgsqlBondingCurveProgressDto, PfPgsqlPriceDto,
};
use chrono::Utc;
use sqlx::PgPool;

/// Insert or update latest price for a (mint, bonding_curve) pair
pub async fn upsert_pf_pgsql_price(pool: &PgPool, dto: PfPgsqlPriceDto) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO pf_prices (
            mint,
            bonding_curve,
            last_signature,
            price,
            source,
            direction,
            decimals,
            virtual_token_reserves,
            virtual_sol_reserves,
            real_token_reserves,
            real_sol_reserves,
            ts,
            created_at,
            updated_at
        )
        VALUES (
            $1,$2,$3,$4,$5,$6,$7,
            $8,$9,$10,$11,
            $12,$13,$14
        )
        ON CONFLICT (mint, bonding_curve)
        DO UPDATE SET
            last_signature = EXCLUDED.last_signature,
            price = EXCLUDED.price,
            source = EXCLUDED.source,
            direction = EXCLUDED.direction,
            decimal = EXCLUDED.decimal,
            virtual_token_reserves = EXCLUDED.virtual_token_reserves,
            virtual_sol_reserves = EXCLUDED.virtual_sol_reserves,
            real_token_reserves = EXCLUDED.real_token_reserves,
            real_sol_reserves = EXCLUDED.real_sol_reserves,
            ts = EXCLUDED.ts,
            updated_at = EXCLUDED.updated_at
        "#,
    )
    .bind(&dto.mint)
    .bind(&dto.bonding_curve)
    .bind(&dto.last_signature)
    .bind(dto.price)
    .bind(dto.source)
    .bind(dto.direction)
    .bind(dto.decimals)
    .bind(dto.virtual_token_reserves)
    .bind(dto.virtual_sol_reserves)
    .bind(dto.real_token_reserves)
    .bind(dto.real_sol_reserves)
    .bind(dto.ts)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

/// Insert or update bonding-curve progress for a bonding_curve (canonical row)
pub async fn upsert_pf_pgsql_bonding_curve_progress(
    pool: &PgPool,
    dto: PfPgsqlBondingCurveProgressDto,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO pf_bonding_curve_progress (
            mint,
            bonding_curve,
            last_signature,

            decimals,

            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,

            progress_bps,
            progress_pct,
            price_lamports,
            market_cap_lamports,

            is_pre_migration,
            is_migrated,
            is_tradeable,

            created_at,
            updated_at
        )
        VALUES (
            $1,$2,$3,
            $4,
            $5,$6,$7,$8,
            $9,$10,$11,$12,
            $13,$14,$15,
            $16,$17
        )
        ON CONFLICT (bonding_curve)
        DO UPDATE SET
            mint                   = EXCLUDED.mint,
            last_signature         = EXCLUDED.last_signature,

            decimals               = EXCLUDED.decimals,

            virtual_sol_reserves   = EXCLUDED.virtual_sol_reserves,
            virtual_token_reserves = EXCLUDED.virtual_token_reserves,
            real_sol_reserves      = EXCLUDED.real_sol_reserves,
            real_token_reserves    = EXCLUDED.real_token_reserves,

            progress_bps           = EXCLUDED.progress_bps,
            progress_pct           = EXCLUDED.progress_pct,
            price_lamports         = EXCLUDED.price_lamports,
            market_cap_lamports    = EXCLUDED.market_cap_lamports,

            is_pre_migration       = EXCLUDED.is_pre_migration,
            is_migrated            = EXCLUDED.is_migrated,
            is_tradeable           = EXCLUDED.is_tradeable,

            updated_at             = EXCLUDED.updated_at
        "#,
    )
    .bind(&dto.mint)
    .bind(&dto.bonding_curve)
    .bind(&dto.last_signature)
    .bind(dto.decimals)
    .bind(dto.virtual_sol_reserves as i64)
    .bind(dto.virtual_token_reserves as i64)
    .bind(dto.real_sol_reserves as i64)
    .bind(dto.real_token_reserves as i64)
    .bind(dto.progress_bps as i32)
    .bind(dto.progress_pct)
    .bind(dto.price_lamports as i64)
    .bind(dto.market_cap_lamports as i64)
    .bind(dto.is_pre_migration)
    .bind(dto.is_migrated)
    .bind(dto.is_tradeable)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}
