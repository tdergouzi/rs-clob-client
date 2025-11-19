use crate::constants::{CLOB_DOMAIN_NAME, CLOB_VERSION, MSG_TO_SIGN};
use crate::errors::{ClobError, ClobResult};
use ethers::types::{
    transaction::eip712::{EIP712Domain, Eip712},
    Address, U256,
};
use ethers::signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};

/// ClobAuth EIP-712 struct for signing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClobAuth {
    pub address: Address,
    pub timestamp: String,
    pub nonce: U256,
    pub message: String,
}

impl Eip712 for ClobAuth {
    type Error = ClobError;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(EIP712Domain {
            name: Some(CLOB_DOMAIN_NAME.to_string()),
            version: Some(CLOB_VERSION.to_string()),
            chain_id: None, // Will be set in the function
            verifying_contract: None,
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        // EIP-712 type hash for:
        // ClobAuth(address address,string timestamp,uint256 nonce,string message)
        Ok(ethers::utils::keccak256(
            b"ClobAuth(address address,string timestamp,uint256 nonce,string message)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        use ethers::abi::{encode, Token};

        let tokens = vec![
            Token::Uint(U256::from(Self::type_hash()?)),
            Token::Address(self.address),
            Token::Uint(U256::from(ethers::utils::keccak256(
                self.timestamp.as_bytes(),
            ))),
            Token::Uint(self.nonce),
            Token::Uint(U256::from(ethers::utils::keccak256(
                self.message.as_bytes(),
            ))),
        ];

        Ok(ethers::utils::keccak256(encode(&tokens)))
    }
}

/// Builds the canonical Polymarket CLOB EIP-712 signature
///
/// This creates an EIP-712 signature for L1 authentication.
/// The signature proves control over a wallet without requiring
/// the user to send a transaction.
///
/// # Arguments
///
/// * `wallet` - LocalWallet containing the private key
/// * `chain_id` - Blockchain network ID (137 for Polygon, 80002 for Amoy)
/// * `timestamp` - Unix timestamp in seconds
/// * `nonce` - Nonce for the signature
///
/// # Returns
///
/// Hex-encoded signature string (0x...)
pub async fn build_clob_eip712_signature(
    wallet: &LocalWallet,
    chain_id: u64,
    timestamp: u64,
    nonce: u64,
) -> ClobResult<String> {
    let address = wallet.address();
    
    let clob_auth = ClobAuth {
        address,
        timestamp: timestamp.to_string(),
        nonce: U256::from(nonce),
        message: MSG_TO_SIGN.to_string(),
    };

    // Create domain with chain_id
    let mut domain = clob_auth.domain()?;
    domain.chain_id = Some(U256::from(chain_id));

    // Sign the typed data
    let signature = wallet
        .sign_typed_data(&clob_auth)
        .await
        .map_err(|e| ClobError::SigningError(e.to_string()))?;

    // Convert to hex string with 0x prefix
    Ok(format!("0x{}", hex::encode(signature.to_vec())))
}

/// Helper function to build signature with wallet from private key string
pub async fn build_clob_eip712_signature_from_key(
    private_key: &str,
    chain_id: u64,
    timestamp: u64,
    nonce: u64,
) -> ClobResult<String> {
    let wallet: LocalWallet = private_key
        .parse()
        .map_err(|e| ClobError::WalletError(format!("Invalid private key: {}", e)))?;
    
    build_clob_eip712_signature(&wallet, chain_id, timestamp, nonce).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eip712_signature() {
        // Create a test wallet
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let chain_id = 137; // Polygon
        let timestamp = 1234567890;
        let nonce = 0;

        let result = build_clob_eip712_signature(&wallet, chain_id, timestamp, nonce).await;
        assert!(result.is_ok());

        let signature = result.unwrap();
        // Should start with 0x and be a valid hex string
        assert!(signature.starts_with("0x"));
        assert_eq!(signature.len(), 132); // 0x + 130 hex chars (65 bytes)
    }

    #[tokio::test]
    async fn test_clob_auth_domain() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let clob_auth = ClobAuth {
            address: wallet.address(),
            timestamp: "1234567890".to_string(),
            nonce: U256::from(0),
            message: MSG_TO_SIGN.to_string(),
        };

        let domain = clob_auth.domain().unwrap();
        assert_eq!(domain.name, Some(CLOB_DOMAIN_NAME.to_string()));
        assert_eq!(domain.version, Some(CLOB_VERSION.to_string()));
    }
}

