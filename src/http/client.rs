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

    /// Add default headers to the request (similar to TypeScript overloadHeaders)
    fn add_default_headers(
        &self,
        method: &str,
        headers: Option<HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut final_headers = headers.unwrap_or_default();

        // Add default headers if not already present
        final_headers
            .entry("User-Agent".to_string())
            .or_insert_with(|| "@polymarket/clob-client".to_string());
        final_headers
            .entry("Accept".to_string())
            .or_insert_with(|| "*/*".to_string());
        final_headers
            .entry("Connection".to_string())
            .or_insert_with(|| "keep-alive".to_string());
        final_headers
            .entry("Content-Type".to_string())
            .or_insert_with(|| "application/json".to_string());

        // Add Accept-Encoding for GET requests
        if method == "GET" {
            final_headers
                .entry("Accept-Encoding".to_string())
                .or_insert_with(|| "gzip".to_string());
        }

        final_headers
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

        // Add default headers merged with provided headers
        let final_headers = self.add_default_headers("GET", headers);
        for (key, value) in final_headers {
            request = request.header(key, value);
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

        // Add default headers merged with provided headers
        let final_headers = self.add_default_headers("POST", headers);
        for (key, value) in final_headers {
            request = request.header(key, value);
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

        // Add default headers merged with provided headers
        let final_headers = self.add_default_headers("DELETE", headers);
        for (key, value) in final_headers {
            request = request.header(key, value);
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
        let url = response.url().clone();

        if status.is_success() {
            // Parse successful response
            let data = response.json::<T>().await.map_err(|e| {
                // Convert reqwest error to JSON error via string
                let error_msg = format!("Failed to parse JSON response: {}", e);
                eprintln!("[CLOB Client] request error: {}", error_msg);
                ClobError::Other(error_msg)
            })?;
            Ok(data)
        } else {
            // Handle error response with detailed logging
            let status_code = status.as_u16();
            let status_text = status.canonical_reason().unwrap_or("Unknown");
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Log error details similar to TypeScript version
            eprintln!(
                "[CLOB Client] request error: {{\"status\": {}, \"statusText\": \"{}\", \"data\": \"{}\", \"url\": \"{}\"}}",
                status_code, status_text, error_text, url
            );

            Err(ClobError::ApiError {
                message: error_text,
                status: status_code,
            })
        }
    }
}
