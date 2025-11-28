mod common;

use rs_clob_client::types::{CreateOrderOptions, OrderType, Side, TickSize, UserMarketOrder, UserOrder};
use common::{create_test_client_with_api_key};

#[tokio::test]
async fn test_get_order() {
    let client = create_test_client_with_api_key(0);

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
async fn test_create_market_buy_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create a YES market buy order for the equivalent of 100 USDC for the market price
    let market_buy_order = client
        .create_market_order(
            &UserMarketOrder {
                token_id: yes_token.to_string(),
                amount: 100.0, // $$$
                side: Side::Buy,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: Some(OrderType::Fok), // or FAK
            },
            None, // options
        )
        .await
        .expect("Failed to create market buy order");

    // Assertions
    assert!(market_buy_order.is_object(), "Market buy order should be a valid JSON object");

    println!("Created Market BUY Order: {:#?}", market_buy_order);

    // Send it to the server
    let response = client
        .post_order(market_buy_order, OrderType::Fok)
        .await
        .expect("Failed to post order");

    assert!(response.is_object(), "Post order response should be a valid JSON object");

    println!("Post Order Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_and_post_market_buy_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: yes_token.to_string(),
                amount: 100.0, // $$$
                side: Side::Buy,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: Some(OrderType::Fok), // or FAK
            },
            Some(CreateOrderOptions {
                tick_size: TickSize::ZeroPointZeroOne,
                neg_risk: None,
            }),
            OrderType::Fok, // or FAK
        )
        .await
        .expect("Failed to create and post market order");

    // Assertions
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_market_sell_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create a YES market sell order for the equivalent of 110 shares for the market price
    let market_sell_order = client
        .create_market_order(
            &UserMarketOrder {
                token_id: yes_token.to_string(),
                amount: 110.0, // SHARES
                side: Side::Sell,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: None,
            },
            None, // options
        )
        .await
        .expect("Failed to create market sell order");

    // Assertions
    assert!(market_sell_order.is_object(), "Market sell order should be a valid JSON object");

    println!("Created Market SELL Order: {:#?}", market_sell_order);

    // Send it to the server
    let response = client
        .post_order(market_sell_order, OrderType::Fok)
        .await
        .expect("Failed to post order");

    assert!(response.is_object(), "Post order response should be a valid JSON object");

    println!("Post Order Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_and_post_market_sell_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create the order and send it to the server in a single step
    let response = client
        .create_and_post_market_order(
            &UserMarketOrder {
                token_id: yes_token.to_string(),
                amount: 110.0, // SHARES
                side: Side::Sell,
                price: None,
                fee_rate_bps: None,
                nonce: None,
                taker: None,
                order_type: None,
            },
            Some(CreateOrderOptions {
                tick_size: TickSize::ZeroPointZeroOne,
                neg_risk: None,
            }),
            OrderType::Fok, // or FAK
        )
        .await
        .expect("Failed to create and post market sell order");

    // Assertions
    assert!(response.is_object(), "Create and post response should be a valid JSON object");

    println!("Create and Post Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_limit_buy_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create a YES limit buy order for 100 shares at price 0.50
    let limit_buy_order = client
        .create_order(
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
            None, // options
        )
        .await
        .expect("Failed to create limit buy order");

    // Assertions
    assert!(limit_buy_order.is_object(), "Limit buy order should be a valid JSON object");

    println!("Created Limit BUY Order: {:#?}", limit_buy_order);

    // Send it to the server
    let response = client
        .post_order(limit_buy_order, OrderType::Gtc)
        .await
        .expect("Failed to post order");

    assert!(response.is_object(), "Post order response should be a valid JSON object");

    println!("Post Order Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_and_post_limit_buy_order() {
    let client = create_test_client_with_api_key(0);

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
async fn test_create_limit_sell_order() {
    let client = create_test_client_with_api_key(0);

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Create a YES limit sell order for 110 shares at price 0.60
    let limit_sell_order = client
        .create_order(
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
            None, // options
        )
        .await
        .expect("Failed to create limit sell order");

    // Assertions
    assert!(limit_sell_order.is_object(), "Limit sell order should be a valid JSON object");

    println!("Created Limit SELL Order: {:#?}", limit_sell_order);

    // Send it to the server
    let response = client
        .post_order(limit_sell_order, OrderType::Gtc)
        .await
        .expect("Failed to post order");

    assert!(response.is_object(), "Post order response should be a valid JSON object");

    println!("Post Order Response: {:#?}", response);
}

#[tokio::test]
async fn test_create_and_post_limit_sell_order() {
    let client = create_test_client_with_api_key(0);

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