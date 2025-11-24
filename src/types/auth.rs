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

