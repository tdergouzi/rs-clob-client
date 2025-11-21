use crate::constants::{END_CURSOR, INITIAL_CURSOR};
use crate::endpoints::endpoints;
use crate::errors::{ClobError, ClobResult};
use crate::headers::{create_l1_headers, create_l2_headers, inject_builder_headers};
use crate::http::HttpClient;
use crate::order_builder::{calculate_buy_market_price, calculate_sell_market_price, OrderBuilder};
use crate::types::*;
use alloy_signer_local::PrivateKeySigner;
use rs_builder_signing_sdk::{BuilderConfig, BuilderHeaderPayload};
use rs_order_utils::SignedOrder;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

/// Main CLOB client for interacting with Polymarket's Central Limit Order Book
pub struct ClobClient {
    /// Base URL for the CLOB API
    host: String,

    /// Blockchain network (Polygon or Amoy)
    chain_id: Chain,

    /// HTTP client for making requests
    http_client: HttpClient,

    /// Wallet for L1 authentication (optional)
    wallet: Option<PrivateKeySigner>,

    /// API credentials for L2 authentication (optional)
    creds: Option<ApiKeyCreds>,

    /// Order builder for creating and signing orders (requires a wallet)
    order_builder: Option<OrderBuilder>,

    /// Signature type for orders (0 = EOA, 1 = Poly Proxy, 2 = EIP-1271)
    signature_type: u8,

    /// Cached tick sizes for tokens (uses interior mutability)
    tick_sizes: RefCell<HashMap<String, TickSize>>,

    /// Cached negative risk flags for tokens (uses interior mutability)
    neg_risk: RefCell<HashMap<String, bool>>,

    /// Cached fee rates for tokens (uses interior mutability)
    fee_rates: RefCell<HashMap<String, u32>>,

    /// Whether to use server time for signatures
    use_server_time: bool,

