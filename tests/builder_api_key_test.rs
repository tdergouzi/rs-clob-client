mod common;

use common::create_test_client_with_api_key;

#[tokio::test]
async fn test_create_builder_api_key() {
    let client = create_test_client_with_api_key();

    let result = client
        .create_builder_api_key()
        .await
        .expect("Failed to create builder API key");

    println!(
        "=== Create Builder API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_get_builder_api_keys() {
    let client = create_test_client_with_api_key();

    let result = client
        .get_builder_api_keys()
        .await
        .expect("Failed to get builder API keys");

    println!(
        "=== Get Builder API Keys ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_revoke_builder_api_key() {
    let client = create_test_client_with_api_key();

    let result = client
        .revoke_builder_api_key()
        .await
        .expect("Failed to revoke builder API key");

    println!(
        "=== Revoke Builder API Key ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}