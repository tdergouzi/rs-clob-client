mod common;

use common::create_test_client;

#[tokio::test]
async fn test_get_orderbook() {
    let client = create_test_client();

    // YES token ID
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";

    // Get orderbook
    let mut orderbook = client
        .get_order_book(yes_token)
        .await
        .expect("Failed to fetch orderbook");

    // Assertions
    assert!(!orderbook.bids.is_empty() || !orderbook.asks.is_empty(), 
            "Orderbook should have at least bids or asks");

    println!("Orderbook: {:#?}", orderbook);

    // Calculate and print orderbook hash
    let hash = client.get_order_book_hash(&mut orderbook);
    assert!(!hash.is_empty(), "Orderbook hash should not be empty");
    
    println!("Orderbook hash: {}", hash);
}

