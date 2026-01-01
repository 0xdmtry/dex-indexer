use crate::models::classifier::StringType;
use crate::models::db::{DbError, Pool, Token, TokenWithPools};
use sqlx::PgPool;

pub async fn fetch_from_db(
    pool: &PgPool,
    input: &str,
    string_type: &StringType,
) -> Result<Vec<TokenWithPools>, DbError> {
    match string_type {
        StringType::Name => {
            let normalized = input.trim().to_lowercase();
            let tokens = sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE LOWER(name) = $1")
                .bind(normalized)
                .fetch_all(pool)
                .await?;

            enrich_with_pools(pool, tokens).await
        }
        StringType::Symbol => {
            let normalized = input.trim().to_lowercase();
            let tokens =
                sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE LOWER(symbol) = $1")
                    .bind(normalized)
                    .fetch_all(pool)
                    .await?;

            enrich_with_pools(pool, tokens).await
        }
        StringType::Address => {
            // Try mint
            if let Ok(tokens) = sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE mint = $1")
                .bind(input)
                .fetch_all(pool)
                .await
            {
                if !tokens.is_empty() {
                    return enrich_with_pools(pool, tokens).await;
                }
            }

            // Try bonding_curve
            if let Ok(tokens) =
                sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE bonding_curve = $1")
                    .bind(input)
                    .fetch_all(pool)
                    .await
            {
                if !tokens.is_empty() {
                    return enrich_with_pools(pool, tokens).await;
                }
            }

            // Try pool - get mint first, then token
            let mints = sqlx::query_scalar::<_, String>("SELECT mint FROM pools WHERE pool = $1")
                .bind(input)
                .fetch_all(pool)
                .await?;

            if mints.is_empty() {
                return Ok(Vec::new());
            }

            let tokens = sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE mint = ANY($1)")
                .bind(&mints)
                .fetch_all(pool)
                .await?;

            enrich_with_pools(pool, tokens).await
        }
    }
}

async fn enrich_with_pools(
    pool: &PgPool,
    tokens: Vec<Token>,
) -> Result<Vec<TokenWithPools>, DbError> {
    let mut results = Vec::new();

    for token in tokens {
        let pools = sqlx::query_as::<_, Pool>("SELECT * FROM pools WHERE mint = $1")
            .bind(&token.mint)
            .fetch_all(pool)
            .await?;

        results.push(TokenWithPools { token, pools });
    }

    Ok(results)
}
