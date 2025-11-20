use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Core Authentication & API Keys
// ============================================================================

/// API key credentials for L2 authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreds {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

/// Raw API key response from server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyRaw {
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
}

impl From<ApiKeyRaw> for ApiKeyCreds {
    fn from(raw: ApiKeyRaw) -> Self {
        Self {
            key: raw.api_key,
            secret: raw.secret,
            passphrase: raw.passphrase,
        }
    }
}

/// Response containing multiple API keys
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeysResponse {
    pub api_keys: Vec<ApiKeyCreds>,
}

/// Builder API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderApiKey {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

/// Builder API key response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderApiKeyResponse {
    pub key: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "revokedAt")]
    pub revoked_at: Option<String>,
}

// ============================================================================
// Fundamental Enums
// ============================================================================

/// Blockchain network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chain {
    /// Polygon mainnet
    #[serde(rename = "137")]
    Polygon = 137,
    /// Amoy testnet
    #[serde(rename = "80002")]
    Amoy = 80002,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        *self as u64
    }
}

/// Order side (buy or sell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    /// Convert Side to uppercase string for API requests
    pub fn to_uppercase(&self) -> String {
        match self {
            Side::Buy => "BUY".to_string(),
            Side::Sell => "SELL".to_string(),
        }
    }
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Good Till Cancel - standard limit order
    Gtc,
    /// Fill or Kill - must execute completely or not at all
    Fok,
    /// Good Till Date - limit order with expiration
    Gtd,
    /// Fill and Kill - partial fills allowed, cancel remainder
    Fak,
}

/// Asset type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetType {
    Collateral,
    Conditional,
}

/// Trader side in a trade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TraderSide {
    Taker,
    Maker,
}

/// Tick size type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TickSize {
    #[serde(rename = "0.1")]
    ZeroPointOne,
    #[serde(rename = "0.01")]
    ZeroPointZeroOne,
    #[serde(rename = "0.001")]
    ZeroPointZeroZeroOne,
    #[serde(rename = "0.0001")]
    ZeroPointZeroZeroZeroOne,
}

impl TickSize {
    pub fn as_f64(&self) -> f64 {
        match self {
            TickSize::ZeroPointOne => 0.1,
            TickSize::ZeroPointZeroOne => 0.01,
            TickSize::ZeroPointZeroZeroOne => 0.001,
            TickSize::ZeroPointZeroZeroZeroOne => 0.0001,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TickSize::ZeroPointOne => "0.1",
            TickSize::ZeroPointZeroOne => "0.01",
            TickSize::ZeroPointZeroZeroOne => "0.001",
            TickSize::ZeroPointZeroZeroZeroOne => "0.0001",
        }
    }
}

/// Price history interval
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PriceHistoryInterval {
    Max,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "1h")]
    OneHour,
}

// ============================================================================
// Order Types & Parameters
// ============================================================================

/// Simplified user order for creating limit orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrder {
    /// Token ID of the conditional token asset being traded
    #[serde(rename = "tokenID")]
    pub token_id: String,

    /// Price used to create the order
    pub price: f64,

    /// Size in terms of the ConditionalToken
    pub size: f64,

    /// Side of the order
    pub side: Side,

    /// Fee rate, in basis points, charged to the order maker
    #[serde(rename = "feeRateBps", skip_serializing_if = "Option::is_none")]
    pub fee_rate_bps: Option<u32>,

    /// Nonce used for onchain cancellations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,

    /// Timestamp after which the order is expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<u64>,

    /// Address of the order taker (zero address = public order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker: Option<Address>,
}

/// Simplified market order for users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMarketOrder {
    /// Token ID of the conditional token asset being traded
    #[serde(rename = "tokenID")]
    pub token_id: String,

    /// Price (if not present, market price will be calculated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// BUY orders: $$$ Amount to buy
    /// SELL orders: Shares to sell
    pub amount: f64,

    /// Side of the order
    pub side: Side,

    /// Fee rate, in basis points
    #[serde(rename = "feeRateBps", skip_serializing_if = "Option::is_none")]
    pub fee_rate_bps: Option<u32>,

    /// Nonce used for onchain cancellations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,

    /// Address of the order taker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker: Option<Address>,

    /// Order type (FOK or FAK)
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
}

/// Order payload for cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPayload {
    pub order_id: String,
}

/// Order market cancel parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMarketCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
}

/// Arguments for posting multiple orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostOrdersArgs {
    pub order: serde_json::Value,
    pub order_type: OrderType,
}

/// Open order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    pub id: String,
    pub status: String,
    pub owner: String,
    pub maker_address: String,
    pub market: String,
    pub asset_id: String,
    pub side: String,
    pub original_size: String,
    pub size_matched: String,
    pub price: String,
    pub associate_trades: Vec<String>,
    pub outcome: String,
    pub created_at: u64,
    pub expiration: String,
    pub order_type: String,
}

/// Open orders response
pub type OpenOrdersResponse = Vec<OpenOrder>;

/// Open order parameters for filtering
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
}

/// Create order options
#[derive(Debug, Clone)]
pub struct CreateOrderOptions {
    pub tick_size: TickSize,
    pub neg_risk: Option<bool>,
}

/// Round configuration for price calculations
#[derive(Debug, Clone)]
pub struct RoundConfig {
    pub price: u32,
    pub size: u32,
    pub amount: u32,
}

// ============================================================================
// Trading & Market Data
// ============================================================================

/// Maker order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakerOrder {
    pub order_id: String,
    pub owner: String,
    pub maker_address: String,
    pub matched_amount: String,
    pub price: String,
    pub fee_rate_bps: String,
    pub asset_id: String,
    pub outcome: String,
    pub side: Side,
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
// Order Scoring
// ============================================================================

