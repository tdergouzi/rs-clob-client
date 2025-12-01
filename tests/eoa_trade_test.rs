mod common;

use rs_clob_client::types::{CreateOrderOptions, OrderType, Side, TickSize, UserMarketOrder, UserOrder};
use common::{create_test_client_with_wallet};

/// Fed decision in December 25 bps decrease yes token ID
const YES_TOKEN: &str = "87769991026114894163580777793845523168226980076553814689875238288185044414090";

#[tokio::test]
async fn test_get_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client.create_or_derive_api_key(None).await.expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Get order by ID
    let order = client
        .get_order("0x831680cb77da95792af5a052c87c8abf9d2ae5cb21f275670bc0ff58f2823c5c")
        .await
        .expect("Failed to fetch order");

    // Assertions
    assert!(!order.id.is_empty(), "Order ID should not be empty");
    assert!(!order.status.is_empty(), "Order status should not be empty");

    println!("{:#?}", order);
}

#[tokio::test]
async fn test_trade_market_buy_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client.create_or_derive_api_key(None).await.expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create and post a market buy order for the equivalent of 5 USDC for the market price
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: YES_TOKEN.to_string(),
                amount: 2.0,
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
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_trade_market_sell_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client.create_or_derive_api_key(None).await.expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: YES_TOKEN.to_string(),
                amount: 2.247190, // SHARES
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
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_trade_limit_buy_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client.create_or_derive_api_key(None).await.expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_order(
            &UserOrder {
                token_id: yes_token.to_string(),
                price: 0.50,
                size: 100.0, // SHARES
                side: Side::Buy,
                fee_rate_bps: None,
                nonce: None,
                expiration: None,
                taker: None,
            },
            Some(CreateOrderOptions {
                tick_size: TickSize::ZeroPointZeroOne,
                neg_risk: None,
            }),
            OrderType::Gtc,
        )
        .await
        .expect("Failed to create and post limit order");

    // Assertions
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_trade_limit_sell_order() {
    let mut client = create_test_client_with_wallet();
    let creds = client.create_or_derive_api_key(None).await.expect("Failed to create or derive API key");
    client.set_api_creds(creds);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_order(
            &UserOrder {
                token_id: yes_token.to_string(),
                price: 0.60,
                size: 110.0, // SHARES
                side: Side::Sell,
                fee_rate_bps: None,
                nonce: None,
                expiration: None,
                taker: None,
            },
            Some(CreateOrderOptions {
                tick_size: TickSize::ZeroPointZeroOne,
                neg_risk: None,
            }),
            OrderType::Gtc,
        )
        .await
        .expect("Failed to create and post limit sell order");

    // Assertions
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}