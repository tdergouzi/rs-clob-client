use crate::client::ClobClient;
use crate::endpoints::endpoints;
use crate::errors::{ClobError, ClobResult};
use crate::headers::{create_l1_headers, create_l2_headers, inject_builder_headers};
use crate::types::*;
use rs_builder_signing_sdk::BuilderHeaderPayload;
use std::collections::HashMap;

impl ClobClient {
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
        let response: ApiKeyRaw = self
            .http_client
            .get(endpoints::DERIVE_API_KEY, Some(headers), None)
            .await?;

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

        self.http_client
            .get(endpoint_path, Some(headers), None)
            .await
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

        self.http_client
            .get(endpoint_path, Some(headers), None)
            .await
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

        self.http_client
            .get(endpoint_path, Some(headers), None)
            .await
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

        self.http_client
            .get(endpoint_path, Some(headers), None)
            .await
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
    pub(crate) fn can_l1_auth(&self) -> ClobResult<()> {
        if self.wallet.is_none() {
            return Err(ClobError::L1AuthUnavailable);
        }
        Ok(())
    }

    /// Checks if L2 authentication is available
    pub(crate) fn can_l2_auth(&self) -> ClobResult<()> {
        self.can_l1_auth()?;

        if self.creds.is_none() {
            return Err(ClobError::L2AuthNotAvailable);
        }

        Ok(())
    }

    /// Checks if builder authentication is available
    pub(crate) fn can_builder_auth(&self) -> bool {
        self.builder_config
            .as_ref()
            .map_or(false, |config| config.is_valid())
    }

    /// Ensures builder authentication is available, returns error otherwise
    pub(crate) fn must_builder_auth(&self) -> ClobResult<()> {
        if !self.can_builder_auth() {
            return Err(ClobError::BuilderAuthNotAvailable);
        }
        Ok(())
    }

    /// Gets builder headers for builder API authentication
    pub(crate) async fn _get_builder_headers(
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
    pub(crate) async fn _generate_builder_headers(
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
}

