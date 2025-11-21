use rs_clob_client::{
    client::ClobClient,
    types::{Chain, PaginationPayload},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Get host from environment variable or use default
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");

    // Create client without authentication (public endpoint)
    let client = ClobClient::new(
        host,
        Chain::Polygon,
        None,  // No wallet needed for public endpoints
        None,  // No API credentials needed
        None,  // No signature type
        None,  // No funder address
        None,  // No geo block token
        false, // Don't use server time
        None,  // No builder config
    );

    println!("Fetching markets...\n");

    // Get first page of markets
    let response: PaginationPayload = client.get_markets(None).await?;

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
    let second_page: PaginationPayload = client.get_markets(Some(response.next_cursor)).await?;

    println!("Next Cursor: {}", second_page.next_cursor);
    println!("Number of markets: {}", second_page.data.len());
    println!("\nFirst 3 markets from second page:");

    for (i, market) in second_page.data.iter().take(3).enumerate() {
        println!("\n{}. Market:", i + 1);
        println!("{:#?}", market);
    }

    Ok(())
}
