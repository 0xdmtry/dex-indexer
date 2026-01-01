CREATE TABLE pf_ch_trades
(
    /* ========= Transaction identity ========= */
    signature                String,
    slot                     UInt64,
    blockhash                String,

    /* ========= Actors ========= */
    signer                   String,
    fee_payer                String,
    user                     String,
    creator                  String,
    fee_recipient            String,

    /* ========= Token / market ========= */
    mint                     String,
    bonding_curve            String,
    is_pump_pool             UInt8,

    /* ========= Instruction semantics ========= */
    ix_name                  String,
    is_buy                   UInt8,

    /* ========= Trade amounts ========= */
    sol_amount               UInt64,
    token_amount             UInt64,
    trade_size_lamports      UInt64,

    /* ========= Fees ========= */
    transaction_fee          UInt64,
    fee_lamports             UInt64,
    fee_basis_points         UInt64,
    creator_fee_lamports     UInt64,
    creator_fee_basis_points UInt64,

    /* ========= Market / bonding curve state ========= */
    decimals                 UInt32,
    virtual_sol_reserves     UInt64,
    virtual_token_reserves   UInt64,
    real_sol_reserves        UInt64,
    real_token_reserves      UInt64,
    market_cap_lamports      UInt64,

    /* ========= Volume & tracking ========= */
    track_volume             UInt8,
    total_unclaimed_tokens   UInt64,
    total_claimed_tokens     UInt64,
    current_sol_volume       UInt64,
    last_update_timestamp    Int64,

    /* ========= Timestamp ========= */
    timestamp                DateTime64(3, 'UTC')
) ENGINE = MergeTree
PARTITION BY toYYYYMM(timestamp)
ORDER BY (mint, bonding_curve, timestamp)
SETTINGS index_granularity = 8192;
