mod common;

use rs_clob_client::types::{OrderType, Side, UserMarketOrder};
use common::create_authenticated_test_client;

#[tokio::test]
async fn test_create_market_sell_order() {
    let client = create_authenticated_test_client();

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

