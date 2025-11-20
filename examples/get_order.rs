use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use dotenvy::dotenv;
use rs_clob_client::{ApiKeyCreds, Chain, ClobClient};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

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
    let clob_client = ClobClient::new(
        host,
        chain_id,
        Some(wallet),
        Some(creds),
        None,     // signature_type
        None,     // funder_address
        None,     // geo_block_token
        false,    // use_server_time
        None,     // builder_config
    );

    // Get order by ID
    let order = clob_client
        .get_order("0x831680cb77da95792af5a052c87c8abf9d2ae5cb21f275670bc0ff58f2823c5c")
        .await?;

    println!("{:#?}", order);

    Ok(())
}

