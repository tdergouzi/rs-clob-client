use crate::constants::{CLOB_DOMAIN_NAME, CLOB_VERSION, MSG_TO_SIGN};
use crate::errors::{ClobError, ClobResult};
use alloy_primitives::{keccak256, Address, B256, U256};
use alloy_signer::Signer;
use alloy_signer_local::PrivateKeySigner;
use serde::{Deserialize, Serialize};

/// ClobAuth structure for EIP-712 signing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClobAuth {
    pub address: Address,
    pub timestamp: String,
    pub nonce: U256,
    pub message: String,
}

impl ClobAuth {
    /// EIP-712 type string
    const TYPE_STRING: &'static str =
        "ClobAuth(address address,string timestamp,uint256 nonce,string message)";

    /// Compute the EIP-712 domain separator
    fn domain_separator(chain_id: u64) -> B256 {
        // EIP712Domain(string name,string version,uint256 chainId)
        let domain_type_hash =
            keccak256(b"EIP712Domain(string name,string version,uint256 chainId)");
        let name_hash = keccak256(CLOB_DOMAIN_NAME.as_bytes());
        let version_hash = keccak256(CLOB_VERSION.as_bytes());

        // Encode: keccak256(abi.encode(typeHash, nameHash, versionHash, chainId))
        let mut encoded = Vec::new();
        encoded.extend_from_slice(domain_type_hash.as_slice());
        encoded.extend_from_slice(name_hash.as_slice());
        encoded.extend_from_slice(version_hash.as_slice());

        // Encode chain_id as uint256 (32 bytes, big-endian)
        let chain_id_u256 = U256::from(chain_id);
        encoded.extend_from_slice(&chain_id_u256.to_be_bytes::<32>());

        keccak256(&encoded)
    }

    /// Compute the struct hash
    fn struct_hash(&self) -> B256 {
        let type_hash = keccak256(Self::TYPE_STRING.as_bytes());
        let timestamp_hash = keccak256(self.timestamp.as_bytes());
        let message_hash = keccak256(self.message.as_bytes());

        // Encode: keccak256(abi.encode(typeHash, address, keccak256(timestamp), nonce, keccak256(message)))
        let mut encoded = Vec::new();
        encoded.extend_from_slice(type_hash.as_slice());

        // Encode address as 32 bytes (left-padded to 32 bytes)
        let mut address_bytes = [0u8; 32];
        address_bytes[12..].copy_from_slice(self.address.as_slice());
        encoded.extend_from_slice(&address_bytes);

        encoded.extend_from_slice(timestamp_hash.as_slice());
        encoded.extend_from_slice(&self.nonce.to_be_bytes::<32>());
        encoded.extend_from_slice(message_hash.as_slice());

        keccak256(&encoded)
    }

    /// Compute the EIP-712 message hash
    fn eip712_hash(&self, chain_id: u64) -> B256 {
        let domain_separator = Self::domain_separator(chain_id);
        let struct_hash = self.struct_hash();

        // "\x19\x01" ‖ domainSeparator ‖ structHash
        let mut message = Vec::new();
        message.push(0x19);
        message.push(0x01);
        message.extend_from_slice(domain_separator.as_slice());
        message.extend_from_slice(struct_hash.as_slice());

        keccak256(&message)
    }
}

/// Builds the canonical Polymarket CLOB EIP-712 signature
pub async fn build_clob_eip712_signature(
    wallet: &PrivateKeySigner,
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

    // Compute the EIP-712 hash
    let message_hash = clob_auth.eip712_hash(chain_id);

    // Sign the hash
    let signature = wallet
        .sign_hash(&message_hash)
        .await
        .map_err(|e| ClobError::SigningError(e.to_string()))?;

    Ok(format!("0x{}", hex::encode(signature.as_bytes())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eip712_signature() {
        let wallet = PrivateKeySigner::random();
        let result = build_clob_eip712_signature(&wallet, 137, 1234567890, 0).await;

        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(signature.starts_with("0x"));
        assert_eq!(signature.len(), 132);
    }
}
