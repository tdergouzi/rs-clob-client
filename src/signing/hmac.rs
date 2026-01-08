use crate::errors::ClobResult;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Builds the canonical Polymarket CLOB HMAC signature
pub fn build_poly_hmac_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    request_path: &str,
    body: Option<&str>,
) -> ClobResult<String> {
    let mut message = format!("{}{}{}", timestamp, method, request_path);
    if let Some(body_str) = body {
        message.push_str(body_str);
    }

    let secret_bytes = general_purpose::URL_SAFE.decode(secret)?;

    let mut mac = HmacSha256::new_from_slice(&secret_bytes)
        .map_err(|e| crate::errors::ClobError::SigningError(e.to_string()))?;

    mac.update(message.as_bytes());

    let result = mac.finalize();
    let signature_bytes = result.into_bytes();

    let signature = general_purpose::STANDARD.encode(&signature_bytes);

    let url_safe = signature.replace('+', "-").replace('/', "_");

    Ok(url_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_signature() {
        let secret = general_purpose::STANDARD.encode("test_secret_key");
        let sig = build_poly_hmac_signature(&secret, 1234567890, "GET", "/test", None);

        assert!(sig.is_ok());
        let signature = sig.unwrap();
        assert!(!signature.contains('+'));
        assert!(!signature.contains('/'));
    }

    #[test]
    fn test_hmac_with_body() {
        let secret = general_purpose::STANDARD.encode("test_secret");
        let body = r#"{"tokenID":"123","price":0.5}"#;
        let sig = build_poly_hmac_signature(&secret, 1234567890, "POST", "/order", Some(body));

        assert!(sig.is_ok());
    }
}
