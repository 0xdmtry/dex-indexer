use crate::handlers::req_handler::resolve_token;
use crate::models::enums::{KReqData, KReqType};
use crate::models::kafka::{
    KPriceReqBondingCurve, KPriceReqPool, KPriceReqPoolState, KReq, KTokenReqFulfill,
};
use crate::models::resolver::EnrichedResolvedToken;
use crate::state::AppState;
use futures::StreamExt;
use log::{error, warn};
use redis::AsyncCommands;
use redis::Client;
use redis::aio::ConnectionManager;

pub async fn handle_price_req(
    state: AppState,
    redis_url: String,
    req_tx: tokio::sync::mpsc::Sender<KReq>,
) {
    let client = Client::open(redis_url).unwrap();
    let mut pubsub = client.get_async_pubsub().await.unwrap();

    pubsub.subscribe("req_handler").await.unwrap();

    let mut stream = pubsub.on_message();

    while let Some(msg) = stream.next().await {
        if let Ok(payload) = msg.get_payload::<Vec<String>>() {
            let pools: Vec<String> = serde_json::from_str(&payload[0]).unwrap(); // TODO re-do the unwrap()

            // TODO review necessity in the limit
            let _channel_name = msg.get_channel_name();
            let _limit: usize = 1_000_000;

            let mut cache = state.cache.clone();

            let identifier = &pools[0].clone();

            // TODO re-verify logic behind checking if stream already exists
            let _exists: bool = cache
                .sismember("subscriptions:by_bonding_curves:set", identifier)
                .await
                .unwrap(); // TODO re-do the unwrap()
            let _exists: bool = cache
                .sismember("subscriptions:by_pools:set", identifier)
                .await
                .unwrap(); // TODO re-do the unwrap()

            let channel_id: String = pools[1].clone();

            match resolve_token(
                &pools[0].clone(),
                &mut state.cache.clone(),
                &state.pg_pool.clone(),
                state.rpc_client.clone(),
            )
            .await
            {
                Ok(Some(token)) => {
                    let token_str = serde_json::to_string(&token).unwrap(); // TODO re-do the unwrap()
                    publish_init_token_data(&mut cache, channel_id, token_str.clone()).await;
                    cache_token(&mut cache, &token, token_str.to_string())
                        .await
                        .unwrap(); // TODO re-do the unwrap()
                    cache_stream(&mut cache, &token, token_str.to_string())
                        .await
                        .unwrap(); // TODO re-do the unwrap()
                    send_token_for_price_stream(&mut cache, token.clone(), req_tx.clone()).await;
                    send_token_for_fulfill(token.clone(), req_tx.clone()).await;
                }
                Ok(None) => {
                    warn!("Nothing Found!");
                }
                Err(e) => {
                    warn!("Error: {e}");
                }
            }
        } else {
            warn!("Failed to parse payload");
        }
    }
}

