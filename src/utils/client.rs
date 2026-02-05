//! HTTP Client wrapper for TITAN-CLI
//!
//! Provides a configured reqwest client with:
//! - Default timeouts
//! - Custom User-Agent header
//! - Error handling

use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// Titan Protocol HTTP Client
pub struct TitanClient {
    client: Client,
}

impl TitanClient {
    /// Create a new TitanClient with default configuration
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("TITAN-CLI/1.0")
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(3))
            .build()?;

        Ok(Self { client })
    }

    /// Check the health of an endpoint
    pub async fn check_health(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok("healthy".to_string())
        } else {
            Ok(format!("unhealthy: {}", response.status()))
        }
    }

    /// Send a GET request and return the response body
    #[allow(dead_code)]
    pub async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    /// Send a POST request with JSON body
    #[allow(dead_code)]
    pub async fn post<T: serde::Serialize>(&self, url: &str, body: &T) -> Result<String> {
        let response = self.client.post(url).json(body).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}

impl Default for TitanClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
