use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(rename = "1m")]
    OneMinute,
}

impl PriceHistoryInterval {
    pub fn to_string(&self) -> String {
        match self {
            PriceHistoryInterval::Max => "max".to_string(),
            PriceHistoryInterval::OneWeek => "1w".to_string(),
            PriceHistoryInterval::OneDay => "1d".to_string(),
            PriceHistoryInterval::SixHours => "6h".to_string(),
            PriceHistoryInterval::OneHour => "1h".to_string(),
            PriceHistoryInterval::OneMinute => "1m".to_string(),
        }
    }
}

// ============================================================================
// Utility Types
// ============================================================================

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
// Cache Types
// ============================================================================

/// Tick sizes cache
pub type TickSizes = HashMap<String, TickSize>;

/// Negative risk flags cache
pub type NegRisk = HashMap<String, bool>;

/// Fee rates cache
pub type FeeRates = HashMap<String, u32>;