    /// Builder configuration for builder API authentication (optional)
    builder_config: Option<BuilderConfig>,
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
            host,
            chain_id,
            wallet,
            creds,
            order_builder,
            signature_type: sig_type,
            tick_sizes: RefCell::new(HashMap::new()),
            neg_risk: RefCell::new(HashMap::new()),
            fee_rates: RefCell::new(HashMap::new()),
            use_server_time,
            builder_config,
        }
    }

    // ===================================
    // Public Endpoints (No Auth Required)
    // ===================================

    /// Gets server status
    pub async fn get_ok(&self) -> ClobResult<serde_json::Value> {
        self.http_client.get("/", None, None).await
    }

    /// Gets server time
    pub async fn get_server_time(&self) -> ClobResult<u64> {
        self.http_client.get(endpoints::TIME, None, None).await
    }

    /// Gets sampling simplified markets with pagination
    pub async fn get_sampling_simplified_markets(
        &self,
        next_cursor: Option<String>,
    ) -> ClobResult<PaginationPayload> {
        let cursor = next_cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string());

        let mut params = HashMap::new();
        params.insert("next_cursor".to_string(), cursor);

        self.http_client.get(endpoints::GET_SAMPLING_SIMPLIFIED_MARKETS, None, Some(params)).await
    }

    /// Gets sampling markets with pagination
    pub async fn get_sampling_markets(
        &self,
        next_cursor: Option<String>,
    ) -> ClobResult<PaginationPayload> {
        let cursor = next_cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string());

        let mut params = HashMap::new();
        params.insert("next_cursor".to_string(), cursor);

        self.http_client.get(endpoints::GET_SAMPLING_MARKETS, None, Some(params)).await
    }

    /// Gets simplified markets with pagination
    pub async fn get_simplified_markets(
        &self,
        next_cursor: Option<String>,
    ) -> ClobResult<PaginationPayload> {
        let cursor = next_cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string());

        let mut params = HashMap::new();
        params.insert("next_cursor".to_string(), cursor);

        self.http_client.get(endpoints::GET_SIMPLIFIED_MARKETS, None, Some(params)).await
    }

    /// Gets all markets with pagination
    pub async fn get_markets(&self, next_cursor: Option<String>) -> ClobResult<PaginationPayload> {
        let cursor = next_cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string());
        let endpoint = endpoints::GET_MARKETS;

        let mut params = HashMap::new();
        params.insert("next_cursor".to_string(), cursor);

        self.http_client.get(endpoint, None, Some(params)).await
    }

    /// Gets a specific market by condition ID
    pub async fn get_market(&self, condition_id: &str) -> ClobResult<serde_json::Value> {
        let endpoint = format!("{}{}", endpoints::GET_MARKET, condition_id);
        self.http_client.get(&endpoint, None, None).await
    }

    /// Gets orderbook for a token
    pub async fn get_order_book(&self, token_id: &str) -> ClobResult<OrderBookSummary> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        self.http_client.get(endpoints::GET_ORDER_BOOK, None, Some(params)).await
    }

    /// Gets multiple orderbooks
    pub async fn get_order_books(
        &self,
        params: Vec<BookParams>,
    ) -> ClobResult<Vec<OrderBookSummary>> {
        self.http_client
            .post(endpoints::GET_ORDER_BOOKS, None, Some(params), None)
            .await
    }

    /// Gets tick size for a token (with caching)
    pub async fn get_tick_size(&self, token_id: &str) -> ClobResult<TickSize> {
        // Check cache first
        if let Some(tick_size) = self.tick_sizes.borrow().get(token_id) {
            return Ok(*tick_size);
        }

        // Fetch from API
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        #[derive(Deserialize)]
        struct TickSizeResponse {
            minimum_tick_size: String,
        }

        let response: TickSizeResponse =
            self.http_client.get(endpoints::GET_TICK_SIZE, None, Some(params)).await?;
        let tick_size =
            crate::utilities::parse_tick_size(&response.minimum_tick_size).ok_or_else(|| {
                ClobError::Other(format!("Invalid tick size: {}", response.minimum_tick_size))
            })?;

        // Cache the result
        self.tick_sizes
            .borrow_mut()
            .insert(token_id.to_string(), tick_size);

        Ok(tick_size)
    }

    /// Gets negative risk flag for a token (with caching)
    pub async fn get_neg_risk(&self, token_id: &str) -> ClobResult<bool> {
        // Check cache first
        if let Some(&neg_risk) = self.neg_risk.borrow().get(token_id) {
            return Ok(neg_risk);
        }

        // Fetch from API
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        #[derive(Deserialize)]
        struct NegRiskResponse {
            neg_risk: bool,
        }

        let response: NegRiskResponse = self.http_client.get(endpoints::GET_NEG_RISK, None, Some(params)).await?;

        // Cache the result
        self.neg_risk
            .borrow_mut()
            .insert(token_id.to_string(), response.neg_risk);

        Ok(response.neg_risk)
    }

    /// Gets fee rate in basis points for a token (with caching)
    pub async fn get_fee_rate_bps(&self, token_id: &str) -> ClobResult<u32> {
        // Check cache first
        if let Some(&fee_rate) = self.fee_rates.borrow().get(token_id) {
            return Ok(fee_rate);
        }

        // Fetch from API
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        #[derive(Deserialize)]
        struct FeeRateResponse {
            #[serde(rename = "makerBaseFeeRateBps")]
            maker_base_fee_rate_bps: u32,
        }

        let response: FeeRateResponse = self.http_client.get(endpoints::GET_FEE_RATE, None, Some(params)).await?;

        // Cache the result
        self.fee_rates
            .borrow_mut()
            .insert(token_id.to_string(), response.maker_base_fee_rate_bps);

        Ok(response.maker_base_fee_rate_bps)
    }

    /// Calculates the hash for the given orderbook
    /// This modifies the orderbook by setting its hash field
    pub fn get_order_book_hash(&self, orderbook: &mut OrderBookSummary) -> String {
        crate::utilities::generate_orderbook_summary_hash(orderbook)
    }

    /// Gets midpoint price for a token
    pub async fn get_midpoint(&self, token_id: &str) -> ClobResult<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        self.http_client.get(endpoints::GET_MIDPOINT, None, Some(params)).await
    }

    /// Gets multiple midpoints
    pub async fn get_midpoints(&self, params: Vec<BookParams>) -> ClobResult<serde_json::Value> {
        self.http_client
            .post(endpoints::GET_MIDPOINTS, None, Some(params), None)
            .await
    }

    /// Gets price for a token and side
    pub async fn get_price(&self, token_id: &str, side: Side) -> ClobResult<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());
        params.insert("side".to_string(), side.to_uppercase());

        self.http_client.get(endpoints::GET_PRICE, None, Some(params)).await
    }

    /// Gets multiple prices
    pub async fn get_prices(&self, params: Vec<BookParams>) -> ClobResult<serde_json::Value> {
        self.http_client
            .post(endpoints::GET_PRICES, None, Some(params), None)
            .await
    }

    /// Gets spread for a token
    pub async fn get_spread(&self, token_id: &str) -> ClobResult<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        self.http_client.get(endpoints::GET_SPREAD, None, Some(params)).await
    }

    /// Gets multiple spreads
    pub async fn get_spreads(&self, params: Vec<BookParams>) -> ClobResult<serde_json::Value> {
        self.http_client
            .post(endpoints::GET_SPREADS, None, Some(params), None)
            .await
    }

    /// Gets last trade price for a token
    pub async fn get_last_trade_price(&self, token_id: &str) -> ClobResult<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        self.http_client.get(endpoints::GET_LAST_TRADE_PRICE, None, Some(params)).await
    }

    /// Gets last trade prices for multiple tokens
    pub async fn get_last_trades_prices(
        &self,
        params: Vec<BookParams>,
    ) -> ClobResult<serde_json::Value> {
        self.http_client
            .post(endpoints::GET_LAST_TRADES_PRICES, None, Some(params), None)
            .await
    }

    /// Gets historical prices
    pub async fn get_prices_history(
        &self,
        params: PriceHistoryFilterParams,
    ) -> ClobResult<Vec<MarketPrice>> {
        let mut query_params = HashMap::new();

        if let Some(market) = params.market {
            query_params.insert("market".to_string(), market);
        }
        if let Some(start) = params.start_ts {
            query_params.insert("startTs".to_string(), start.to_string());
        }
        if let Some(end) = params.end_ts {
            query_params.insert("endTs".to_string(), end.to_string());
        }
        if let Some(fidelity) = params.fidelity {
            query_params.insert("fidelity".to_string(), fidelity.to_string());
        }

        self.http_client
            .get(endpoints::GET_PRICES_HISTORY, None, Some(query_params))
            .await
    }

    // ===================================
    // L1 Auth Methods (API Key Management)
    // ===================================

    /// Creates a new API key using L1 authentication
    pub async fn create_api_key(&self, nonce: Option<u64>) -> ClobResult<ApiKeyCreds> {
        self.can_l1_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;

        // Get timestamp if server time is enabled
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        // Create L1 headers
        let headers = create_l1_headers(wallet, self.chain_id.chain_id(), nonce, timestamp)
            .await?
            .to_headers();

        // Make request
        let response: ApiKeyRaw = self
            .http_client
            .post(endpoints::CREATE_API_KEY, Some(headers), None::<()>, None)
            .await?;

        Ok(response.into())
    }

    /// Derives an existing API key using L1 authentication
    pub async fn derive_api_key(&self, nonce: Option<u64>) -> ClobResult<ApiKeyCreds> {
        self.can_l1_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;

        // Get timestamp if server time is enabled
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        // Create L1 headers
        let headers = create_l1_headers(wallet, self.chain_id.chain_id(), nonce, timestamp)
            .await?
            .to_headers();

        // Make request
        let response: ApiKeyRaw = self.http_client.get(endpoints::DERIVE_API_KEY, Some(headers), None).await?;

        Ok(response.into())
    }

    /// Creates or derives an API key (creates if doesn't exist, derives otherwise)
    pub async fn create_or_derive_api_key(&self, nonce: Option<u64>) -> ClobResult<ApiKeyCreds> {
        // Try to derive first
        match self.derive_api_key(nonce).await {
            Ok(creds) => Ok(creds),
            Err(_) => {
                // If derive fails, create new
                self.create_api_key(nonce).await
            }
        }
    }

    // ===================================
    // L2 Auth Methods (API Key Operations)
    // ===================================

    /// Gets all API keys for the user
    pub async fn get_api_keys(&self) -> ClobResult<ApiKeysResponse> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_API_KEYS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(endpoint_path, Some(headers), None).await
    }

    /// Gets closed-only mode status (checks if account is restricted)
    pub async fn get_closed_only_mode(&self) -> ClobResult<BanStatus> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::CLOSED_ONLY;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(endpoint_path, Some(headers), None).await
    }

    /// Deletes the current API key
    pub async fn delete_api_key(&self) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::DELETE_API_KEY;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "DELETE", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client
            .delete(endpoint_path, Some(headers), None::<()>, None)
            .await
    }

    // ===================================
    // L2 Auth Methods (Order Queries)
    // ===================================

    /// Gets an order by ID
    pub async fn get_order(&self, order_id: &str) -> ClobResult<OpenOrder> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = format!("{}{}", endpoints::GET_ORDER, order_id);
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", &endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(&endpoint_path, Some(headers), None).await
    }

    // ===================================
    // L2 Auth Methods (Trade History)
    // ===================================

    /// Gets all trades with automatic pagination
    pub async fn get_trades(&self, params: Option<TradeParams>) -> ClobResult<Vec<Trade>> {
        self.can_l2_auth()?;

        let mut results = Vec::new();
        let mut next_cursor = INITIAL_CURSOR.to_string();

        while next_cursor != END_CURSOR {
            let response = self
                .get_trades_paginated(params.clone(), Some(next_cursor.clone()))
                .await?;
            next_cursor = response.next_cursor;
            results.extend(response.data);
        }

        Ok(results)
    }

    /// Gets trades with pagination support
    pub async fn get_trades_paginated(
        &self,
        params: Option<TradeParams>,
        cursor: Option<String>,
    ) -> ClobResult<TradesPaginatedResponse> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_TRADES;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();

        // Add cursor
        query_params.insert(
            "next_cursor".to_string(),
            cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string()),
        );

        // Add user params
        if let Some(p) = params {
            if let Some(id) = p.id {
                query_params.insert("id".to_string(), id);
            }
            if let Some(market) = p.market {
                query_params.insert("market".to_string(), market);
            }
            if let Some(asset_id) = p.asset_id {
                query_params.insert("asset_id".to_string(), asset_id);
            }
            if let Some(maker) = p.maker_address {
                query_params.insert("maker_address".to_string(), maker);
            }
            if let Some(before) = p.before {
                query_params.insert("before".to_string(), before.to_string());
            }
            if let Some(after) = p.after {
                query_params.insert("after".to_string(), after.to_string());
            }
        }

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    // ===================================
    // Builder Auth Methods (Trades)
    // ===================================

    /// Gets builder trades with pagination
    pub async fn get_builder_trades(
        &self,
        params: Option<TradeParams>,
        cursor: Option<String>,
    ) -> ClobResult<BuilderTradesResponse> {
        self.must_builder_auth()?;

        let endpoint_path = endpoints::GET_BUILDER_TRADES;

        // Get builder headers (already a HashMap)
        let headers = self
            ._get_builder_headers("GET", endpoint_path, None)
            .await?;

        let mut query_params = HashMap::new();

        // Add cursor
        query_params.insert(
            "next_cursor".to_string(),
            cursor.unwrap_or_else(|| INITIAL_CURSOR.to_string()),
        );

        // Add user params
        if let Some(p) = params {
            if let Some(id) = p.id {
                query_params.insert("id".to_string(), id);
            }
            if let Some(market) = p.market {
                query_params.insert("market".to_string(), market);
            }
            if let Some(asset_id) = p.asset_id {
                query_params.insert("asset_id".to_string(), asset_id);
            }
        }

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    // ===================================
    // L2 Auth Methods (Notifications)
    // ===================================

    /// Gets user notifications
    pub async fn get_notifications(&self) -> ClobResult<Vec<Notification>> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_NOTIFICATIONS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(endpoint_path, Some(headers), None).await
    }

    /// Marks notifications as read
    pub async fn drop_notifications(&self, params: DropNotificationParams) -> ClobResult<()> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::DROP_NOTIFICATIONS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "DELETE", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();

        if !params.ids.is_empty() {
            query_params.insert("ids".to_string(), params.ids.join(","));
        }

        let _: serde_json::Value = self
            .http_client
            .delete(endpoint_path, Some(headers), None::<()>, Some(query_params))
            .await?;

        Ok(())
    }

    // ===================================
    // L2 Auth Methods (Balance/Allowance)
    // ===================================

    /// Gets balance and allowance for a token
    pub async fn get_balance_allowance(
        &self,
        params: BalanceAllowanceParams,
    ) -> ClobResult<BalanceAllowanceResponse> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_BALANCE_ALLOWANCE;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();
        let asset_type_str = match params.asset_type {
            AssetType::Collateral => "COLLATERAL",
            AssetType::Conditional => "CONDITIONAL",
        };
        query_params.insert("asset_type".to_string(), asset_type_str.to_string());

        if let Some(token_id) = params.token_id {
            query_params.insert("token_id".to_string(), token_id);
        }

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    /// Updates balance allowance cache
    pub async fn update_balance_allowance(&self, params: BalanceAllowanceParams) -> ClobResult<()> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::UPDATE_BALANCE_ALLOWANCE;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "POST", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();
        let asset_type_str = match params.asset_type {
            AssetType::Collateral => "COLLATERAL",
            AssetType::Conditional => "CONDITIONAL",
        };
        query_params.insert("asset_type".to_string(), asset_type_str.to_string());

        if let Some(token_id) = params.token_id {
            query_params.insert("token_id".to_string(), token_id);
        }

        let _: serde_json::Value = self
            .http_client
            .post(endpoint_path, Some(headers), None::<()>, Some(query_params))
            .await?;

        Ok(())
    }

    // ===================================
    // L1 Auth Methods (Order Creation)
    // ===================================

    /// Creates a signed limit order
    ///
    /// # Arguments
    ///
    /// * `user_order` - Order parameters (token_id, price, size, side, etc.)
    /// * `options` - Optional CreateOrderOptions (tick_size, neg_risk)
    ///
    /// # Returns
    ///
    /// A JSON representation of the signed order ready for posting
    pub async fn create_order(
        &self,
        user_order: &UserOrder,
        options: Option<CreateOrderOptions>,
    ) -> ClobResult<serde_json::Value> {
        self.can_l1_auth()?;

        let token_id = &user_order.token_id;

        // Resolve tick size
        let tick_size = if let Some(opts) = &options {
            opts.tick_size
        } else {
            self.get_tick_size(token_id).await?
        };

        // Resolve fee rate
        let fee_rate_bps = self
            ._resolve_fee_rate_bps(token_id, user_order.fee_rate_bps)
            .await?;

        // Resolve neg_risk
        let neg_risk = if let Some(opts) = &options {
            opts.neg_risk.unwrap_or_else(|| false)
        } else {
            self.get_neg_risk(token_id).await?
        };

        let create_options = CreateOrderOptions {
            tick_size,
            neg_risk: Some(neg_risk),
        };

        let mut order = user_order.clone();
        order.fee_rate_bps = Some(fee_rate_bps);

        let order_builder = self
            .order_builder
            .as_ref()
            .ok_or(ClobError::L1AuthUnavailable)?;

        let signed_order = order_builder.build_order(&order, &create_options).await?;
        self.signed_order_to_json(signed_order)
    }

    /// Creates a signed market order
    ///
    /// # Arguments
    ///
    /// * `user_market_order` - Market order parameters (token_id, amount, side, etc.)
    /// * `options` - Optional CreateOrderOptions (tick_size, neg_risk)
    ///
    /// # Returns
    ///
    /// A JSON representation of the signed order ready for posting
    pub async fn create_market_order(
        &self,
        user_market_order: &UserMarketOrder,
        options: Option<CreateOrderOptions>,
    ) -> ClobResult<serde_json::Value> {
        self.can_l1_auth()?;

        let token_id = &user_market_order.token_id;

        // Resolve tick size
        let tick_size = if let Some(opts) = &options {
            opts.tick_size
        } else {
            self.get_tick_size(token_id).await?
        };

        // Resolve fee rate
        let fee_rate_bps = self
            ._resolve_fee_rate_bps(token_id, user_market_order.fee_rate_bps)
            .await?;

        // Resolve neg_risk
        let neg_risk = if let Some(opts) = &options {
            opts.neg_risk.unwrap_or(false)
        } else {
            self.get_neg_risk(token_id).await?
        };

        let create_options = CreateOrderOptions {
            tick_size,
            neg_risk: Some(neg_risk),
        };

        let mut order = user_market_order.clone();
        order.fee_rate_bps = Some(fee_rate_bps);

        // Calculate market price if not provided
        if order.price.is_none() {
            let price = self
                .calculate_market_price(
                    token_id,
                    order.side,
                    order.amount,
                    order.order_type.unwrap_or(OrderType::Fok),
                )
                .await?;
            order.price = Some(price);
        }

        let order_builder = self
            .order_builder
            .as_ref()
            .ok_or(ClobError::L1AuthUnavailable)?;

        let signed_order = order_builder
            .build_market_order(&order, &create_options)
            .await?;
        self.signed_order_to_json(signed_order)
    }

    // ===================================
    // L2 Auth Methods (Order Operations)
    // ===================================

    /// Creates and posts a limit order in one call
    ///
    /// # Arguments
    ///
    /// * `user_order` - Order parameters
    /// * `options` - Optional CreateOrderOptions
    /// * `order_type` - GTC, FOK, FAK, or GTD
    ///
    /// # Returns
    ///
    /// API response with order status
    pub async fn create_and_post_order(
        &self,
        user_order: &UserOrder,
        options: Option<CreateOrderOptions>,
        order_type: OrderType,
    ) -> ClobResult<serde_json::Value> {
        let order = self.create_order(user_order, options).await?;
        self.post_order(order, order_type).await
    }

    /// Creates and posts a market order in one call
    ///
    /// # Arguments
    ///
    /// * `user_market_order` - Market order parameters
    /// * `options` - Optional CreateOrderOptions
    /// * `order_type` - Typically FOK or FAK
    ///
    /// # Returns
    ///
    /// API response with order status
    pub async fn create_and_post_market_order(
        &self,
        user_market_order: &UserMarketOrder,
        options: Option<CreateOrderOptions>,
        order_type: OrderType,
    ) -> ClobResult<serde_json::Value> {
        let order = self.create_market_order(user_market_order, options).await?;
        self.post_order(order, order_type).await
    }

    /// Gets open orders for the user
    pub async fn get_open_orders(
        &self,
        params: Option<OpenOrderParams>,
    ) -> ClobResult<OpenOrdersResponse> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_OPEN_ORDERS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();

        if let Some(p) = params {
            if let Some(id) = p.id {
                query_params.insert("id".to_string(), id);
            }
            if let Some(market) = p.market {
                query_params.insert("market".to_string(), market);
            }
            if let Some(asset_id) = p.asset_id {
                query_params.insert("asset_id".to_string(), asset_id);
            }
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.http_client.get(endpoint_path, Some(headers), params).await
    }

    /// Posts an order to the exchange
    pub async fn post_order(
        &self,
        order: serde_json::Value,
        order_type: OrderType,
    ) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        // Prepare order payload
        let order_payload = self.order_to_json(order, order_type)?;
        let body = serde_json::to_string(&order_payload)?;

        // Create L2 headers with body
        let endpoint_path = endpoints::POST_ORDER;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers =
            create_l2_headers(wallet, creds, "POST", endpoint_path, Some(&body), timestamp).await?;

        // Inject builder headers if available
        let final_headers = if self.can_builder_auth() {
            match self
                ._generate_builder_headers(headers.clone(), "POST", endpoint_path, Some(&body))
                .await?
            {
                Some(builder_headers) => builder_headers.to_headers(),
                None => headers.to_headers(),
            }
        } else {
            headers.to_headers()
        };

        // Make request
        self.http_client
            .post(endpoint_path, Some(final_headers), Some(order_payload), None)
            .await
    }

    /// Posts multiple orders to the exchange
    pub async fn post_orders(&self, orders: Vec<PostOrdersArgs>) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        // Convert each order to payload format
        let owner = &creds.key;
        let payloads: Vec<_> = orders
            .iter()
            .map(|arg| {
                serde_json::json!({
                    "order": arg.order,
                    "owner": owner,
                    "orderType": arg.order_type,
                    "deferExec": false
                })
            })
            .collect();

        let body = serde_json::to_string(&payloads)?;

        let endpoint_path = endpoints::POST_ORDERS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers =
            create_l2_headers(wallet, creds, "POST", endpoint_path, Some(&body), timestamp).await?;

        // Inject builder headers if available
        let final_headers = if self.can_builder_auth() {
            match self
                ._generate_builder_headers(headers.clone(), "POST", endpoint_path, Some(&body))
                .await?
            {
                Some(builder_headers) => builder_headers.to_headers(),
                None => headers.to_headers(),
            }
        } else {
            headers.to_headers()
        };

        self.http_client
            .post(endpoint_path, Some(final_headers), Some(payloads), None)
            .await
    }

    /// Cancels a single order by ID
    pub async fn cancel_order(&self, order_id: &str) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let payload = OrderPayload {
            order_id: order_id.to_string(),
        };
        let body = serde_json::to_string(&payload)?;

        let endpoint_path = endpoints::CANCEL_ORDER;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(
            wallet,
            creds,
            "DELETE",
            endpoint_path,
            Some(&body),
            timestamp,
        )
        .await?
        .to_headers();

        self.http_client
            .delete(endpoint_path, Some(headers), Some(payload), None)
            .await
    }

    /// Cancels multiple orders by IDs
    pub async fn cancel_orders(&self, order_ids: Vec<String>) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        #[derive(serde::Serialize)]
        struct CancelOrdersPayload {
            order_ids: Vec<String>,
        }

        let payload = CancelOrdersPayload { order_ids };
        let body = serde_json::to_string(&payload)?;

        let endpoint_path = endpoints::CANCEL_ORDERS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(
            wallet,
            creds,
            "DELETE",
            endpoint_path,
            Some(&body),
            timestamp,
        )
        .await?
        .to_headers();

        self.http_client
            .delete(endpoint_path, Some(headers), Some(payload), None)
            .await
    }

    /// Cancels all open orders
    pub async fn cancel_all(&self) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::CANCEL_ALL;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "DELETE", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client
            .delete(endpoint_path, Some(headers), None::<()>, None)
            .await
    }

    /// Cancels orders for a specific market or asset
    pub async fn cancel_market_orders(
        &self,
        params: OrderMarketCancelParams,
    ) -> ClobResult<serde_json::Value> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let body = serde_json::to_string(&params)?;

        let endpoint_path = endpoints::CANCEL_MARKET_ORDERS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(
            wallet,
            creds,
            "DELETE",
            endpoint_path,
            Some(&body),
            timestamp,
        )
        .await?
        .to_headers();

        self.http_client
            .delete(endpoint_path, Some(headers), Some(params), None)
            .await
    }

    /// Checks if an order is eligible for rewards
    pub async fn is_order_scoring(&self, params: OrderScoringParams) -> ClobResult<OrderScoring> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::IS_ORDER_SCORING;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();
        query_params.insert("order_id".to_string(), params.order_id);

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    /// Checks if multiple orders are eligible for rewards
    pub async fn are_orders_scoring(
        &self,
        params: OrdersScoringParams,
    ) -> ClobResult<OrdersScoring> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::ARE_ORDERS_SCORING;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();
        query_params.insert("order_ids".to_string(), params.order_ids.join(","));

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    // ===================================
    // L2 Auth Methods (Rewards)
    // ===================================

    /// Gets daily earnings for the user (with automatic pagination)
    pub async fn get_earnings_for_user_for_day(&self, date: &str) -> ClobResult<Vec<UserEarning>> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_EARNINGS_FOR_USER_FOR_DAY;

        let mut results = Vec::new();
        let mut next_cursor = INITIAL_CURSOR.to_string();

        while next_cursor != END_CURSOR {
            let timestamp = if self.use_server_time {
                Some(self.get_server_time().await?)
            } else {
                None
            };

            let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
                .await?
                .to_headers();

            let mut query_params = HashMap::new();
            query_params.insert("date".to_string(), date.to_string());
            query_params.insert(
                "signature_type".to_string(),
                self.signature_type.to_string(),
            );
            query_params.insert("next_cursor".to_string(), next_cursor.clone());

            #[derive(Deserialize)]
            struct EarningsResponse {
                data: Vec<UserEarning>,
                next_cursor: String,
            }

            let response: EarningsResponse = self
                .http_client
                .get(endpoint_path, Some(headers), Some(query_params))
                .await?;

            next_cursor = response.next_cursor;
            results.extend(response.data);
        }

        Ok(results)
    }

    /// Gets total daily earnings for the user
    pub async fn get_total_earnings_for_user_for_day(
        &self,
        date: &str,
    ) -> ClobResult<Vec<TotalUserEarning>> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_TOTAL_EARNINGS_FOR_USER_FOR_DAY;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        let mut query_params = HashMap::new();
        query_params.insert("date".to_string(), date.to_string());

        self.http_client
            .get(endpoint_path, Some(headers), Some(query_params))
            .await
    }

    /// Gets detailed earnings and markets config for the user (with automatic pagination)
    pub async fn get_user_earnings_and_markets_config(
        &self,
        date: &str,
        order_by: &str,
        position: &str,
        no_competition: bool,
    ) -> ClobResult<Vec<UserRewardsEarning>> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_REWARDS_EARNINGS_PERCENTAGES;

        let mut results = Vec::new();
        let mut next_cursor = INITIAL_CURSOR.to_string();

        while next_cursor != END_CURSOR {
            let timestamp = if self.use_server_time {
                Some(self.get_server_time().await?)
            } else {
                None
            };

            let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
                .await?
                .to_headers();

            let mut query_params = HashMap::new();
            query_params.insert("date".to_string(), date.to_string());
            query_params.insert(
                "signature_type".to_string(),
                self.signature_type.to_string(),
            );
            query_params.insert("next_cursor".to_string(), next_cursor.clone());
            query_params.insert("order_by".to_string(), order_by.to_string());
            query_params.insert("position".to_string(), position.to_string());
            query_params.insert("no_competition".to_string(), no_competition.to_string());

            #[derive(Deserialize)]
            struct UserRewardsEarningResponse {
                data: Vec<UserRewardsEarning>,
                next_cursor: String,
            }

            let response: UserRewardsEarningResponse = self
                .http_client
                .get(endpoint_path, Some(headers), Some(query_params))
                .await?;

            next_cursor = response.next_cursor;
            results.extend(response.data);
        }

        Ok(results)
    }

    /// Gets reward distribution percentages
    pub async fn get_reward_percentages(&self) -> ClobResult<RewardsPercentages> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_REWARDS_EARNINGS_PERCENTAGES;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(endpoint_path, Some(headers), None).await
    }

    /// Gets current reward programs (with automatic pagination)
    pub async fn get_current_rewards(&self) -> ClobResult<Vec<MarketReward>> {
        let mut results = Vec::new();
        let mut next_cursor = INITIAL_CURSOR.to_string();

        while next_cursor != END_CURSOR {
            let mut params = HashMap::new();
            params.insert("next_cursor".to_string(), next_cursor.clone());

            #[derive(Deserialize)]
            struct RewardsResponse {
                data: Vec<MarketReward>,
                next_cursor: String,
            }

            let response: RewardsResponse =
                self.http_client.get(endpoints::GET_REWARDS_MARKETS_CURRENT, None, Some(params)).await?;

            next_cursor = response.next_cursor;
            results.extend(response.data);
        }

        Ok(results)
    }

    /// Gets raw rewards for a specific market (with automatic pagination)
    pub async fn get_raw_rewards_for_market(
        &self,
        condition_id: &str,
    ) -> ClobResult<Vec<MarketReward>> {
        let endpoint = format!(
            "{}{}",
            endpoints::GET_REWARDS_MARKETS,
            condition_id
        );

        let mut results = Vec::new();
        let mut next_cursor = INITIAL_CURSOR.to_string();

        while next_cursor != END_CURSOR {
            let mut params = HashMap::new();
            params.insert("next_cursor".to_string(), next_cursor.clone());

            #[derive(Deserialize)]
            struct RewardsResponse {
                data: Vec<MarketReward>,
                next_cursor: String,
            }

            let response: RewardsResponse =
                self.http_client.get(&endpoint, None, Some(params)).await?;

            next_cursor = response.next_cursor;
            results.extend(response.data);
        }

        Ok(results)
    }

    /// Gets public market trade events
    pub async fn get_market_trades_events(
        &self,
        condition_id: &str,
    ) -> ClobResult<Vec<MarketTradeEvent>> {
        let endpoint = format!(
            "{}{}{}",
            self.host,
            endpoints::GET_MARKET_TRADES_EVENTS,
            condition_id
        );
        self.http_client.get(&endpoint, None, None).await
    }

    // ===================================
    // Public Method (Market Price Calculation)
    // ===================================

    /// Calculates market execution price from orderbook
    ///
    /// # Arguments
    ///
    /// * `token_id` - Token ID to calculate price for
    /// * `side` - Buy or Sell
    /// * `amount` - Amount in USDC (for Buy) or tokens (for Sell)
    /// * `order_type` - FOK or FAK
    ///
    /// # Returns
    ///
    /// Calculated execution price with buffer
    pub async fn calculate_market_price(
        &self,
        token_id: &str,
        side: Side,
        amount: f64,
        order_type: OrderType,
    ) -> ClobResult<f64> {
        let orderbook = self.get_order_book(token_id).await?;

        match side {
            Side::Buy => {
                if orderbook.asks.is_empty() {
                    return Err(ClobError::NoMatch);
                }
                calculate_buy_market_price(&orderbook.asks, amount, order_type)
            }
            Side::Sell => {
                if orderbook.bids.is_empty() {
                    return Err(ClobError::NoMatch);
                }
                calculate_sell_market_price(&orderbook.bids, amount, order_type)
            }
        }
    }

    // ===================================
    // L2 Auth Methods (Builder API Management)
    // ===================================

    /// Creates a builder API key
    pub async fn create_builder_api_key(&self) -> ClobResult<BuilderApiKey> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::CREATE_BUILDER_API_KEY;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "POST", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client
            .post(endpoint_path, Some(headers), None::<()>, None)
            .await
    }

    /// Gets all builder API keys
    pub async fn get_builder_api_keys(&self) -> ClobResult<Vec<BuilderApiKeyResponse>> {
        self.can_l2_auth()?;

        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        let creds = self.creds.as_ref().ok_or(ClobError::L2AuthNotAvailable)?;

        let endpoint_path = endpoints::GET_BUILDER_API_KEYS;
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        let headers = create_l2_headers(wallet, creds, "GET", endpoint_path, None, timestamp)
            .await?
            .to_headers();

        self.http_client.get(endpoint_path, Some(headers), None).await
    }

    /// Revokes a builder API key
    pub async fn revoke_builder_api_key(&self) -> ClobResult<serde_json::Value> {
        self.must_builder_auth()?;

        let endpoint_path = endpoints::REVOKE_BUILDER_API_KEY;

        // Get builder headers (already a HashMap)
        let headers = self
            ._get_builder_headers("DELETE", endpoint_path, None)
            .await?;

        self.http_client
            .delete(endpoint_path, Some(headers), None::<()>, None)
            .await
    }

    // ===================================
    // Private Helper Methods
    // ===================================

    /// Checks if L1 authentication is available
    fn can_l1_auth(&self) -> ClobResult<()> {
        if self.wallet.is_none() {
            return Err(ClobError::L1AuthUnavailable);
        }
        Ok(())
    }

    /// Checks if L2 authentication is available
    fn can_l2_auth(&self) -> ClobResult<()> {
        self.can_l1_auth()?;

        if self.creds.is_none() {
            return Err(ClobError::L2AuthNotAvailable);
        }

        Ok(())
    }

    /// Checks if builder authentication is available
    fn can_builder_auth(&self) -> bool {
        self.builder_config
            .as_ref()
            .map_or(false, |config| config.is_valid())
    }

    /// Ensures builder authentication is available, returns error otherwise
    fn must_builder_auth(&self) -> ClobResult<()> {
        if !self.can_builder_auth() {
            return Err(ClobError::BuilderAuthNotAvailable);
        }
        Ok(())
    }

    /// Gets builder headers for builder API authentication
    async fn _get_builder_headers(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> ClobResult<BuilderHeaderPayload> {
        let config = self
            .builder_config
            .as_ref()
            .ok_or(ClobError::BuilderAuthNotAvailable)?;

        // Get timestamp if server time is enabled
        let timestamp = if self.use_server_time {
            Some(self.get_server_time().await?)
        } else {
            None
        };

        // BuilderHeaderPayload is a HashMap<String, String>, so no conversion needed
        config
            .generate_builder_headers(method, path, body, timestamp)
            .await
            .map_err(|_e| ClobError::BuilderAuthFailed)
    }

    /// Generates L2 headers with builder headers injected
    async fn _generate_builder_headers(
        &self,
        l2_headers: L2PolyHeader,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> ClobResult<Option<L2WithBuilderHeader>> {
        if self.builder_config.is_none() {
            return Ok(None);
        }

        match self._get_builder_headers(method, path, body).await {
            Ok(builder_headers) => Ok(Some(inject_builder_headers(l2_headers, builder_headers))),
            Err(_) => Ok(None),
        }
    }

    /// Resolves the fee rate for a token
    ///
    /// If the user provides a fee rate and it doesn't match the market fee rate,
    /// returns an error.
    async fn _resolve_fee_rate_bps(
        &self,
        token_id: &str,
        user_fee: Option<u32>,
    ) -> ClobResult<u32> {
        let market_fee = self.get_fee_rate_bps(token_id).await?;

        if let Some(user_provided) = user_fee {
            if market_fee > 0 && user_provided != market_fee {
                return Err(ClobError::InvalidFeeRate {
                    user_fee_rate: user_provided,
                    market_fee_rate: market_fee,
                });
            }
        }

        Ok(market_fee)
    }

    /// Converts order to JSON payload for API submission
    fn order_to_json(
        &self,
        order: serde_json::Value,
        order_type: OrderType,
    ) -> ClobResult<serde_json::Value> {
        let owner = self
            .creds
            .as_ref()
            .ok_or(ClobError::L2AuthNotAvailable)?
            .key
            .clone();

        // Wrap the order in the expected payload format
        Ok(serde_json::json!({
            "order": order,
            "owner": owner,
            "orderType": order_type,
            "deferExec": false
        }))
    }

    /// Converts a SignedOrder to JSON format for API submission
    fn signed_order_to_json(&self, signed_order: SignedOrder) -> ClobResult<serde_json::Value> {
        serde_json::to_value(&signed_order).map_err(|e| ClobError::JsonError(e))
    }
}
