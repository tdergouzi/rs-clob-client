use crate::client::ClobClient;
use crate::constants::{END_CURSOR, INITIAL_CURSOR};
use crate::endpoints::endpoints;
use crate::errors::{ClobError, ClobResult};
use crate::headers::create_l2_headers;
use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

impl ClobClient {
    // ===================================
    // L2 Auth Methods
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

        self.http_client
            .get(endpoint_path, Some(headers), None)
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
}

