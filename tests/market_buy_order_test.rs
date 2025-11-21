use alloy_signer_local::PrivateKeySigner;
use rs_clob_client::{
    client::ClobClient,
    types::{ApiKeyCreds, Chain, CreateOrderOptions, OrderType, Side, TickSize, UserMarketOrder},
};
use std::env;

/// Helper function to create an authenticated test client
fn create_test_client() -> ClobClient {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Parse private key from environment
    let pk = env::var("PK").expect("PK must be set");
    let wallet: PrivateKeySigner = pk.parse().expect("Invalid private key");

    // Parse chain ID
    let chain_id_str: String = env::var("CHAIN_ID").unwrap_or_else(|_| "80002".to_string());
    let chain_id: Chain = match chain_id_str.parse::<u64>().unwrap() {
        137 => Chain::Polygon,
        80002 => Chain::Amoy,
        _ => Chain::Amoy,
    };

    let address = wallet.address();
    println!("Address: {}, chainId: {}", address, chain_id_str);

    // Get API host
    let host = env::var("CLOB_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    // Create API key credentials
    let creds = ApiKeyCreds {
        key: env::var("CLOB_API_KEY").expect("CLOB_API_KEY must be set"),
        secret: env::var("CLOB_SECRET").expect("CLOB_SECRET must be set"),
        passphrase: env::var("CLOB_PASS_PHRASE").expect("CLOB_PASS_PHRASE must be set"),
    };

    // Create CLOB client
    ClobClient::new(
        host,
        chain_id,
        Some(wallet),
        Some(creds),
        None,     // signature_type
        None,     // funder_address
        None,     // geo_block_token
        false,    // use_server_time
        None,     // builder_config
    )
}

#[tokio::test]
async fn test_create_market_buy_order() {
    let client = create_test_client();

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
    let client = create_test_client();

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

