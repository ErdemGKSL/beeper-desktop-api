/// Client module for Beeper Desktop API
///
/// Contains the main BeeperClient and method implementations for different API areas.

pub mod chats;
pub mod messages;
pub mod accounts;
pub mod search;
pub mod app;
mod utils;

use reqwest::Client;
use serde::Deserialize;
use self::utils::handle_response;

/// Main Beeper API client
///
/// Stores the bearer token and base URL for API requests.
/// All requests to the API will include the token in the Authorization header.
#[derive(Clone)]
pub struct BeeperClient {
    token: String,
    base_url: String,
    http_client: Client,
}

impl BeeperClient {
    /// Creates a new Beeper API client
    ///
    /// # Arguments
    ///
    /// * `token` - Bearer token for authentication
    /// * `base_url` - Base URL of the Beeper Desktop API server
    pub fn new(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: base_url.into(),
            http_client: Client::new(),
        }
    }

    /// Creates a new Beeper API client with default base URL
    ///
    /// # Arguments
    ///
    /// * `token` - Bearer token for authentication
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::new(token, DEFAULT_BASE_URL)
    }

    /// Updates the bearer token
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = token.into();
    }

    /// Updates the base URL
    pub fn set_base_url(&mut self, base_url: impl Into<String>) {
        self.base_url = base_url.into();
    }

    /// Gets the current base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub(crate) fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn get_http_client(&self) -> &Client {
        &self.http_client
    }
}

const DEFAULT_BASE_URL: &str = "http://localhost:23373";

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BeeperClient::new("test-token", "http://localhost:23373");
        assert_eq!(client.base_url(), "http://localhost:23373");
    }

    #[test]
    fn test_client_with_default_url() {
        let client = BeeperClient::with_token("test-token");
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_set_token() {
        let mut client = BeeperClient::new("old-token", "http://localhost:23373");
        client.set_token("new-token");
        assert_eq!(client.base_url(), "http://localhost:23373");
    }

    #[test]
    fn test_client_set_base_url() {
        let mut client = BeeperClient::new("test-token", "http://localhost:23373");
        client.set_base_url("http://example.com:8080");
        assert_eq!(client.base_url(), "http://example.com:8080");
    }

    #[test]
    fn test_client_clone() {
        let client1 = BeeperClient::new("test-token", "http://localhost:23373");
        let client2 = client1.clone();
        assert_eq!(client1.base_url(), client2.base_url());
    }

    #[test]
    fn test_get_auth_header() {
        let client = BeeperClient::new("my-secret-token", "http://localhost:23373");
        let auth_header = client.get_auth_header();
        assert_eq!(auth_header, "Bearer my-secret-token");
    }
}
