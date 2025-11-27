mod common;

use rs_clob_client::types::{CreateOrderOptions, OrderType, Side, TickSize, UserMarketOrder};
use common::create_test_client_with_wallet;

#[tokio::test]
async fn test_create_market_buy_order() {
    let client = create_test_client_with_wallet();

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
    let client = create_test_client_with_wallet();

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

