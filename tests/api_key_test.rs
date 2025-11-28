mod common;

use common::{create_test_client_with_api_key, create_test_client_with_wallet};

#[tokio::test]
async fn test_create_api_key() {
    let client = create_test_client_with_wallet();

    let nonce = Some(2);
    let result = client
        .create_api_key(nonce)
        .await
        .expect("Failed to create API key");

    println!(
        "=== API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_derive_api_key() {
    let client = create_test_client_with_wallet();

    let nonce = Some(2);
    let result = client
        .derive_api_key(nonce)
        .await
        .expect("Failed to derive API key");

    println!(
        "=== Derived API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_create_or_derive_api_key() {
    let client = create_test_client_with_wallet();

    let nonce = Some(2);
    let result = client
        .create_or_derive_api_key(nonce)
        .await
        .expect("Failed to create or derive API key");

    println!(
        "=== Created or Derived API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_get_api_keys() {
    let client = create_test_client_with_api_key();

    let result = client.get_api_keys().await.expect("Failed to get API keys");

    println!(
        "=== API Keys ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_delete_api_key() {
    let client = create_test_client_with_api_key();

    let result = client
        .delete_api_key()
        .await
        .expect("Failed to delete API key");

    println!(
        "=== Deleted API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_get_closed_only_mode() {
    let client = create_test_client_with_api_key();

    let result = client
        .get_closed_only_mode()
        .await
        .expect("Failed to get closed-only mode");

    println!(
        "=== Closed-Only Mode ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}
