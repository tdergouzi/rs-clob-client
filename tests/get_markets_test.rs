use rs_clob_client::{
    client::ClobClient,
    types::{Chain, PaginationPayload},
};
use std::env;

/// Helper function to create a test client
fn create_test_client() -> ClobClient {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get host from environment variable or use default
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");

    // Create client without authentication (public endpoint)
    ClobClient::new(
        host,
        Chain::Polygon,
        None,  // No wallet needed for public endpoints
        None,  // No API credentials needed
        None,  // No signature type
        None,  // No funder address
        None,  // No geo block token
        false, // Don't use server time
        None,  // No builder config
    )
}

#[tokio::test]
async fn test_get_markets() {
    let client = create_test_client();

    // Get first page of markets
    let response: PaginationPayload = client
        .get_markets(None)
        .await
        .expect("Failed to fetch markets");

    // Assertions
    assert!(!response.next_cursor.is_empty(), "Next cursor should not be empty");
    assert!(response.data.len() > 0, "Should have at least one market");
    assert!(response.count > 0, "Count should be greater than 0");

    // Log results for verification
    println!("=== Markets (First Page) ===");
    println!("Next Cursor: {}", response.next_cursor);
    println!("Number of markets: {}", response.data.len());
    println!("\nFirst 3 markets:");

    for (i, market) in response.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{:#?}", market);
    }
}

#[tokio::test]
async fn test_get_markets_with_cursor() {
    let client = create_test_client();

    // Get first page of markets
    let response: PaginationPayload = client
        .get_markets(None)
        .await
        .expect("Failed to fetch first page of markets");

    assert!(!response.next_cursor.is_empty(), "First page next cursor should not be empty");
    assert!(response.data.len() > 0, "First page should have at least one market");

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

    assert!(second_page.data.len() > 0, "Second page should have at least one market");

    println!("Next Cursor: {}", second_page.next_cursor);
    println!("Number of markets: {}", second_page.data.len());
    println!("\nFirst 3 markets from second page:");

    for (i, market) in second_page.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{:#?}", market);
    }
}

