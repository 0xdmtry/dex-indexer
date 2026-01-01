-- Price event source type
CREATE TYPE pf_price_source AS ENUM (
    'pf_bonding_curve',
    'pf_trade',
    'unknown'
);

-- Trade direction for swaps
CREATE TYPE pf_trade_direction AS ENUM (
    'buy',
    'sell'
);
