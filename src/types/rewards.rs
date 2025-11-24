use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

