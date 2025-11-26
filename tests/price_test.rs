mod common;

use common::create_test_client;
use rs_clob_client::types::{
    OrderBookParams, PriceHistoryInterval, PriceHistoryParams, PriceParams, Side, SpreadsParams, LastTradePriceParams,
};

const YES_TOKEN_ID: &str =
    "98861221941952098410661779464520326542627371393679468645396942578853799448969";
const NO_TOKEN_ID: &str =
    "1590293477094050907486207079346730658466569083582527022110944767563122184311";

#[tokio::test]
async fn test_get_price() {
    let client = create_test_client();

    let params = PriceParams {
        token_id: YES_TOKEN_ID.to_string(),
        side: Side::Buy,
    };

    let price = client
        .get_price(params)
        .await
        .expect("Failed to fetch price");

    assert!(!price.price.is_empty(), "Price should not be empty");

    println!(
        "=== Price ===\n{}",
        serde_json::to_string_pretty(&price).unwrap()
    );
}

#[tokio::test]
async fn test_get_prices() {
    let client = create_test_client();

    let params = vec![
        PriceParams {
            token_id: YES_TOKEN_ID.to_string(),
            side: Side::Buy,
        },
        PriceParams {
            token_id: NO_TOKEN_ID.to_string(),
            side: Side::Buy,
        },
    ];

    let prices = client
        .get_prices(params)
        .await
        .expect("Failed to fetch prices");

    // Response is a map: { token_id: { side: price } }
    assert!(!prices.is_null(), "Prices should not be null");
    assert!(prices.is_object(), "Prices should be an object");

    println!(
        "=== Prices ===\n{}",
        serde_json::to_string_pretty(&prices).unwrap()
    );
}

#[tokio::test]
async fn test_get_midpoint() {
    let client = create_test_client();

    let midpoint = client
        .get_midpoint(YES_TOKEN_ID)
        .await
        .expect("Failed to fetch midpoint");

    assert!(!midpoint.mid.is_empty(), "Midpoint should not be empty");

    println!(
        "=== Midpoint ===\n{}",
        serde_json::to_string_pretty(&midpoint).unwrap()
    );
}

#[tokio::test]
async fn test_get_midpoints() {
    let client = create_test_client();

    let params = vec![
        OrderBookParams {
            token_id: YES_TOKEN_ID.to_string(),
            side: None,
        },
        OrderBookParams {
            token_id: NO_TOKEN_ID.to_string(),
            side: None,
        },
    ];

    let midpoints = client
        .get_midpoints(params)
        .await
        .expect("Failed to fetch midpoints");

    assert!(!midpoints.is_null(), "Midpoints should not be null");

    println!(
        "=== Midpoints ===\n{}",
        serde_json::to_string_pretty(&midpoints).unwrap()
    );
}

#[tokio::test]
async fn test_get_prices_history() {
    let client = create_test_client();

    let params = PriceHistoryParams {
        token_id: YES_TOKEN_ID.to_string(),
        fidelity: 60,
        interval: Some(PriceHistoryInterval::OneDay),
        ..Default::default()
    };

    let history = client
        .get_prices_history(params)
        .await
        .expect("Failed to fetch price history");

    assert!(
        !history.history.is_empty(),
        "Price history should not be empty"
    );

    println!("=== Price History ({} items) ===", history.history.len());
    // Print first 5 items
    for (i, item) in history.history.iter().take(5).enumerate() {
        println!("{}. timestamp: {}, price: {}", i + 1, item.t, item.p);
    }
}

#[tokio::test]
async fn test_get_spreads() {
    let client = create_test_client();

    let params = vec![
        SpreadsParams {
            token_id: YES_TOKEN_ID.to_string(),
            side: None,
        },
        SpreadsParams {
            token_id: NO_TOKEN_ID.to_string(),
            side: Some(Side::Buy),
        },
    ];

    let spreads = client
        .get_spreads(params)
        .await
        .expect("Failed to fetch spreads");

    // Response is a map: { token_id: { side: price } }
    assert!(!spreads.is_null(), "Spreads should not be null");
    assert!(spreads.is_object(), "Spreads should be an object");

    println!(
        "=== Spreads ===\n{}",
        serde_json::to_string_pretty(&spreads).unwrap()
    );
}

#[tokio::test]
async fn test_get_last_trades_price() {
    let client = create_test_client();

    let result = client
        .get_last_trade_price(YES_TOKEN_ID)
        .await
        .expect("Failed to fetch price");

    assert!(!result.is_null(), "Result should not be null");

    println!(
        "=== Last Trade Price ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}

#[tokio::test]
async fn test_get_last_trades_prices() {
    let client = create_test_client();

    let params = vec![
        LastTradePriceParams { token_id: YES_TOKEN_ID.to_string() },
        LastTradePriceParams { token_id: NO_TOKEN_ID.to_string() },
    ];

    let result = client
        .get_last_trades_prices(params)
        .await
        .expect("Failed to fetch prices");

    assert!(!result.is_null(), "Result should not be null");

    println!(
        "=== Last Trades Prices ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}