/// Order scoring parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderScoringParams {
    pub order_id: String,
}

/// Order scoring response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderScoring {
    pub scoring: bool,
}

/// Orders scoring parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdersScoringParams {
    pub order_ids: Vec<String>,
}

/// Orders scoring response
pub type OrdersScoring = HashMap<String, bool>;

// ============================================================================
// Rewards & Earnings
// ============================================================================

/// User earning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEarning {
    pub date: String,
    pub condition_id: String,
    pub asset_address: String,
    pub maker_address: String,
    pub earnings: f64,
    pub asset_rate: f64,
}

/// Total user earning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalUserEarning {
    pub date: String,
    pub asset_address: String,
    pub maker_address: String,
    pub earnings: f64,
    pub asset_rate: f64,
}

/// Rewards percentages
pub type RewardsPercentages = HashMap<String, f64>;

/// Token info for rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_id: String,
    pub outcome: String,
    pub price: f64,
}

/// Rewards config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardsConfig {
    pub date: String,
    pub asset_address: String,
    pub rewards_daily_rate: f64,
}

/// Market reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketReward {
    pub condition_id: String,
    pub question: String,
    pub market_slug: String,
    pub event_slug: String,
    pub image: String,
    pub rewards_max_spread: f64,
    pub rewards_min_size: f64,
    pub tokens: Vec<Token>,
    pub rewards_config: Vec<RewardsConfig>,
}

/// User rewards earning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRewardsEarning {
    pub condition_id: String,
    pub question: String,
    pub market_slug: String,
    pub event_slug: String,
    pub image: String,
    pub rewards_max_spread: f64,
    pub rewards_min_size: f64,
    pub market_competitiveness: f64,
    pub tokens: Vec<Token>,
    pub rewards_config: Vec<RewardsConfig>,
}

// ============================================================================
// Notifications
// ============================================================================

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "type")]
    pub notification_type: u32,
    pub owner: String,
    pub payload: serde_json::Value,
}

/// Drop notification parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropNotificationParams {
    pub ids: Vec<String>,
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

// ============================================================================
// Authentication Headers
// ============================================================================

/// L1 authentication headers (EIP-712 signature based)
/// Used for API key management operations
#[derive(Debug, Clone)]
pub struct L1PolyHeader {
    pub poly_address: String,
    pub poly_signature: String,
    pub poly_timestamp: String,
    pub poly_nonce: String,
}

impl L1PolyHeader {
    /// Converts the struct to a HashMap for HTTP client usage
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("POLY_ADDRESS".to_string(), self.poly_address.clone());
        headers.insert("POLY_SIGNATURE".to_string(), self.poly_signature.clone());
        headers.insert("POLY_TIMESTAMP".to_string(), self.poly_timestamp.clone());
        headers.insert("POLY_NONCE".to_string(), self.poly_nonce.clone());
        headers
    }
}

/// L2 authentication headers (HMAC signature based)
/// Used for trading operations with API credentials
#[derive(Debug, Clone)]
pub struct L2PolyHeader {
    pub poly_address: String,
    pub poly_signature: String,
    pub poly_timestamp: String,
    pub poly_api_key: String,
    pub poly_passphrase: String,
}

impl L2PolyHeader {
    /// Converts the struct to a HashMap for HTTP client usage
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("POLY_ADDRESS".to_string(), self.poly_address.clone());
        headers.insert("POLY_SIGNATURE".to_string(), self.poly_signature.clone());
        headers.insert("POLY_TIMESTAMP".to_string(), self.poly_timestamp.clone());
        headers.insert("POLY_API_KEY".to_string(), self.poly_api_key.clone());
        headers.insert("POLY_PASSPHRASE".to_string(), self.poly_passphrase.clone());
        headers
    }
}

/// L2 headers with builder authentication
/// Combines L2 headers with builder-specific headers
#[derive(Debug, Clone)]
pub struct L2WithBuilderHeader {
    pub poly_address: String,
    pub poly_signature: String,
    pub poly_timestamp: String,
    pub poly_api_key: String,
    pub poly_passphrase: String,
    pub poly_builder_api_key: String,
    pub poly_builder_timestamp: String,
    pub poly_builder_passphrase: String,
    pub poly_builder_signature: String,
}

impl L2WithBuilderHeader {
    /// Converts the struct to a HashMap for HTTP client usage
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("POLY_ADDRESS".to_string(), self.poly_address.clone());
        headers.insert("POLY_SIGNATURE".to_string(), self.poly_signature.clone());
        headers.insert("POLY_TIMESTAMP".to_string(), self.poly_timestamp.clone());
        headers.insert("POLY_API_KEY".to_string(), self.poly_api_key.clone());
        headers.insert("POLY_PASSPHRASE".to_string(), self.poly_passphrase.clone());
        headers.insert(
            "POLY_BUILDER_API_KEY".to_string(),
            self.poly_builder_api_key.clone(),
        );
        headers.insert(
            "POLY_BUILDER_TIMESTAMP".to_string(),
            self.poly_builder_timestamp.clone(),
        );
        headers.insert(
            "POLY_BUILDER_PASSPHRASE".to_string(),
            self.poly_builder_passphrase.clone(),
        );
        headers.insert(
            "POLY_BUILDER_SIGNATURE".to_string(),
            self.poly_builder_signature.clone(),
        );
        headers
    }
}

// ============================================================================
// Cache Types
// ============================================================================

/// Tick sizes cache
pub type TickSizes = HashMap<String, TickSize>;

/// Negative risk flags cache
pub type NegRisk = HashMap<String, bool>;

/// Fee rates cache
pub type FeeRates = HashMap<String, u32>;
