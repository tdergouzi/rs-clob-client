mod common;

use common::create_test_client;
use rs_clob_client::types::OrderBookParams;

#[tokio::test]
async fn test_get_orderbook() {
    let client = create_test_client();

    // YES token ID
    let yes_token = "17485324633782403667662076761548030320520944195229065834792808204789898306659";

    // Get orderbook
    let orderbook = client
        .get_order_book(yes_token)
        .await
        .expect("Failed to fetch orderbook");

    // Assertions
    assert!(
        !orderbook.bids.is_empty() || !orderbook.asks.is_empty(),
        "Orderbook should have at least bids or asks"
    );

    // println!(
    //     "Orderbook: {}",
    //     serde_json::to_string_pretty(&orderbook).unwrap()
    // );

    println!("Orderbook the last ask: {:?}", orderbook.asks[orderbook.asks.len() - 1]);
}

#[tokio::test]
async fn test_get_orderbooks() {
    let client = create_test_client();

    let params = vec![
        OrderBookParams {
            token_id: "98861221941952098410661779464520326542627371393679468645396942578853799448969".to_string(),
            side: None,
        },
        OrderBookParams {
            token_id: "1590293477094050907486207079346730658466569083582527022110944767563122184311".to_string(),
            side: None,
        },
    ];

    // Get orderbooks
    let result = client
        .get_order_books(params)
        .await
        .expect("Failed to fetch orderbooks");

    assert!(result.len() > 0, "Should have at least one orderbook");

    println!("=== Orderbooks ===");
    for (i, orderbook) in result.iter().take(3).enumerate() {
        println!("\n{}. Orderbook:", i + 1);
        println!("{}", serde_json::to_string_pretty(orderbook).unwrap());
    }
}
