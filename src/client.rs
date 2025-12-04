use crate::http::HttpClient;
use crate::order_builder::OrderBuilder;
use crate::types::*;
use alloy_signer_local::PrivateKeySigner;
use rs_builder_signing_sdk::BuilderConfig;
use std::sync::RwLock;
use std::collections::HashMap;

mod auth;
mod public;
mod trading;
// mod rewards; // No tests for rewards yet

/// Main CLOB client for interacting with Polymarket's Central Limit Order Book
pub struct ClobClient {
    /// Base URL for the CLOB API
    #[allow(unused)]
    pub(crate) host: String,

    /// Blockchain network (Polygon or Amoy)
    pub(crate) chain_id: Chain,

    /// HTTP client for making requests
    pub(crate) http_client: HttpClient,

    /// HTTP client for making requests to the Gamma API
    pub(crate) gamma_api_client: HttpClient,

    /// Wallet for L1 authentication (optional)
    pub(crate) wallet: Option<PrivateKeySigner>,

    /// API credentials for L2 authentication (optional)
    pub(crate) creds: Option<ApiKeyCreds>,

    /// Order builder for creating and signing orders (requires a wallet)
    pub(crate) order_builder: Option<OrderBuilder>,

    /// Signature type for orders (0 = EOA, 1 = Poly Proxy, 2 = EIP-1271)
    #[allow(unused)]
    pub(crate) signature_type: u8,

    /// Cached tick sizes for tokens (thread-safe)
    pub(crate) tick_sizes: RwLock<HashMap<String, TickSize>>,

    /// Cached negative risk flags for tokens (thread-safe)
    pub(crate) neg_risk: RwLock<HashMap<String, bool>>,

    /// Cached fee rates for tokens (thread-safe)
    pub(crate) fee_rates: RwLock<HashMap<String, u32>>,

    /// Whether to use server time for signatures
    pub(crate) use_server_time: bool,

    /// Builder configuration for builder API authentication (optional)
    pub(crate) builder_config: Option<BuilderConfig>,
}

impl ClobClient {
    /// Creates a new ClobClient instance (matches TypeScript constructor)
    ///
    /// # Arguments
    ///
    /// * `host` - Base URL for the CLOB API (e.g., "https://clob.polymarket.com")
    /// * `chain_id` - Blockchain network (Chain::Polygon or Chain::Amoy)
    /// * `wallet` - Optional wallet for L1 authentication and signing orders
    /// * `creds` - Optional API credentials for L2 authentication
    /// * `signature_type` - Signature type for orders (0 = EOA, 1 = Poly Proxy, 2 = EIP-1271)
    /// * `funder_address` - Optional funder address for smart contract wallets
    /// * `geo_block_token` - Optional geo-block token
    /// * `use_server_time` - Whether to use server time for signatures
    /// * `builder_config` - Optional builder configuration for builder API authentication
    pub fn new(
        host: String,
        gamma_host: String,
        chain_id: Chain,
        wallet: Option<PrivateKeySigner>,
        creds: Option<ApiKeyCreds>,
        signature_type: Option<u8>,
        funder_address: Option<String>,
        geo_block_token: Option<String>,
        use_server_time: bool,
        builder_config: Option<BuilderConfig>,
    ) -> Self {
        let host = if host.ends_with('/') {
            host[..host.len() - 1].to_string()
        } else {
            host
        };

        let gamma_host = if gamma_host.ends_with('/') {
            gamma_host[..gamma_host.len() - 1].to_string()
        } else {
            gamma_host
        };

        let gamma_api_client = HttpClient::new(gamma_host);

        // Default signature type to EOA (0) if not provided
        let sig_type = signature_type.unwrap_or(0);

        // Convert signature type to SignatureType enum
        let sig_type_enum = match sig_type {
            0 => rs_order_utils::SignatureType::Eoa,
            1 => rs_order_utils::SignatureType::PolyProxy,
            2 => rs_order_utils::SignatureType::PolyGnosisSafe,
            _ => rs_order_utils::SignatureType::Eoa,
        };

        // Parse funder address if provided
        let funder_addr = funder_address.as_ref().and_then(|addr| {
            use std::str::FromStr;
            alloy_primitives::Address::from_str(addr).ok()
        });

        // Initialize OrderBuilder only if wallet is provided
        let order_builder = wallet.as_ref().map(|w| {
            OrderBuilder::new(
                w.clone(),
                chain_id,
                Some(sig_type_enum),
                funder_addr,
                None, // get_signer
            )
        });

        // Create HTTP client and set geo_block_token if provided
        let http_client = if let Some(token) = &geo_block_token {
            HttpClient::new(host.clone()).with_geo_block_token(token.clone())
        } else {
            HttpClient::new(host.clone())
        };

        Self {
            http_client,
            gamma_api_client,
            host,
            chain_id,
            wallet,
            creds,
            order_builder,
            signature_type: sig_type,
            tick_sizes: RwLock::new(HashMap::new()),
            neg_risk: RwLock::new(HashMap::new()),
            fee_rates: RwLock::new(HashMap::new()),
            use_server_time,
            builder_config,
        }
    }

    pub fn set_api_creds(&mut self, creds: ApiKeyCreds) {
        self.creds = Some(creds);
    }
}
