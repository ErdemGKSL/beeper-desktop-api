//! Integration tests for account operations

mod common;

use beeper_desktop_api::BeeperClient;
use common::{get_test_token, get_test_base_url, should_run_integration_tests};

#[tokio::test]
async fn test_get_accounts() {
    if !should_run_integration_tests() {
        println!("Skipping test_get_accounts - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.get_accounts().await {
        Ok(accounts) => {
            println!("✓ Successfully retrieved {} accounts", accounts.len());
            assert!(!accounts.is_empty(), "Expected at least one account");
        }
        Err(e) => {
            panic!("Failed to get accounts: {}", e);
        }
    }
}

#[tokio::test]
async fn test_client_creation() {
    let client = BeeperClient::new("test-token", "http://localhost:23373");
    assert_eq!(client.base_url(), "http://localhost:23373");
}

#[tokio::test]
async fn test_client_with_default_url() {
    let client = BeeperClient::with_token("test-token");
    assert_eq!(client.base_url(), "http://localhost:23373");
}

#[tokio::test]
async fn test_token_update() {
    let mut client = BeeperClient::new("original-token", "http://localhost:23373");
    client.set_token("new-token");
    // Just verify it doesn't panic - we can't directly verify the token is private
    let client_ref = &client;
    assert_eq!(client_ref.base_url(), "http://localhost:23373");
}

#[tokio::test]
async fn test_base_url_update() {
    let mut client = BeeperClient::new("test-token", "http://localhost:23373");
    client.set_base_url("http://localhost:3000");
    assert_eq!(client.base_url(), "http://localhost:3000");
}
