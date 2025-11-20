use crate::errors::ClobResult;
use crate::signing::eip712::build_clob_eip712_signature;
use crate::types::L1PolyHeader;
use ethers::signers::{LocalWallet, Signer};

/// Creates L1 authentication headers using EIP-712 signature for API key management
pub async fn create_l1_headers(
    wallet: &LocalWallet,
    chain_id: u64,
    nonce: Option<u64>,
    timestamp: Option<u64>,
) -> ClobResult<L1PolyHeader> {
    let ts = timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let n = nonce.unwrap_or(0);

    let signature = build_clob_eip712_signature(wallet, chain_id, ts, n).await?;
    let address = format!("{:?}", wallet.address());

    Ok(L1PolyHeader {
        poly_address: address,
        poly_signature: signature,
        poly_timestamp: ts.to_string(),
        poly_nonce: n.to_string(),
    })
}
