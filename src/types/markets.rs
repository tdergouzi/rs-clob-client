use serde::{Deserialize, Serialize};

use super::orders::MakerOrder;
use super::primitives::{AssetType, PriceHistoryInterval, Side, TraderSide};

// ============================================================================
// Market Data
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationPayload {
    pub limit: u32,
    pub count: u32,
    pub next_cursor: String,
    pub data: Vec<serde_json::Value>,
}

pub struct TagParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub slug: String,
    pub force_show: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub force_hide: Option<bool>,
    pub is_carousel: Option<bool>,
}

pub struct EventParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub tag_id: Option<u64>,
    pub closed: Option<bool>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
}

/// Event from the /events endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolutionSource", skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(rename = "openInterest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "published_at", skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitive: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<f64>,
    #[serde(rename = "featuredImage", skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<String>,
    #[serde(rename = "liquidityAmm", skip_serializing_if = "Option::is_none")]
    pub liquidity_amm: Option<f64>,
    #[serde(rename = "liquidityClob", skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<f64>,
    #[serde(rename = "enableOrderBook", skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "negRisk", skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "negRiskMarketID", skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "commentCount", skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i64>,
    // Nested objects - using serde_json::Value for flexibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "closedTime", skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "showAllOutcomes", skip_serializing_if = "Option::is_none")]
    pub show_all_outcomes: Option<bool>,
    #[serde(rename = "showMarketImages", skip_serializing_if = "Option::is_none")]
    pub show_market_images: Option<bool>,
    #[serde(
        rename = "automaticallyResolved",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "enableNegRisk", skip_serializing_if = "Option::is_none")]
    pub enable_neg_risk: Option<bool>,
    #[serde(
        rename = "automaticallyActive",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatically_active: Option<bool>,
    #[serde(rename = "seriesSlug", skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "negRiskAugmented", skip_serializing_if = "Option::is_none")]
    pub neg_risk_augmented: Option<bool>,
    #[serde(rename = "pendingDeployment", skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "isTemplate", skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
}

pub struct MarketParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub condition_id: Option<String>,
    pub closed: Option<bool>,
}

/// Market from the /markets endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome_prices: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_num: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_num: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "startDateIso", skip_serializing_if = "Option::is_none")]
    pub start_date_iso: Option<String>,
    #[serde(rename = "endDateIso", skip_serializing_if = "Option::is_none")]
    pub end_date_iso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_maker_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price_min_tick_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_min_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepting_orders_timestamp: Option<String>,
    // Fee structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_base_fee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_base_fee: Option<f64>,
    // Volume metrics - time periods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<f64>,
    // Volume metrics - CLOB
    #[serde(rename = "volume24hrClob", skip_serializing_if = "Option::is_none")]
    pub volume24hr_clob: Option<f64>,
    #[serde(rename = "volume1wkClob", skip_serializing_if = "Option::is_none")]
    pub volume1wk_clob: Option<f64>,
    #[serde(rename = "volume1moClob", skip_serializing_if = "Option::is_none")]
    pub volume1mo_clob: Option<f64>,
    #[serde(rename = "volume1yrClob", skip_serializing_if = "Option::is_none")]
    pub volume1yr_clob: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_clob: Option<f64>,
    // Volume metrics - AMM
    #[serde(rename = "volume24hrAmm", skip_serializing_if = "Option::is_none")]
    pub volume24hr_amm: Option<f64>,
    #[serde(rename = "volume1wkAmm", skip_serializing_if = "Option::is_none")]
    pub volume1wk_amm: Option<f64>,
    #[serde(rename = "volume1moAmm", skip_serializing_if = "Option::is_none")]
    pub volume1mo_amm: Option<f64>,
    #[serde(rename = "volume1yrAmm", skip_serializing_if = "Option::is_none")]
    pub volume1yr_amm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_amm: Option<f64>,
    // Liquidity metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_amm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_item_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trade_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_day_price_change: Option<f64>,
    // Nested objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
}

/// Book parameters for batch requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookParams {
    pub token_id: String,
    pub side: Option<Side>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceParams {
    pub token_id: String,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Midpoint {
    pub mid: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceHistoryParams {
    pub token_id: String,
    pub fidelity: u32,
    #[serde(rename = "startTs", skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<u64>,
    #[serde(rename = "endTs", skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PriceHistoryInterval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPriceItem {
    /// Timestamp
    pub t: u64,
    /// Price
    pub p: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPrice {
    pub history: Vec<HistoryPriceItem>,
}

// ============================================================================
// Trading Data
// ============================================================================

/// Order summary in orderbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub price: String,
    pub size: String,
}
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
