use crate::constants::{CLOB_DOMAIN_NAME, CLOB_VERSION, MSG_TO_SIGN};
use crate::errors::{ClobError, ClobResult};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{
    transaction::eip712::{EIP712Domain, Eip712},
    Address, U256,
};
use serde::{Deserialize, Serialize};

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
            chain_id: None,
            verifying_contract: None,
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
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

    let mut domain = clob_auth.domain()?;
    domain.chain_id = Some(U256::from(chain_id));

    let signature = wallet
        .sign_typed_data(&clob_auth)
        .await
        .map_err(|e| ClobError::SigningError(e.to_string()))?;

    Ok(format!("0x{}", hex::encode(signature.to_vec())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eip712_signature() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let result = build_clob_eip712_signature(&wallet, 137, 1234567890, 0).await;
        
        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(signature.starts_with("0x"));
        assert_eq!(signature.len(), 132);
    }
}
