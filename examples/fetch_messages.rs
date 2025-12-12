use beeper_desktop_api::BeeperClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example fetch_messages");
            std::process::exit(1);
        });

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Fetch all chats to find the first one
    println!("📋 Fetching chats...");
    let chats_response = client.list_chats(None, None).await?;

    if chats_response.items.is_empty() {
        println!("❌ No chats found!");
        return Ok(());
    }

    // Get the first chat
    let first_chat = &chats_response.items[0];
    let chat_id = &first_chat.id;
    let chat_title = &first_chat.title;

    println!("✅ Found {} chats", chats_response.items.len());
    println!();
    println!("📬 Fetching messages from first chat: '{}'", chat_title);
    println!("   Chat ID: {}", chat_id);
    println!();

    // Fetch messages from the first chat
    let messages_response = client.list_messages(chat_id, None, None).await?;

    println!("✅ Successfully retrieved {} messages:", messages_response.items.len());
    println!();

    if messages_response.items.is_empty() {
        println!("No messages found in this chat.");
    } else {
        // Display messages with formatting
        for (index, message) in messages_response.items.iter().enumerate() {
            let sender = message.sender_name.as_deref().unwrap_or("Unknown");
            let text = message.text.as_deref().unwrap_or("[No text content]");
            let timestamp = &message.timestamp;

            // Show attachments count if any
            let attachment_info = if let Some(attachments) = &message.attachments {
                format!(" [+{} attachment(s)]", attachments.len())
            } else {
                String::new()
            };

            // Show reactions count if any
            let reaction_info = if let Some(reactions) = &message.reactions {
                format!(" [+{} reaction(s)]", reactions.len())
            } else {
                String::new()
            };

            // Show reply indicator if replying to another message
            let reply_info = if message.reply_to_id.is_some() {
                " [↩️ Reply]".to_string()
            } else {
                String::new()
            };

            println!("  {}. [{}] {}: {}{}{}{}", 
                index + 1, 
                timestamp, 
                sender, 
                text,
                attachment_info,
                reaction_info,
                reply_info
            );
        }
    }

    println!();
    println!("Message details (JSON):");
    println!("{:#?}", messages_response.items);

    // Show pagination info if available
    if messages_response.has_more {
        println!();
        println!("ℹ️  More messages available. Use pagination to fetch older/newer messages.");
    }

    Ok(())
}
