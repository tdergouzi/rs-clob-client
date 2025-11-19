use crate::errors::ClobResult;
use crate::signing::hmac::build_poly_hmac_signature;
use crate::types::ApiKeyCreds;
use ethers::signers::{LocalWallet, Signer};
use std::collections::HashMap;

/// Creates L2 authentication headers using HMAC signature
///
/// L2 auth is used for trading operations and requires API credentials.
/// It uses HMAC-SHA256 for fast, stateless authentication.
///
/// # Arguments
///
/// * `wallet` - LocalWallet (for address)
/// * `creds` - API key credentials
/// * `method` - HTTP method (GET, POST, DELETE)
/// * `request_path` - API endpoint path
/// * `body` - Optional request body (JSON string)
/// * `timestamp` - Optional timestamp (defaults to current time)
///
/// # Returns
///
/// HashMap with headers:
/// - POLY_ADDRESS: Wallet address
/// - POLY_SIGNATURE: HMAC signature
/// - POLY_TIMESTAMP: Unix timestamp
/// - POLY_API_KEY: API key
/// - POLY_PASSPHRASE: API passphrase
pub async fn create_l2_headers(
    wallet: &LocalWallet,
    creds: &ApiKeyCreds,
    method: &str,
    request_path: &str,
    body: Option<&str>,
    timestamp: Option<u64>,
) -> ClobResult<HashMap<String, String>> {
    let ts = timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let signature = build_poly_hmac_signature(&creds.secret, ts, method, request_path, body)?;
    let address = format!("{:?}", wallet.address());

    let mut headers = HashMap::new();
    headers.insert("POLY_ADDRESS".to_string(), address);
    headers.insert("POLY_SIGNATURE".to_string(), signature);
    headers.insert("POLY_TIMESTAMP".to_string(), ts.to_string());
    headers.insert("POLY_API_KEY".to_string(), creds.key.clone());
    headers.insert("POLY_PASSPHRASE".to_string(), creds.passphrase.clone());

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose, Engine as _};

    #[tokio::test]
    async fn test_create_l2_headers() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let creds = ApiKeyCreds {
            key: "test-key".to_string(),
            secret: general_purpose::STANDARD.encode("test-secret"),
            passphrase: "test-passphrase".to_string(),
        };

        let result = create_l2_headers(&wallet, &creds, "GET", "/test", None, None).await;
        assert!(result.is_ok());

        let headers = result.unwrap();
        assert!(headers.contains_key("POLY_ADDRESS"));
        assert!(headers.contains_key("POLY_SIGNATURE"));
        assert!(headers.contains_key("POLY_TIMESTAMP"));
        assert!(headers.contains_key("POLY_API_KEY"));
        assert!(headers.contains_key("POLY_PASSPHRASE"));

        assert_eq!(headers["POLY_API_KEY"], "test-key");
        assert_eq!(headers["POLY_PASSPHRASE"], "test-passphrase");
    }

    #[tokio::test]
    async fn test_create_l2_headers_with_body() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let creds = ApiKeyCreds {
            key: "test-key".to_string(),
            secret: general_purpose::STANDARD.encode("test-secret"),
            passphrase: "test-passphrase".to_string(),
        };

        let body = r#"{"tokenID":"123","price":0.5}"#;
        let result = create_l2_headers(&wallet, &creds, "POST", "/order", Some(body), None).await;
        assert!(result.is_ok());

        let headers = result.unwrap();
        // Signature should be different when body is included
        assert!(!headers["POLY_SIGNATURE"].is_empty());
    }

    #[tokio::test]
    async fn test_create_l2_headers_with_custom_timestamp() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let creds = ApiKeyCreds {
            key: "test-key".to_string(),
            secret: general_purpose::STANDARD.encode("test-secret"),
            passphrase: "test-passphrase".to_string(),
        };

        let timestamp = Some(1234567890);
        let result =
            create_l2_headers(&wallet, &creds, "GET", "/test", None, timestamp).await;
        assert!(result.is_ok());

        let headers = result.unwrap();
        assert_eq!(headers["POLY_TIMESTAMP"], "1234567890");
    }
}


