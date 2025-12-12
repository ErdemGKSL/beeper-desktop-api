use beeper_desktop_api::{BeeperClient, SendMessageInput};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example send_message");
            std::process::exit(1);
        });

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Fetch all chats to find the first non-group chat
    println!("📋 Fetching chats...");
    let chats_response = client.list_chats(None, None).await?;

    if chats_response.items.is_empty() {
        println!("❌ No chats found!");
        return Ok(());
    }

    // Find the first single (non-group) chat
    let single_chat = chats_response
        .items
        .iter()
        .find(|chat| chat.chat_type == "single");

    if let Some(chat) = single_chat {
        let chat_id = &chat.id;
        let chat_title = &chat.title;

        println!("✅ Found {} chats", chats_response.items.len());
        println!();
        println!("💬 Sending message to first single chat: '{}'", chat_title);
        println!("   Chat ID: {}", chat_id);
        println!();

        // Create the message input
        let message_input = SendMessageInput {
            text: "Hello".to_string(),
            reply_to_id: None,
        };

        // Send the message
        let response = client.send_message(chat_id, message_input).await?;

        println!("✅ Message sent successfully!");
        println!();
        println!("Message Details:");
        println!("  Chat ID: {}", response.chat_id);
        println!("  Pending Message ID: {}", response.pending_message_id);
        println!();
        println!("Full response (JSON):");
        println!("{:#?}", response);
    } else {
        println!("❌ No single (non-group) chats found!");
        println!();
        println!("Available chats:");
        for (index, chat) in chats_response.items.iter().enumerate() {
            println!("  {}. {} ({})", index + 1, chat.title, chat.chat_type);
        }
    }

    Ok(())
}
