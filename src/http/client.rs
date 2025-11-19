// HTTP client implementation
// Phase 3: Complete HTTP layer with authentication header injection

use crate::errors::{ClobError, ClobResult};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;

/// HTTP client for making requests to the CLOB API
pub struct HttpClient {
    client: Client,
    base_url: String,
    geo_block_token: Option<String>,
}

impl HttpClient {
    /// Create a new HTTP client with the given base URL
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            geo_block_token: None,
        }
    }

    /// Set a geo-block token for bypassing geographic restrictions
    pub fn with_geo_block_token(mut self, token: String) -> Self {
        self.geo_block_token = Some(token);
        self
    }

    /// Send a GET request
    pub async fn get<T>(
        &self,
        endpoint: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
    ) -> ClobResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.get(&url);

        // Add headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                request = request.header(key, value);
            }
        }

        // Add query parameters
        let mut query_params = params.unwrap_or_default();
        if let Some(token) = &self.geo_block_token {
            query_params.insert("geo_block_token".to_string(), token.clone());
        }
        if !query_params.is_empty() {
            request = request.query(&query_params);
        }

        // Send request and handle response
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Send a POST request
    pub async fn post<T, B>(
        &self,
        endpoint: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<B>,
        params: Option<HashMap<String, String>>,
    ) -> ClobResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.post(&url);

        // Add headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                request = request.header(key, value);
            }
        }

        // Add body
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }

        // Add query parameters
        let mut query_params = params.unwrap_or_default();
        if let Some(token) = &self.geo_block_token {
            query_params.insert("geo_block_token".to_string(), token.clone());
        }
        if !query_params.is_empty() {
            request = request.query(&query_params);
        }

        // Send request and handle response
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Send a DELETE request
    pub async fn delete<T, B>(
        &self,
        endpoint: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<B>,
        params: Option<HashMap<String, String>>,
    ) -> ClobResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.delete(&url);

        // Add headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                request = request.header(key, value);
            }
        }

        // Add body (for delete with payload)
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }

        // Add query parameters
        let mut query_params = params.unwrap_or_default();
        if let Some(token) = &self.geo_block_token {
            query_params.insert("geo_block_token".to_string(), token.clone());
        }
        if !query_params.is_empty() {
            request = request.query(&query_params);
        }

        // Send request and handle response
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handle HTTP response and parse JSON or return error
    async fn handle_response<T>(&self, response: Response) -> ClobResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            // Parse successful response
            let data = response.json::<T>().await.map_err(|e| {
                // Convert reqwest error to JSON error via string
                let error_msg = format!("Failed to parse JSON response: {}", e);
                ClobError::Other(error_msg)
            })?;
            Ok(data)
        } else {
            // Handle error response
            let status_code = status.as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(ClobError::ApiError {
                message: error_text,
                status: status_code,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_get_request() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result":"success"}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let result: serde_json::Value = client.get("/test", None, None).await.unwrap();
        assert_eq!(result["result"], "success");
    }

    #[tokio::test]
    async fn test_get_with_params() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/markets")
            .match_query(mockito::Matcher::UrlEncoded(
                "status".into(),
                "active".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"markets":[]}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let mut params = HashMap::new();
        params.insert("status".to_string(), "active".to_string());

        let result: serde_json::Value = client.get("/markets", None, Some(params)).await.unwrap();
        assert!(result["markets"].is_array());
    }

    #[tokio::test]
    async fn test_post_request() {
        let mut server = Server::new_async().await;
        let _m = server.mock("POST", "/order")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"orderId":"123"}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let body = serde_json::json!({"price": 0.5});
        let result: serde_json::Value = client
            .post("/order", None, Some(body), None)
            .await
            .unwrap();
        assert_eq!(result["orderId"], "123");
    }

    #[tokio::test]
    async fn test_delete_request() {
        let mut server = Server::new_async().await;
        let _m = server.mock("DELETE", "/order/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success":true}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let result: serde_json::Value = client
            .delete::<serde_json::Value, serde_json::Value>("/order/123", None, None, None)
            .await
            .unwrap();
        assert_eq!(result["success"], true);
    }

    #[tokio::test]
    async fn test_error_response() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/error")
            .with_status(400)
            .with_body("Bad request")
            .create_async().await;

        let client = HttpClient::new(server.url());
        let result: Result<serde_json::Value, _> = client.get("/error", None, None).await;
        assert!(result.is_err());

        if let Err(ClobError::ApiError { message, status }) = result {
            assert_eq!(status, 400);
            assert_eq!(message, "Bad request");
        } else {
            panic!("Expected ApiError");
        }
    }

    #[tokio::test]
    async fn test_headers_injection() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/auth")
            .match_header("POLY_ADDRESS", "0x123")
            .with_status(200)
            .with_body(r#"{"authenticated":true}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let mut headers = HashMap::new();
        headers.insert("POLY_ADDRESS".to_string(), "0x123".to_string());

        let result: serde_json::Value = client.get("/auth", Some(headers), None).await.unwrap();
        assert_eq!(result["authenticated"], true);
    }

    #[tokio::test]
    async fn test_geo_block_token() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/markets")
            .match_query(mockito::Matcher::UrlEncoded(
                "geo_block_token".into(),
                "test_token".into(),
            ))
            .with_status(200)
            .with_body(r#"{"markets":[]}"#)
            .create_async().await;

        let client =
            HttpClient::new(server.url()).with_geo_block_token("test_token".to_string());

        let result: serde_json::Value = client.get("/markets", None, None).await.unwrap();
        assert!(result["markets"].is_array());
    }

    #[tokio::test]
    async fn test_delete_with_body() {
        let mut server = Server::new_async().await;
        let _m = server.mock("DELETE", "/orders")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deleted":2}"#)
            .create_async().await;

        let client = HttpClient::new(server.url());
        let body = serde_json::json!({"orderIds": ["1", "2"]});
        let result: serde_json::Value = client
            .delete("/orders", None, Some(body), None)
            .await
            .unwrap();
        assert_eq!(result["deleted"], 2);
    }
}

