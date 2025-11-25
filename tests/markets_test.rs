mod common;

use common::create_test_client;
use rs_clob_client::types::{Market, MarketParams};

#[tokio::test]
async fn test_get_markets() {
    let client = create_test_client();

    let params = MarketParams {
        limit: Some(5),
        offset: None,
        order: Some("volume1yr".to_string()),
        ascending: Some(false),
        condition_id: None,
        closed: Some(false),
    };

    // Get first page of markets
    let result: Vec<Market> = client
        .get_markets(params)
        .await
        .expect("Failed to fetch markets");

    assert!(result.len() > 0, "Should have at least one market");

    println!("=== Markets ===");
    println!("Number of markets: {}", result.len());
    println!("\nFirst 3 markets:");

    for (i, market) in result.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_market_by_id() {
    let client = create_test_client();

    let market: Market = client
        .get_market_by_id("690560")
        .await
        .expect("Failed to fetch market");

    assert!(!market.id.is_empty(), "Market ID should not be empty");

    println!("=== Market ===");
    println!("{}", serde_json::to_string_pretty(&market).unwrap());
}

#[tokio::test]
async fn test_get_market_by_slug() {
    let client = create_test_client();

    let market: Market = client
        .get_market_by_slug("will-trump-release-the-epstein-files-by-december-31")
        .await
        .expect("Failed to fetch market");

    assert!(!market.id.is_empty(), "Market ID should not be empty");

    println!("=== Market ===");
    println!("{}", serde_json::to_string_pretty(&market).unwrap());
}
