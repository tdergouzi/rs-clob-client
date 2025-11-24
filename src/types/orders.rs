use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::primitives::{OrderType, Side};

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

