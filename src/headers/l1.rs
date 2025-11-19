use crate::errors::ClobResult;
use crate::signing::eip712::build_clob_eip712_signature;
use ethers::signers::{LocalWallet, Signer};
use std::collections::HashMap;

/// Creates L1 authentication headers using EIP-712 signature
///
/// L1 auth is used for API key management operations.
/// It proves wallet ownership through an EIP-712 signature.
///
/// # Arguments
///
/// * `wallet` - LocalWallet containing the private key
/// * `chain_id` - Blockchain network ID
/// * `nonce` - Optional nonce (defaults to 0)
/// * `timestamp` - Optional timestamp (defaults to current time)
///
/// # Returns
///
/// HashMap with headers:
/// - POLY_ADDRESS: Wallet address
/// - POLY_SIGNATURE: EIP-712 signature
/// - POLY_TIMESTAMP: Unix timestamp
/// - POLY_NONCE: Nonce value
pub async fn create_l1_headers(
    wallet: &LocalWallet,
    chain_id: u64,
    nonce: Option<u64>,
    timestamp: Option<u64>,
) -> ClobResult<HashMap<String, String>> {
    let ts = timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let n = nonce.unwrap_or(0);

    let signature = build_clob_eip712_signature(wallet, chain_id, ts, n).await?;
    let address = format!("{:?}", wallet.address());

    let mut headers = HashMap::new();
    headers.insert("POLY_ADDRESS".to_string(), address);
    headers.insert("POLY_SIGNATURE".to_string(), signature);
    headers.insert("POLY_TIMESTAMP".to_string(), ts.to_string());
    headers.insert("POLY_NONCE".to_string(), n.to_string());

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_l1_headers() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let chain_id = 137;

        let result = create_l1_headers(&wallet, chain_id, None, None).await;
        assert!(result.is_ok());

        let headers = result.unwrap();
        assert!(headers.contains_key("POLY_ADDRESS"));
        assert!(headers.contains_key("POLY_SIGNATURE"));
        assert!(headers.contains_key("POLY_TIMESTAMP"));
        assert!(headers.contains_key("POLY_NONCE"));

        // Signature should start with 0x
        assert!(headers["POLY_SIGNATURE"].starts_with("0x"));
    }

    #[tokio::test]
    async fn test_create_l1_headers_with_custom_values() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let chain_id = 137;
        let nonce = Some(42);
        let timestamp = Some(1234567890);

        let result = create_l1_headers(&wallet, chain_id, nonce, timestamp).await;
        assert!(result.is_ok());

        let headers = result.unwrap();
        assert_eq!(headers["POLY_NONCE"], "42");
        assert_eq!(headers["POLY_TIMESTAMP"], "1234567890");
    }
}


