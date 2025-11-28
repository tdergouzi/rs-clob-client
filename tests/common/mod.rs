#![allow(unused)]

use alloy_signer_local::PrivateKeySigner;
use rs_builder_signing_sdk::{BuilderApiKeyCreds, BuilderConfig};
use rs_clob_client::{
    client::ClobClient,
    types::{ApiKeyCreds, Chain},
};
use std::env;

pub fn create_test_client() -> ClobClient {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get host from environment variable or use default
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");
    let gamma_host = env::var("CLOB_GAMMA_API_URL").expect("CLOB_GAMMA_API_URL must be set");

    // Create client without authentication (public endpoint)
    ClobClient::new(
        host,
        gamma_host,
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

pub fn create_test_client_with_wallet() -> ClobClient {
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
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");
    let gamma_host = env::var("CLOB_GAMMA_API_URL").expect("CLOB_GAMMA_API_URL must be set");

    // Create CLOB client
    ClobClient::new(
        host,
        gamma_host,
        chain_id,
        Some(wallet),
        None,
        Some(0), // signature_type
        None,    // funder_address
        None,    // geo_block_token
        true,    // use_server_time
        None,    // builder_config
    )
}

pub fn create_test_client_with_api_key() -> ClobClient {
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
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");
    let gamma_host = env::var("CLOB_GAMMA_API_URL").expect("CLOB_GAMMA_API_URL must be set");

    // Create API key credentials
    let creds = ApiKeyCreds {
        key: env::var("CLOB_API_KEY").expect("CLOB_API_KEY must be set"),
        secret: env::var("CLOB_SECRET").expect("CLOB_SECRET must be set"),
        passphrase: env::var("CLOB_PASSPHRASE").expect("CLOB_PASSPHRASE must be set"),
    };

    // Create CLOB client
    ClobClient::new(
        host,
        gamma_host,
        chain_id,
        Some(wallet),
        Some(creds),
        Some(0), // signature_type
        None,    // funder_address
        None,    // geo_block_token
        true,    // use_server_time
        None,    // builder_config
    )
}

pub fn create_test_client_with_builder_api_key() -> ClobClient {
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
    let host = env::var("CLOB_API_URL").expect("CLOB_API_URL must be set");
    let gamma_host = env::var("CLOB_GAMMA_API_URL").expect("CLOB_GAMMA_API_URL must be set");

    // Create API key credentials
    let creds = ApiKeyCreds {
        key: env::var("CLOB_API_KEY").expect("CLOB_API_KEY must be set"),
        secret: env::var("CLOB_SECRET").expect("CLOB_SECRET must be set"),
        passphrase: env::var("CLOB_PASSPHRASE").expect("CLOB_PASS_PHRASE must be set"),
    };

    let builder_config = BuilderConfig::new(
        None,
        Some(BuilderApiKeyCreds {
            key: env::var("CLOB_BUILDER_API_KEY").expect("CLOB_BUILDER_API_KEY must be set"),
            secret: env::var("CLOB_BUILDER_SECRET").expect("CLOB_BUILDER_SECRET must be set"),
            passphrase: env::var("CLOB_BUILDER_PASSPHRASE").expect("CLOB_BUILDER_PASSPHRASE must be set"),
        }),
    )
    .expect("Failed to create builder config");

    // Create CLOB client
    ClobClient::new(
        host,
        gamma_host,
        chain_id,
        Some(wallet),
        Some(creds),
        Some(0),
        None,
        None,
        true,
        Some(builder_config),
    )
}