pub async fn cache_stream(
    cache: &mut ConnectionManager,
    token: &EnrichedResolvedToken,
    token_str: String,
) -> Result<(), redis::RedisError> {
    if !token.mint.is_empty() {
        let key = format!("subscriptions:by_mint:{}", token.mint);
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    if let Some(bonding_curve) = &token.bonding_curve {
        let key = format!("subscriptions:by_bonding_curve:{bonding_curve}");
        cache
            .set_ex::<_, _, ()>(&key, &token_str.clone(), 72000)
            .await?;
    }

    if let Some(pool) = &token.pool {
        let key = format!("subscriptions:by_pool:{pool}");
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    if let Some(pool_state) = &token.pool_state {
        let key = format!("subscriptions:by_pool_state:{pool_state}");
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    Ok(())
}

pub async fn cache_token(
    cache: &mut ConnectionManager,
    token: &EnrichedResolvedToken,
    token_str: String,
) -> Result<(), redis::RedisError> {
    if !token.mint.is_empty() {
        let key = format!("tokens:by_mint:{}", token.mint);
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    if let Some(bonding_curve) = &token.bonding_curve {
        let key = format!("tokens:by_bonding_curve:{bonding_curve}");
        cache
            .set_ex::<_, _, ()>(&key, &token_str.clone(), 72000)
            .await?;
    }

    if let Some(pool) = &token.pool {
        let key = format!("tokens:by_pool:{pool}");
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    if let Some(pool_state) = &token.pool_state {
        let key = format!("tokens:by_pool_state:{pool_state}");
        cache
            .set_ex::<_, _, ()>(&key, token_str.clone(), 72000)
            .await?;
    }

    Ok(())
}

pub async fn publish_init_token_data(
    cache: &mut ConnectionManager,
    channel_id: String,
    token_str: String,
) {
    match cache.publish::<_, _, ()>(channel_id, token_str).await {
        Ok(_) => {}
        Err(e) => {
            error!("Publishing fail: {e}");
        }
    }
}

pub async fn send_token_for_price_stream(
    cache: &mut ConnectionManager,
    token: EnrichedResolvedToken,
    req_tx: tokio::sync::mpsc::Sender<KReq>,
) {
    let kreq = match (
        token.bonding_curve.as_ref(),
        token.pool.as_ref(),
        token.pool_state.as_ref(),
    ) {
        (Some(bc), None, None) => match get_identifiers(cache, "tokens:by_bonding_curve:*").await {
            Ok(mut bonding_curves) => {
                bonding_curves.push(bc.clone());
                Some(KReq {
                    req_type: KReqType::PriceReqBondingCurve,
                    platform: token.platform,
                    data: KReqData::PriceReqBondingCurve(KPriceReqBondingCurve { bonding_curves }),
                })
            }
            Err(_) => {
                error!("Error getting bonding curves");
                None
            }
        },
        (None, Some(pool), None) => match get_identifiers(cache, "tokens:by_pool:*").await {
            Ok(mut pools) => {
                pools.push(pool.clone());
                Some(KReq {
                    req_type: KReqType::PriceReqPool,
                    platform: token.platform,
                    data: KReqData::PriceReqPool(KPriceReqPool { pools }),
                })
            }
            Err(_) => {
                error!("Error getting pools");
                None
            }
        },
        (None, None, Some(pool_state)) => {
            match get_identifiers(cache, "tokens:by_pool_state:*").await {
                Ok(mut pools_states) => {
                    pools_states.push(pool_state.clone());
                    Some(KReq {
                        req_type: KReqType::PriceReqPoolState,
                        platform: token.platform,
                        data: KReqData::PriceReqPoolState(KPriceReqPoolState { pools_states }),
                    })
                }
                Err(_) => {
                    error!("Error getting pools");
                    None
                }
            }
        }
        _ => None,
    };

    if let Some(req) = kreq {
        if let Err(e) = req_tx.send(req).await {
            error!("Failed to send req: {e}");
        }
    } else {
        error!("Failed to parse payload.");
    }
}

async fn get_identifiers(
    con: &mut ConnectionManager,
    pattern: &str,
) -> redis::RedisResult<Vec<String>> {
    let keys: Vec<String> = con.keys(pattern).await?;

    let prefix = pattern.trim_end_matches('*');

    let identifiers: Vec<String> = keys
        .into_iter()
        .filter_map(|key| key.strip_prefix(prefix).map(String::from))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    Ok(identifiers)
}

pub async fn send_token_for_fulfill(
    token: EnrichedResolvedToken,
    req_tx: tokio::sync::mpsc::Sender<KReq>,
) {
    let kreq = KReq {
        req_type: KReqType::TokenReqFulfill,
        platform: token.platform,
        data: KReqData::TokenReqFulfill(Box::from(KTokenReqFulfill {
            mint: token.mint,
            platform: token.platform,

            bonding_curve: token.bonding_curve.clone(),
            pool: token.bonding_curve.clone(),
            pool_state: token.pool_state.clone(),

            price: token.price,

            decimal: token.decimal,

            // ResolvedTokenMetadata
            name: token.name,
            symbol: token.symbol,
            uri: token.uri,

            // ExtendedMetadata
            description: token.description,
            twitter: token.twitter,
            telegram: token.telegram,
            website: token.website,
            image: token.image,
        })),
    };

    if let Err(e) = req_tx.send(kreq).await {
        error!("Failed to send req: {e}");
    }
}
