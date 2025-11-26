/// API endpoint constants
pub mod endpoints {
    // Server Time
    pub const TIME: &str = "/time";

    // API Key endpoints
    pub const CREATE_API_KEY: &str = "/auth/api-key";
    pub const GET_API_KEYS: &str = "/auth/api-keys";
    pub const DELETE_API_KEY: &str = "/auth/api-key";
    pub const DERIVE_API_KEY: &str = "/auth/derive-api-key";
    pub const CLOSED_ONLY: &str = "/auth/ban-status/closed-only";

    // Builder API Key endpoints
    pub const CREATE_BUILDER_API_KEY: &str = "/auth/builder-api-key";
    pub const GET_BUILDER_API_KEYS: &str = "/auth/builder-api-key";
    pub const REVOKE_BUILDER_API_KEY: &str = "/auth/builder-api-key";

    // Tags
    pub const GET_TAGS: &str = "/tags";
    pub const GET_TAG_BY_SLUG: &str = "/tags/slug/";

    // Events
    pub const GET_EVENTS: &str = "/events";
    pub const GET_EVENT: &str = "/events/";
    pub const GET_EVENT_BY_SLUG: &str = "/events/slug/";

    // Markets
    pub const GET_MARKETS: &str = "/markets";
    pub const GET_MARKET: &str = "/markets/";
    pub const GET_MARKET_BY_SLUG: &str = "/markets/slug/";

    // Orderbook
    pub const GET_ORDER_BOOK: &str = "/book";
    pub const GET_ORDER_BOOKS: &str = "/books";

    // Prices
    pub const GET_PRICE: &str = "/price";
    pub const GET_PRICES: &str = "/prices";
    pub const GET_MIDPOINT: &str = "/midpoint";
    pub const GET_MIDPOINTS: &str = "/midpoints";
    pub const GET_PRICES_HISTORY: &str = "/prices-history";
    pub const GET_LAST_TRADE_PRICE: &str = "/last-trade-price";
    pub const GET_LAST_TRADES_PRICES: &str = "/last-trades-prices";

    // Token
    pub const GET_SPREADS: &str = "/spreads";
    pub const GET_TICK_SIZE: &str = "/tick-size";
    pub const GET_NEG_RISK: &str = "/neg-risk";
    pub const GET_FEE_RATE: &str = "/fee-rate";

    // Order endpoints
    pub const POST_ORDER: &str = "/order";
    pub const POST_ORDERS: &str = "/orders";
    pub const CANCEL_ORDER: &str = "/order";
    pub const CANCEL_ORDERS: &str = "/orders";
    pub const GET_ORDER: &str = "/data/order/";
    pub const CANCEL_ALL: &str = "/cancel-all";
    pub const CANCEL_MARKET_ORDERS: &str = "/cancel-market-orders";
    pub const GET_OPEN_ORDERS: &str = "/data/orders";
    pub const GET_TRADES: &str = "/data/trades";
    pub const IS_ORDER_SCORING: &str = "/order-scoring";
    pub const ARE_ORDERS_SCORING: &str = "/orders-scoring";

    // Notifications
    pub const GET_NOTIFICATIONS: &str = "/notifications";
    pub const DROP_NOTIFICATIONS: &str = "/notifications";

    // Balance
    pub const GET_BALANCE_ALLOWANCE: &str = "/balance-allowance";
    pub const UPDATE_BALANCE_ALLOWANCE: &str = "/balance-allowance/update";

    // User Rewards
    pub const GET_EARNINGS_FOR_USER_FOR_DAY: &str = "/rewards/user";
    pub const GET_TOTAL_EARNINGS_FOR_USER_FOR_DAY: &str = "/rewards/user/total";
    pub const GET_LIQUIDITY_REWARD_PERCENTAGES: &str = "/rewards/user/percentages";
    pub const GET_REWARDS_EARNINGS_PERCENTAGES: &str = "/rewards/user/markets";

    // Builder endpoints
    pub const GET_BUILDER_TRADES: &str = "/builder/trades";
}
