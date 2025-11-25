use crate::client::ClobClient;
use crate::constants::{END_CURSOR, INITIAL_CURSOR};
use crate::endpoints::endpoints;
use crate::errors::{ClobError, ClobResult};
use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

impl ClobClient {
    // ===================================
    // Public Endpoints (No Auth Required)
    // ===================================

    /// Server
    pub async fn get_ok(&self) -> ClobResult<serde_json::Value> {
        self.http_client.get("/", None, None).await
    }

    pub async fn get_server_time(&self) -> ClobResult<u64> {
        self.http_client.get(endpoints::TIME, None, None).await
    }

    /// Tags
    pub async fn get_tags(&self, params: TagParams) -> ClobResult<Vec<Tag>> {
        let endpoint = endpoints::GET_TAGS;

        let mut query_params = HashMap::new();
        if let Some(limit) = params.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = params.offset {
            query_params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(order) = params.order {
            query_params.insert("order".to_string(), order);
        }
        if let Some(ascending) = params.ascending {
            query_params.insert("ascending".to_string(), ascending.to_string());
        }

        self.gamma_api_client
            .get(endpoint, None, Some(query_params))
            .await
    }

    pub async fn get_tag_by_slug(&self, slug: &str) -> ClobResult<Tag> {
        if slug.is_empty() {
            return Err(ClobError::Other("Slug is required".to_string()));
        }

        let endpoint = format!("{}{}", endpoints::GET_TAG_BY_SLUG, slug);
        self.gamma_api_client.get(&endpoint, None, None).await
    }

    pub async fn get_popular_tags(&self) -> ClobResult<Vec<Tag>> {
        Ok(crate::constants::get_popular_tags())
    }

    /// Events
    pub async fn get_events(&self, params: EventParams) -> ClobResult<Vec<Event>> {
        let endpoint = endpoints::GET_EVENTS;

        let mut query_params = HashMap::new();
        if let Some(limit) = params.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = params.offset {
            query_params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(tag_id) = params.tag_id {
            query_params.insert("tag_id".to_string(), tag_id.to_string());
        }
        if let Some(closed) = params.closed {
            query_params.insert("closed".to_string(), closed.to_string());
        }
        if let Some(order) = params.order {
            query_params.insert("order".to_string(), order);
        }
        if let Some(ascending) = params.ascending {
            query_params.insert("ascending".to_string(), ascending.to_string());
        }

        self.gamma_api_client
            .get(endpoint, None, Some(query_params))
            .await
    }

    pub async fn get_events_by_id(&self, id: &str) -> ClobResult<Event> {
        let endpoint = format!("{}{}", endpoints::GET_EVENT, id);
        self.gamma_api_client.get(&endpoint, None, None).await
    }

    pub async fn get_event_by_slug(&self, slug: &str) -> ClobResult<Event> {
        let endpoint = format!("{}{}", endpoints::GET_EVENT_BY_SLUG, slug);

        self.gamma_api_client.get(&endpoint, None, None).await
    }

    /// Markets
    pub async fn get_markets(&self, params: MarketParams) -> ClobResult<Vec<Market>> {
        let endpoint = endpoints::GET_MARKETS;

        let mut query_params = HashMap::new();
        if let Some(limit) = params.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = params.offset {
            query_params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(order) = params.order {
            query_params.insert("order".to_string(), order);
        }
        if let Some(ascending) = params.ascending {
            query_params.insert("ascending".to_string(), ascending.to_string());
        }
        if let Some(condition_id) = params.condition_id {  
            query_params.insert("condition_id".to_string(), condition_id.to_string());
        }
        if let Some(closed) = params.closed {
            query_params.insert("closed".to_string(), closed.to_string());
        }

        self.gamma_api_client.get(endpoint, None, Some(query_params)).await
    }

    pub async fn get_market_by_id(&self, id: &str) -> ClobResult<Market> {
        let endpoint = format!("{}{}", endpoints::GET_MARKET, id);
        self.gamma_api_client.get(&endpoint, None, None).await
    }

    pub async fn get_market_by_slug(&self, slug: &str) -> ClobResult<Market> {
        let endpoint = format!("{}{}", endpoints::GET_MARKET_BY_SLUG, slug);
        self.gamma_api_client.get(&endpoint, None, None).await
    }

    /// Orderbook
    pub async fn get_order_book(&self, token_id: &str) -> ClobResult<OrderBookSummary> {
        let mut params = HashMap::new();
        params.insert("token_id".to_string(), token_id.to_string());

        self.http_client
            .get(endpoints::GET_ORDER_BOOK, None, Some(params))
            .await
    }

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

        let response: TickSizeResponse = self
            .http_client
            .get(endpoints::GET_TICK_SIZE, None, Some(params))
            .await?;
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

        let response: NegRiskResponse = self
            .http_client
            .get(endpoints::GET_NEG_RISK, None, Some(params))
            .await?;

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

        let response: FeeRateResponse = self
            .http_client
            .get(endpoints::GET_FEE_RATE, None, Some(params))
            .await?;

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

        self.http_client
            .get(endpoints::GET_MIDPOINT, None, Some(params))
            .await
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

        self.http_client
            .get(endpoints::GET_PRICE, None, Some(params))
            .await
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

        self.http_client
            .get(endpoints::GET_SPREAD, None, Some(params))
            .await
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

        self.http_client
            .get(endpoints::GET_LAST_TRADE_PRICE, None, Some(params))
            .await
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

            let response: RewardsResponse = self
                .http_client
                .get(endpoints::GET_REWARDS_MARKETS_CURRENT, None, Some(params))
                .await?;

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
        let endpoint = format!("{}{}", endpoints::GET_REWARDS_MARKETS, condition_id);

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
}

