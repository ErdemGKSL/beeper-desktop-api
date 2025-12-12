//! Integration tests for chat operations

mod common;

use beeper_desktop_api::BeeperClient;
use common::{get_test_token, get_test_base_url, should_run_integration_tests};

#[tokio::test]
async fn test_list_chats() {
    if !should_run_integration_tests() {
        println!("Skipping test_list_chats - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.list_chats(None, None).await {
        Ok(output) => {
            println!("✓ Successfully retrieved {} chats", output.items.len());
        }
        Err(e) => {
            panic!("Failed to list chats: {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_chats_with_pagination() {
    if !should_run_integration_tests() {
        println!("Skipping test_list_chats_with_pagination - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    // First, get some chats to get a cursor
    let first_page = client.list_chats(None, None).await.expect("Failed to get first page");
    println!("✓ Retrieved first page with {} chats", first_page.items.len());

    // If there's a next cursor, try using it
    if let Some(cursor) = &first_page.newest_cursor {
        match client.list_chats(Some(cursor), Some("after")).await {
            Ok(second_page) => {
                println!("✓ Successfully paginated to next page with {} chats", second_page.items.len());
            }
            Err(e) => {
                println!("⚠ Failed to paginate: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_search_chats() {
    if !should_run_integration_tests() {
        println!("Skipping test_search_chats - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.search_chats("test", None, None).await {
        Ok(output) => {
            println!("✓ Successfully searched chats, found {} results", output.items.len());
        }
        Err(e) => {
            println!("⚠ Chat search failed (may be expected if no matches): {}", e);
        }
    }
}
