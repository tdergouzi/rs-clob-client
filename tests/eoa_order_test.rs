mod common;

use common::create_test_client_with_wallet;
use rs_clob_client::types::{OrderType, Side, TradeParams, UserLimitOrder, UserMarketOrder};

/// Fed decision in December 25 bps decrease yes token ID
const YES_TOKEN: &str =
    "87769991026114894163580777793845523168226980076553814689875238288185044414090";

#[tokio::test]
async fn test_create_market_buy_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create and post a market buy order for the equivalent of 5 USDC for the market price
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: YES_TOKEN.to_string(),
                amount: 5.0,
                side: Side::Buy,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: Some(OrderType::Fok), // or FAK
            },
            None,
            OrderType::Fok, // or FAK
        )
        .await
        .expect("Failed to create and post market order");

    // Assertions
    assert!(
        response.is_object(),
        "Create and post response should be a valid JSON object"
    );

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_market_sell_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: YES_TOKEN.to_string(),
                amount: 5.55555, // SHARES
                side: Side::Sell,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: None,
            },
            None,
            OrderType::Fok, // or FAK
        )
        .await
        .expect("Failed to create and post market sell order");

    // Assertions
    assert!(
        response.is_object(),
        "Create and post response should be a valid JSON object"
    );

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_limit_buy_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_limit_order(
            &UserLimitOrder {
                token_id: YES_TOKEN.to_string(),
                price: 0.80,
                size: 5.0, // SHARES
                side: Side::Buy,
                fee_rate_bps: None,
                nonce: None,
                expiration: None,
                taker: None,
            },
            None,
            OrderType::Gtc,
        )
        .await
        .expect("Failed to create and post limit order");

    // Assertions
    assert!(
        response.is_object(),
        "Create and post response should be a valid JSON object"
    );

    println!("Create and Post Response: {:#?}", response); // 0xf58d1851dbd249d6d26f60f64f30a5cfa58e80950a4a24e14398348a91f6cbf6
}

#[tokio::test]
async fn test_create_limit_sell_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_limit_order(
            &UserLimitOrder {
                token_id: YES_TOKEN.to_string(),
                price: 0.92,
                size: 5.55555, // SHARES
                side: Side::Sell,
                fee_rate_bps: None,
                nonce: None,
                expiration: None,
                taker: None,
            },
            None,
            OrderType::Gtc,
        )
        .await
        .expect("Failed to create and post limit sell order");

    // Assertions
    assert!(
        response.is_object(),
        "Create and post response should be a valid JSON object"
    );

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_get_trades() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    let params = Some(TradeParams {
        id: None,
        market: None,
        asset_id: None,
        maker_address: Some("0x73c8f452f2e628bf98853970cd586801123503fe".to_string()),
        before: None,
        after: None,
    });

    // Get trades
    let trades = client
        .get_trades(params)
        .await
        .expect("Failed to fetch order");

    // Assertions
    assert!(!trades.len() > 0, "Trades should not be empty");

    println!("Trades: {:#?}", trades);
}

#[tokio::test]
async fn test_get_open_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    let order_id = "0x9dd18101e23536c20c65decd73bc867d1a26fe6a34218d54029d74d3cdfdbf85"; // Market sell order
    let order = client
        .get_open_order(order_id)
        .await
        .expect("Failed to fetch order");

    println!("{:#?}", order);
}

#[tokio::test]
async fn test_cancel_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client
        .create_or_derive_api_key(None)
        .await
        .expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    let order_id = "0xdc034d11f56d803cd5cceb71482d92cc6aa9abea4cbf8b6b47d65cb845a80f71"; // Limit sell order
    let response = client
        .cancel_order(order_id)
        .await
        .expect("Failed to cancel order");

    println!("{:#?}", response);
}
