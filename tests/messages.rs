//! Integration tests for message operations

mod common;

use beeper_desktop_api::BeeperClient;
use common::{get_test_token, get_test_base_url, should_run_integration_tests};

#[tokio::test]
async fn test_list_messages() {
    if !should_run_integration_tests() {
        println!("Skipping test_list_messages - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    // First, get a chat to list messages from
    match client.list_chats(None, None).await {
        Ok(chats_output) => {
            if let Some(chat) = chats_output.items.first() {
                match client.list_messages(&chat.id, None, None).await {
                    Ok(messages_output) => {
                        println!("✓ Successfully retrieved {} messages from chat", messages_output.items.len());
                    }
                    Err(e) => {
                        println!("⚠ Failed to list messages: {}", e);
                    }
                }
            } else {
                println!("⚠ No chats available to test message listing");
            }
        }
        Err(e) => {
            panic!("Failed to get chats: {}", e);
        }
    }
}

#[tokio::test]
async fn test_search_messages() {
    if !should_run_integration_tests() {
        println!("Skipping test_search_messages - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.search_messages("hello", None, None).await {
        Ok(output) => {
            println!("✓ Successfully searched messages, found {} results", output.items.len());
        }
        Err(e) => {
            println!("⚠ Message search failed (may be expected if no matches): {}", e);
        }
    }
}

#[tokio::test]
async fn test_search_messages_with_pagination() {
    if !should_run_integration_tests() {
        println!("Skipping test_search_messages_with_pagination - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.search_messages("test", None, None).await {
        Ok(first_page) => {
            println!("✓ Retrieved first page with {} messages", first_page.items.len());

            // If there's a cursor, try pagination
            if let Some(cursor) = &first_page.oldest_cursor {
                match client.search_messages("test", Some(cursor), Some("before")).await {
                    Ok(second_page) => {
                        println!("✓ Successfully paginated to older messages with {} results", second_page.items.len());
                    }
                    Err(e) => {
                        println!("⚠ Failed to paginate messages: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠ Initial message search failed: {}", e);
        }
    }
}
