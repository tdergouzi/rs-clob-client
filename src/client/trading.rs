use crate::client::ClobClient;
use crate::constants::{END_CURSOR, INITIAL_CURSOR};
use crate::endpoints::endpoints;
use crate::errors::{ClobError, ClobResult};
use crate::headers::create_l2_headers;
use crate::order_builder::{calculate_buy_market_price, calculate_sell_market_price};
use crate::types::*;
use rs_order_utils::SignedOrder;
use std::collections::HashMap;

impl ClobClient {
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

        self.http_client
            .get(&endpoint_path, Some(headers), None)
            .await
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

        self.http_client
            .get(
                endpoint_path,
                Some(headers),
                (!query_params.is_empty()).then_some(query_params),
            )
            .await
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
            .post(
                endpoint_path,
                Some(final_headers),
                Some(order_payload),
                None,
            )
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
    // Private Helper Methods
    // ===================================

    /// Resolves the fee rate for a token
    ///
    /// If the user provides a fee rate and it doesn't match the market fee rate,
    /// returns an error.
    pub(crate) async fn _resolve_fee_rate_bps(
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
        }))
    }

    /// Converts a SignedOrder to JSON format for API submission
    fn signed_order_to_json(&self, signed_order: SignedOrder) -> ClobResult<serde_json::Value> {
        let mut json = serde_json::to_value(&signed_order).map_err(|e| ClobError::JsonError(e))?;
        
        // Convert numeric side ("0" or "1") to string side ("BUY" or "SELL")
        // The API expects "BUY"/"SELL" strings, not numeric values
        if let Some(side) = json.get("side") {
            let side_str = match side.as_str() {
                Some("0") => "BUY",
                Some("1") => "SELL",
                _ => return Ok(json), // Keep as-is if already correct format
            };
            json["side"] = serde_json::Value::String(side_str.to_string());
        }
        
        Ok(json)
    }
}
