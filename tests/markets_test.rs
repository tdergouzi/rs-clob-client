mod common;

use rs_clob_client::types::PaginationPayload;
use common::create_test_client;

#[tokio::test]
async fn test_get_markets() {
    let client = create_test_client();

    // Get first page of markets
    let response: PaginationPayload = client
        .get_markets(None)
        .await
        .expect("Failed to fetch markets");

    // Assertions
    assert!(
        !response.next_cursor.is_empty(),
        "Next cursor should not be empty"
    );
    assert!(response.data.len() > 0, "Should have at least one market");
    assert!(response.count > 0, "Count should be greater than 0");

    // Log results for verification
    println!("=== Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_market() {
    let client = create_test_client();

    // Specific condition_id to test
    let condition_id = "0x4048fed324ac27f378ce44da1b12f0d338c8340ef82962989f38eea05409baab";

    // Get market by condition_id
    let response: serde_json::Value = client
        .get_market(condition_id)
        .await
        .expect("Failed to fetch market");

    // Assertions
    assert!(!response.is_null(), "Response should not be null");

    // Verify the response contains expected fields (assuming it's a market object)
    if let Some(obj) = response.as_object() {
        assert!(!obj.is_empty(), "Market object should not be empty");
    }

    // Log results for verification
    println!("=== Market Details ===");
    println!("Condition ID: {}", condition_id);
    println!("Market data:");
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}

#[tokio::test]
async fn test_get_markets_with_cursor() {
    let client = create_test_client();

    // Get first page of markets
    let response: PaginationPayload = client
        .get_markets(None)
        .await
        .expect("Failed to fetch first page of markets");

    assert!(
        !response.next_cursor.is_empty(),
        "First page next cursor should not be empty"
    );
    assert!(
        response.data.len() > 0,
        "First page should have at least one market"
    );

    println!("=== Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{:#?}", market);
    }

    // Fetch second page using the cursor from first page
    println!("\n\n=== Fetching Second Page ===");
    let second_page: PaginationPayload = client
        .get_markets(Some(response.next_cursor))
        .await
        .expect("Failed to fetch second page of markets");

    assert!(
        second_page.data.len() > 0,
        "Second page should have at least one market"
    );

    println!("Next Cursor: {}", second_page.next_cursor);
    println!("Number of markets: {}", second_page.data.len());
    println!("\nFirst 3 markets from second page:");

    for (i, market) in second_page.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_sampling_markets() {
    let client = create_test_client();

    // Get first page of sampling markets
    let response: PaginationPayload = client
        .get_sampling_markets(None)
        .await
        .expect("Failed to fetch sampling markets");

    // Assertions
    assert!(
        !response.next_cursor.is_empty(),
        "Next cursor should not be empty"
    );
    assert!(response.data.len() > 0, "Should have at least one market");
    assert!(response.count > 0, "Count should be greater than 0");

    // Log results for verification
    println!("=== Sampling Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 sampling markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_sampling_markets_with_cursor() {
    let client = create_test_client();

    // Get first page of sampling markets
    let response: PaginationPayload = client
        .get_sampling_markets(None)
        .await
        .expect("Failed to fetch first page of sampling markets");

    assert!(
        !response.next_cursor.is_empty(),
        "First page next cursor should not be empty"
    );
    assert!(
        response.data.len() > 0,
        "First page should have at least one market"
    );

    println!("=== Sampling Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 sampling markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{:#?}", market);
    }

    // Fetch second page using the cursor from first page
    println!("\n\n=== Fetching Second Page ===");
    let second_page: PaginationPayload = client
        .get_sampling_markets(Some(response.next_cursor))
        .await
        .expect("Failed to fetch second page of sampling markets");

    assert!(
        second_page.data.len() > 0,
        "Second page should have at least one market"
    );

    println!("Next Cursor: {}", second_page.next_cursor);
    println!("Number of markets: {}", second_page.data.len());
    println!("\nFirst 3 markets from second page:");

    for (i, market) in second_page.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_simplified_markets() {
    let client = create_test_client();

    // Get first page of simplified markets
    let response: PaginationPayload = client
        .get_simplified_markets(None)
        .await
        .expect("Failed to fetch simplified markets");

    // Assertions
    assert!(
        !response.next_cursor.is_empty(),
        "Next cursor should not be empty"
    );
    assert!(response.data.len() > 0, "Should have at least one market");
    assert!(response.count > 0, "Count should be greater than 0");

    // Log results for verification
    println!("=== Simplified Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 simplified markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}

#[tokio::test]
async fn test_get_sampling_simplified_markets() {
    let client = create_test_client();

    // Get first page of sampling simplified markets
    let response: PaginationPayload = client
        .get_sampling_simplified_markets(None)
        .await
        .expect("Failed to fetch sampling simplified markets");

    // Assertions
    assert!(
        !response.next_cursor.is_empty(),
        "Next cursor should not be empty"
    );
    assert!(response.data.len() > 0, "Should have at least one market");
    assert!(response.count > 0, "Count should be greater than 0");

    // Log results for verification
    println!("=== Sampling Simplified Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 sampling simplified markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{}", serde_json::to_string_pretty(market).unwrap());
    }
}
