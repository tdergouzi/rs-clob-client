mod common;

use common::create_authenticated_test_client;

#[tokio::test]
async fn test_get_order() {
    let client = create_authenticated_test_client();

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

