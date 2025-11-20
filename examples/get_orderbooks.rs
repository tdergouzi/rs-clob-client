use anyhow::Result;
use dotenvy::dotenv;
use rs_clob_client::{BookParams, Chain, ClobClient, Side};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

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
    let clob_client = ClobClient::new(
        host,
        chain_id,
        None, // wallet
        None, // creds
        None, // signature_type
        None, // funder_address
        None, // geo_block_token
        false, // use_server_time
        None, // builder_config
    );

    // Token IDs
    let yes_token = "71321045679252212594626385532706912750332728571942532289631379312455583992563";
    let no_token = "52114319501245915516055106046884209969926127482827954674443846427813813222426";

    // Get multiple orderbooks
    let orderbooks = clob_client
        .get_order_books(vec![
            BookParams {
                token_id: yes_token.to_string(),
                side: Side::Buy,
            },
            BookParams {
                token_id: no_token.to_string(),
                side: Side::Buy,
            },
        ])
        .await?;

    println!("Orderbooks: {:#?}", orderbooks);

    Ok(())
}

