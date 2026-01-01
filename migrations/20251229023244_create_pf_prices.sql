CREATE TABLE pf_prices
(
    id                     BIGSERIAL PRIMARY KEY,

    mint                   TEXT               NOT NULL UNIQUE,
    bonding_curve          TEXT               NOT NULL UNIQUE,
    last_signature         TEXT,

    price                  BIGINT             NOT NULL,
    source                 pf_price_source    NOT NULL,
    direction              pf_trade_direction NOT NULL,

    decimals               SMALLINT           NOT NULL,

    virtual_token_reserves BIGINT             NOT NULL,
    virtual_sol_reserves   BIGINT             NOT NULL,
    real_token_reserves    BIGINT             NOT NULL,
    real_sol_reserves      BIGINT             NOT NULL,

    ts                     TIMESTAMPTZ        NOT NULL,
    created_at             TIMESTAMPTZ        NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ        NOT NULL DEFAULT now()
);

CREATE INDEX idx_pf_prices_ts ON pf_prices (ts);
