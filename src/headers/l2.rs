use crate::errors::ClobResult;
use crate::signing::hmac::build_poly_hmac_signature;
use crate::types::{ApiKeyCreds, L2PolyHeader, L2WithBuilderHeader};
use alloy_signer_local::PrivateKeySigner;
use rs_builder_signing_sdk::BuilderHeaderPayload;

/// Creates L2 authentication headers using HMAC-SHA256 for trading operations
pub async fn create_l2_headers(
    wallet: &PrivateKeySigner,
    creds: &ApiKeyCreds,
    method: &str,
    request_path: &str,
    body: Option<&str>,
    timestamp: Option<u64>,
) -> ClobResult<L2PolyHeader> {
    let ts = timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let signature = build_poly_hmac_signature(&creds.secret, ts, method, request_path, body)?;
    let address = format!("{:#x}", wallet.address());

    Ok(L2PolyHeader {
        poly_address: address,
        poly_signature: signature,
        poly_timestamp: ts.to_string(),
        poly_api_key: creds.key.clone(),
        poly_passphrase: creds.passphrase.clone(),
    })
}

/// Combines L2 headers with builder authentication headers
pub fn inject_builder_headers(
    l2_headers: L2PolyHeader,
    builder_headers: BuilderHeaderPayload,
) -> L2WithBuilderHeader {
    let poly_builder_api_key = builder_headers
        .get("POLY_BUILDER_API_KEY")
        .cloned()
        .unwrap_or_default();
    let poly_builder_timestamp = builder_headers
        .get("POLY_BUILDER_TIMESTAMP")
        .cloned()
        .unwrap_or_default();
    let poly_builder_passphrase = builder_headers
        .get("POLY_BUILDER_PASSPHRASE")
        .cloned()
        .unwrap_or_default();
    let poly_builder_signature = builder_headers
        .get("POLY_BUILDER_SIGNATURE")
        .cloned()
        .unwrap_or_default();

    L2WithBuilderHeader {
        poly_address: l2_headers.poly_address,
        poly_signature: l2_headers.poly_signature,
        poly_timestamp: l2_headers.poly_timestamp,
        poly_api_key: l2_headers.poly_api_key,
        poly_passphrase: l2_headers.poly_passphrase,
        poly_builder_api_key,
        poly_builder_timestamp,
        poly_builder_passphrase,
        poly_builder_signature,
    }
}
