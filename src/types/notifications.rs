use serde::{Deserialize, Serialize};

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

