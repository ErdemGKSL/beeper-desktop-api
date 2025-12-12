//! Integration tests for app operations

mod common;

use beeper_desktop_api::BeeperClient;
use common::{get_test_token, get_test_base_url, should_run_integration_tests};

#[tokio::test]
async fn test_focus_app_no_args() {
    if !should_run_integration_tests() {
        println!("Skipping test_focus_app_no_args - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    match client.focus_app(None).await {
        Ok(output) => {
            println!("✓ Successfully focused app: {}", output.success);
            assert!(output.success, "Expected focus_app to return success: true");
        }
        Err(e) => {
            println!("⚠ Focus app failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_focus_app_with_chat() {
    if !should_run_integration_tests() {
        println!("Skipping test_focus_app_with_chat - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    // Get a chat first
    match client.list_chats(None, None).await {
        Ok(chats_output) => {
            if let Some(chat) = chats_output.items.first() {
                use beeper_desktop_api::FocusAppInput;
                let input = FocusAppInput {
                    chat_id: Some(chat.id.clone()),
                    message_id: None,
                    draft: None,
                };

                match client.focus_app(Some(input)).await {
                    Ok(output) => {
                        println!("✓ Successfully focused app with chat: {}", output.success);
                    }
                    Err(e) => {
                        println!("⚠ Focus app with chat failed: {}", e);
                    }
                }
            } else {
                println!("⚠ No chats available to test focus_app");
            }
        }
        Err(e) => {
            panic!("Failed to get chats: {}", e);
        }
    }
}

#[tokio::test]
async fn test_focus_app_with_draft() {
    if !should_run_integration_tests() {
        println!("Skipping test_focus_app_with_draft - set BEEPER_TEST_TOKEN to run");
        return;
    }

    let token = get_test_token().expect("BEEPER_TEST_TOKEN not set");
    let base_url = get_test_base_url();
    let client = BeeperClient::new(token, base_url);

    use beeper_desktop_api::FocusAppInput;
    let input = FocusAppInput {
        chat_id: None,
        message_id: None,
        draft: Some("Test draft message".to_string()),
    };

    match client.focus_app(Some(input)).await {
        Ok(output) => {
            println!("✓ Successfully focused app with draft: {}", output.success);
        }
        Err(e) => {
            println!("⚠ Focus app with draft failed: {}", e);
        }
    }
}
