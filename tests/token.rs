mod common;

use common::create_test_client;

const YES_TOKEN_ID: &str =
    "98861221941952098410661779464520326542627371393679468645396942578853799448969";

#[tokio::test]
async fn test_get_tick_size() {
    let client = create_test_client();

    let result = client
        .get_tick_size(YES_TOKEN_ID)
        .await
        .expect("Failed to fetch tick size");

    println!("=== Tick Size ===\n{}", result.as_str());
}

#[tokio::test]
async fn test_get_neg_risk() {
    let client = create_test_client();

    let result = client
        .get_neg_risk(YES_TOKEN_ID)
        .await
        .expect("Failed to fetch neg risk");

    println!("=== Neg Risk ===\n{}", result);
}

#[tokio::test]
async fn test_get_fee_rate() {
    let client = create_test_client();

    let result = client
        .get_fee_rate_bps(YES_TOKEN_ID)
        .await
        .expect("Failed to fetch fee rate");

    println!("=== Fee Rate ===\n{}", result);
}
