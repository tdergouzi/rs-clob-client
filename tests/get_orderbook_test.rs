use rs_clob_client::{client::ClobClient, types::Chain};
use std::env;

/// Helper function to create a test client
fn create_test_client() -> ClobClient {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API host
    let host = env::var("CLOB_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    // Parse chain ID
    let chain_id_str = env::var("CHAIN_ID").unwrap_or_else(|_| "80002".to_string());
    let chain_id: Chain = match chain_id_str.parse::<u64>().unwrap() {
        137 => Chain::Polygon,
        80002 => Chain::Amoy,
        _ => Chain::Amoy,
    };

    // Create CLOB client (no authentication needed for public endpoints)
    ClobClient::new(
        host,
        chain_id,
        None, // wallet
        None, // creds
        None, // signature_type
        None, // funder_address
        None, // geo_block_token
        false, // use_server_time
        None, // builder_config
    )
}

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

