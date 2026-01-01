CREATE TABLE pf_bonding_curve_progress
(
    mint                   TEXT             NOT NULL UNIQUE,
    bonding_curve          TEXT             NOT NULL UNIQUE,
    last_signature         TEXT,

    decimals               SMALLINT         NOT NULL,

    virtual_sol_reserves   BIGINT           NOT NULL,
    virtual_token_reserves BIGINT           NOT NULL,
    real_sol_reserves      BIGINT           NOT NULL,
    real_token_reserves    BIGINT           NOT NULL,

    progress_bps           INTEGER          NOT NULL CHECK (progress_bps BETWEEN 0 AND 10000),
    progress_pct           DOUBLE PRECISION NOT NULL CHECK (progress_pct BETWEEN 0 AND 100),
    price_lamports         BIGINT           NOT NULL,
    market_cap_lamports    BIGINT           NOT NULL,

    is_pre_migration       BOOLEAN          NOT NULL,
    is_migrated            BOOLEAN          NOT NULL,
    is_tradeable           BOOLEAN          NOT NULL,

    created_at             TIMESTAMPTZ      NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ      NOT NULL DEFAULT now(),

    PRIMARY KEY (bonding_curve)
);