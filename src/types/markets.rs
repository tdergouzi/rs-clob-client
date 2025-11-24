use serde::{Deserialize, Serialize};

use super::primitives::{AssetType, PriceHistoryInterval, Side, TraderSide};
use super::orders::MakerOrder;

// ============================================================================
// Trading & Market Data
// ============================================================================

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub taker_order_id: String,
    pub market: String,
    pub asset_id: String,
    pub side: Side,
    pub size: String,
    pub fee_rate_bps: String,
    pub price: String,
    pub status: String,
    pub match_time: String,
    pub last_update: String,
    pub outcome: String,
    pub bucket_index: u32,
    pub owner: String,
    pub maker_address: String,
    pub maker_orders: Vec<MakerOrder>,
    pub transaction_hash: String,
    pub trader_side: TraderSide,
}

/// Trade parameters for filtering
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TradeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Paginated trades response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesPaginatedResponse {
    pub data: Vec<Trade>,
    pub next_cursor: String,
}

/// Order summary in orderbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub price: String,
    pub size: String,
}

/// Orderbook summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSummary {
    pub market: String,
    pub asset_id: String,
    pub timestamp: String,
    pub bids: Vec<OrderSummary>,
    pub asks: Vec<OrderSummary>,
    pub min_order_size: String,
    pub tick_size: String,
    pub neg_risk: bool,
    pub hash: String,
}

/// Book parameters for batch requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookParams {
    pub token_id: String,
    pub side: Side,
}

/// Market price point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    /// Timestamp
    pub t: u64,
    /// Price
    pub p: f64,
}

/// Price history filter parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceHistoryFilterParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "startTs", skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<u64>,
    #[serde(rename = "endTs", skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PriceHistoryInterval>,
}

// ============================================================================
// Balance & Allowance
// ============================================================================

/// Balance allowance parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAllowanceParams {
    pub asset_type: AssetType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}

/// Balance allowance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAllowanceResponse {
    pub balance: String,
    pub allowance: String,
}

/// Ban status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanStatus {
    pub closed_only: bool,
}

// ============================================================================
// Pagination & Events
// ============================================================================

/// Pagination payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationPayload {
    pub limit: u32,
    pub count: u32,
    pub next_cursor: String,
    pub data: Vec<serde_json::Value>,
}

/// Event from the /events endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub ticker: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "resolutionSource")]
    pub resolution_source: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub image: String,
    pub icon: String,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub new: bool,
    pub featured: bool,
    pub restricted: bool,
    pub liquidity: f64,
    pub volume: f64,
    #[serde(rename = "openInterest")]
    pub open_interest: f64,
    #[serde(rename = "sortBy")]
    pub sort_by: String,
    pub category: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub competitive: i32,
    pub volume24hr: f64,
    pub volume1wk: f64,
    pub volume1mo: f64,
    pub volume1yr: f64,
    #[serde(rename = "liquidityAmm")]
    pub liquidity_amm: f64,
    #[serde(rename = "liquidityClob")]
    pub liquidity_clob: f64,
    #[serde(rename = "commentCount")]
    pub comment_count: i64,
    // Nested objects - using serde_json::Value for flexibility
    pub markets: Vec<serde_json::Value>,
    pub series: Vec<serde_json::Value>,
    pub tags: Vec<serde_json::Value>,
    pub cyom: bool,
    #[serde(rename = "closedTime")]
    pub closed_time: String,
    #[serde(rename = "showAllOutcomes")]
    pub show_all_outcomes: bool,
    #[serde(rename = "showMarketImages")]
    pub show_market_images: bool,
    #[serde(rename = "enableNegRisk")]
    pub enable_neg_risk: bool,
    #[serde(rename = "seriesSlug")]
    pub series_slug: String,
    #[serde(rename = "negRiskAugmented")]
    pub neg_risk_augmented: bool,
    #[serde(rename = "pendingDeployment")]
    pub pending_deployment: bool,
    pub deploying: bool,
}

/// Market trade event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTradeEvent {
    pub event_type: String,
    pub market: MarketInfo,
    pub user: UserInfo,
    pub side: Side,
    pub size: String,
    pub fee_rate_bps: String,
    pub price: String,
    pub outcome: String,
    pub outcome_index: u32,
    pub transaction_hash: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInfo {
    pub condition_id: String,
    pub asset_id: String,
    pub question: String,
    pub icon: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub address: String,
    pub username: String,
    pub profile_picture: String,
    pub optimized_profile_picture: String,
    pub pseudonym: String,
}

// ============================================================================
// Builder Types
// ============================================================================

/// Builder trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderTrade {
    pub id: String,
    #[serde(rename = "tradeType")]
    pub trade_type: String,
    #[serde(rename = "takerOrderHash")]
    pub taker_order_hash: String,
    pub builder: String,
    pub market: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub side: String,
    pub size: String,
    #[serde(rename = "sizeUsdc")]
    pub size_usdc: String,
    pub price: String,
    pub status: String,
    pub outcome: String,
    #[serde(rename = "outcomeIndex")]
    pub outcome_index: u32,
    pub owner: String,
    pub maker: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "matchTime")]
    pub match_time: String,
    #[serde(rename = "bucketIndex")]
    pub bucket_index: u32,
    pub fee: String,
    #[serde(rename = "feeUsdc")]
    pub fee_usdc: String,
    pub err_msg: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

/// Builder trades response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderTradesResponse {
    pub data: Vec<BuilderTrade>,
    pub next_cursor: String,
}

