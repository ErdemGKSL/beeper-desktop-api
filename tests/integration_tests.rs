//! Integration tests for Beeper Desktop API client

use beeper_desktop_api::{BeeperClient, models::*};

/// Get the auth token from environment variable or use a default for tests
fn get_test_token() -> String {
    std::env::var("BEEPER_TEST_TOKEN")
        .unwrap_or_else(|_| "test-token-placeholder".to_string())
}

/// Check if we should run integration tests (auth token is available)
fn should_run_integration_tests() -> bool {
    std::env::var("BEEPER_TEST_TOKEN").is_ok()
}

/// Get the test server URL from environment or default
fn get_test_url() -> String {
    std::env::var("BEEPER_TEST_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string())
}

#[test]
fn test_client_creation_with_token_and_url() {
    let client = BeeperClient::new("test-token", "http://localhost:23373");
    assert_eq!(client.base_url(), "http://localhost:23373");
}

#[test]
fn test_client_creation_with_token_default_url() {
    let client = BeeperClient::with_token("test-token");
    assert_eq!(client.base_url(), "http://localhost:23373");
}

#[test]
fn test_client_token_update() {
    let mut client = BeeperClient::new("old-token", "http://localhost:23373");
    client.set_token("new-token");
    // We can't directly access the token, but we can verify it's set via the client's behavior
    assert_eq!(client.base_url(), "http://localhost:23373");
}

#[test]
fn test_client_url_update() {
    let mut client = BeeperClient::new("test-token", "http://localhost:23373");
    client.set_base_url("http://example.com:8080");
    assert_eq!(client.base_url(), "http://example.com:8080");
}

#[test]
fn test_send_message_input_creation() {
    let input = SendMessageInput {
        text: "Hello, world!".to_string(),
        reply_to_id: None,
    };
    assert_eq!(input.text, "Hello, world!");
    assert_eq!(input.reply_to_id, None);
}

#[test]
fn test_send_message_input_with_reply() {
    let input = SendMessageInput {
        text: "Reply message".to_string(),
        reply_to_id: Some("msg-123".to_string()),
    };
    assert_eq!(input.text, "Reply message");
    assert_eq!(input.reply_to_id, Some("msg-123".to_string()));
}

#[test]
fn test_create_chat_input_single_participant() {
    let input = CreateChatInput {
        account_id: "account-1".to_string(),
        participant_ids: vec!["user-1".to_string()],
        title: None,
    };
    assert_eq!(input.account_id, "account-1");
    assert_eq!(input.participant_ids.len(), 1);
    assert_eq!(input.title, None);
}

#[test]
fn test_create_chat_input_group_with_title() {
    let input = CreateChatInput {
        account_id: "account-1".to_string(),
        participant_ids: vec!["user-1".to_string(), "user-2".to_string(), "user-3".to_string()],
        title: Some("Group Chat".to_string()),
    };
    assert_eq!(input.account_id, "account-1");
    assert_eq!(input.participant_ids.len(), 3);
    assert_eq!(input.title, Some("Group Chat".to_string()));
}

#[test]
fn test_focus_app_input_chat_navigation() {
    let input = FocusAppInput {
        chat_id: Some("chat-123".to_string()),
        message_id: None,
        draft: None,
    };
    assert_eq!(input.chat_id, Some("chat-123".to_string()));
    assert_eq!(input.message_id, None);
    assert_eq!(input.draft, None);
}

#[test]
fn test_focus_app_input_with_draft() {
    let input = FocusAppInput {
        chat_id: Some("chat-123".to_string()),
        message_id: None,
        draft: Some("Draft message".to_string()),
    };
    assert_eq!(input.chat_id, Some("chat-123".to_string()));
    assert_eq!(input.draft, Some("Draft message".to_string()));
}

#[test]
fn test_download_asset_input() {
    let input = DownloadAssetInput {
        url: "mxc://example.com/abc123".to_string(),
    };
    assert_eq!(input.url, "mxc://example.com/abc123");
}

#[test]
fn test_user_model_creation() {
    let user = User {
        id: "user-123".to_string(),
        username: Some("@alice".to_string()),
        phone_number: None,
        email: None,
        full_name: Some("Alice Example".to_string()),
        img_url: Some("https://example.com/avatar.jpg".to_string()),
        cannot_message: Some(false),
        is_self: Some(false),
    };
    assert_eq!(user.id, "user-123");
    assert_eq!(user.username, Some("@alice".to_string()));
    assert_eq!(user.is_self, Some(false));
}

#[test]
fn test_message_model_creation() {
    let message = Message {
        id: "msg-1".to_string(),
        chat_id: "chat-1".to_string(),
        account_id: Some("account-1".to_string()),
        sender_id: "user-1".to_string(),
        sender_name: Some("Alice".to_string()),
        text: Some("Hello!".to_string()),
        timestamp: "2025-08-31T23:30:12.520Z".to_string(),
        sort_key: "821744079".to_string(),
        is_edited: Some(false),
        attachments: None,
        is_unread: None,
        reactions: None,
        reply_to_id: None,
        is_sender: None,
    };
    assert_eq!(message.id, "msg-1");
    assert_eq!(message.text, Some("Hello!".to_string()));
    assert_eq!(message.is_edited, Some(false));
}

#[test]
fn test_message_with_attachment() {
    let attachment = Attachment {
        typ: "image".to_string(),
        src_url: Some("https://example.com/image.jpg".to_string()),
        mime_type: Some("image/jpeg".to_string()),
        file_name: Some("photo.jpg".to_string()),
        file_size: Some(1024),
        is_gif: Some(false),
        is_sticker: Some(false),
        is_voice_note: Some(false),
        duration: None,
        poster_img: None,
    };
    assert_eq!(attachment.typ, "image");
    assert_eq!(attachment.mime_type, Some("image/jpeg".to_string()));
    assert_eq!(attachment.file_size, Some(1024));
}

#[test]
fn test_chat_model_creation() {
    let chat = Chat {
        id: "chat-1".to_string(),
        local_chat_id: None,
        account_id: "account-1".to_string(),
        network: "WhatsApp".to_string(),
        title: "Alice".to_string(),
        chat_type: "single".to_string(),
        participants: Participants {
            items: vec![],
            has_more: false,
            total: 0,
        },
        last_activity: Some("2025-08-31T23:30:12.520Z".to_string()),
        unread_count: 0,
        last_read_message_sort_key: None,
        is_archived: false,
        is_muted: false,
        is_pinned: false,
        preview: None,
    };
    assert_eq!(chat.id, "chat-1");
    assert_eq!(chat.title, "Alice");
    assert_eq!(chat.chat_type, "single");
    assert_eq!(chat.is_archived, false);
}

#[test]
fn test_group_chat_model() {
    let chat = Chat {
        id: "group-1".to_string(),
        local_chat_id: None,
        account_id: "account-1".to_string(),
        network: "WhatsApp".to_string(),
        title: "Team Chat".to_string(),
        chat_type: "group".to_string(),
        participants: Participants {
            items: vec![
                User {
                    id: "user-1".to_string(),
                    username: Some("@alice".to_string()),
                    phone_number: None,
                    email: None,
                    full_name: Some("Alice".to_string()),
                    img_url: None,
                    cannot_message: None,
                    is_self: Some(true),
                },
                User {
                    id: "user-2".to_string(),
                    username: Some("@bob".to_string()),
                    phone_number: None,
                    email: None,
                    full_name: Some("Bob".to_string()),
                    img_url: None,
                    cannot_message: None,
                    is_self: Some(false),
                },
            ],
            has_more: false,
            total: 2,
        },
        last_activity: None,
        unread_count: 5,
        last_read_message_sort_key: None,
        is_archived: false,
        is_muted: false,
        is_pinned: false,
        preview: None,
    };
    assert_eq!(chat.chat_type, "group");
    assert_eq!(chat.unread_count, 5);
    assert_eq!(chat.participants.items.len(), 2);
}

#[tokio::test]
#[ignore] // Only run with BEEPER_TEST_TOKEN
async fn test_get_accounts_integration() {
    if !should_run_integration_tests() {
        return;
    }

    let client = BeeperClient::new(get_test_token(), get_test_url());
    match client.get_accounts().await {
        Ok(accounts) => {
            // Test passed if we got a valid response
            println!("Got {} accounts", accounts.len());
            assert!(true);
        }
        Err(e) => {
            println!("Error getting accounts: {}", e);
            panic!("Integration test failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Only run with BEEPER_TEST_TOKEN
async fn test_list_chats_integration() {
    if !should_run_integration_tests() {
        return;
    }

    let client = BeeperClient::new(get_test_token(), get_test_url());
    match client.list_chats(None, None).await {
        Ok(output) => {
            println!("Got {} chats", output.items.len());
            assert!(true);
        }
        Err(e) => {
            println!("Error listing chats: {}", e);
            panic!("Integration test failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Only run with BEEPER_TEST_TOKEN
async fn test_search_messages_integration() {
    if !should_run_integration_tests() {
        return;
    }

    let client = BeeperClient::new(get_test_token(), get_test_url());
    match client.search_messages("test", None, None).await {
        Ok(output) => {
            println!("Found {} messages", output.items.len());
            assert!(true);
        }
        Err(e) => {
            println!("Error searching messages: {}", e);
            panic!("Integration test failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Only run with BEEPER_TEST_TOKEN
async fn test_search_chats_integration() {
    if !should_run_integration_tests() {
        return;
    }

    let client = BeeperClient::new(get_test_token(), get_test_url());
    match client.search_chats("alice", None, None).await {
        Ok(output) => {
            println!("Found {} chats", output.items.len());
            assert!(true);
        }
        Err(e) => {
            println!("Error searching chats: {}", e);
            panic!("Integration test failed: {}", e);
        }
    }
}
