use crate::errors::ClobResult;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Builds the canonical Polymarket CLOB HMAC signature
///
/// This creates an HMAC-SHA256 signature for L2 authentication.
/// The message format is: timestamp + method + requestPath + body (optional)
///
/// # Arguments
///
/// * `secret` - Base64-encoded secret key from API credentials
/// * `timestamp` - Unix timestamp in seconds
/// * `method` - HTTP method (GET, POST, DELETE)
/// * `request_path` - API endpoint path (e.g., "/order")
/// * `body` - Optional request body (JSON string)
///
/// # Returns
///
/// URL-safe base64-encoded signature
pub fn build_poly_hmac_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    request_path: &str,
    body: Option<&str>,
) -> ClobResult<String> {
    // Build the message to sign
    let mut message = format!("{}{}{}", timestamp, method, request_path);
    if let Some(body_str) = body {
        message.push_str(body_str);
    }

    // Decode the base64 secret
    let secret_bytes = general_purpose::STANDARD.decode(secret)?;

    // Create HMAC instance
    let mut mac = HmacSha256::new_from_slice(&secret_bytes)
        .map_err(|e| crate::errors::ClobError::SigningError(e.to_string()))?;

    // Update with message
    mac.update(message.as_bytes());

    // Get the signature
    let result = mac.finalize();
    let signature_bytes = result.into_bytes();

    // Encode to base64
    let signature = general_purpose::STANDARD.encode(&signature_bytes);

    // Convert to URL-safe base64
    // Replace '+' with '-' and '/' with '_', but keep '=' padding
    let url_safe = signature.replace('+', "-").replace('/', "_");

    Ok(url_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_signature() {
        // Create a test secret
        let secret = general_purpose::STANDARD.encode("test_secret_key");
        let timestamp = 1234567890;
        let method = "GET";
        let request_path = "/test";

        let sig = build_poly_hmac_signature(&secret, timestamp, method, request_path, None);
        assert!(sig.is_ok());

        let signature = sig.unwrap();
        // Should be base64-like but URL-safe
        assert!(!signature.contains('+'));
        assert!(!signature.contains('/'));
    }

    #[test]
    fn test_hmac_with_body() {
        let secret = general_purpose::STANDARD.encode("test_secret");
        let timestamp = 1234567890;
        let method = "POST";
        let request_path = "/order";
        let body = r#"{"tokenID":"123","price":0.5}"#;

        let sig = build_poly_hmac_signature(&secret, timestamp, method, request_path, Some(body));
        assert!(sig.is_ok());
    }
}

