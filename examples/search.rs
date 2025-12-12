use beeper_desktop_api::BeeperClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example search");
            std::process::exit(1);
        });

    // Get search query from command line arguments or use default
    let search_query = env::args()
        .nth(1)
        .unwrap_or_else(|| "hello".to_string());

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!("Searching for: \"{}\"", search_query);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Search for chats
    println!("🔍 Searching chats...");
    let chats_search = client.search_chats(&search_query, None, None).await?;
    
    println!("✅ Found {} matching chat(s):", chats_search.items.len());
    println!();

    if chats_search.items.is_empty() {
        println!("No chats found matching \"{}\"", search_query);
    } else {
        for (index, chat) in chats_search.items.iter().enumerate() {
            println!("  {}. {} ({})", index + 1, chat.title, chat.chat_type);
            println!("     └─ Network: {}", chat.network);
            println!("     └─ Participants: {}", chat.participants.total);
            println!("     └─ Unread: {}", chat.unread_count);
            if let Some(last_activity) = &chat.last_activity {
                println!("     └─ Last Activity: {}", last_activity);
            }
            println!();
        }
    }

    // Search for messages
    println!("📨 Searching messages...");
    let messages_search = client.search_messages(&search_query, None, None).await?;
    
    println!("✅ Found {} matching message(s):", messages_search.items.len());
    println!();

    if messages_search.items.is_empty() {
        println!("No messages found matching \"{}\"", search_query);
    } else {
        // Group messages by chat for better display
        let mut chats_in_results = std::collections::HashMap::new();
        
        for message in &messages_search.items {
            chats_in_results
                .entry(message.chat_id.clone())
                .or_insert_with(Vec::new)
                .push(message);
        }

        for (chat_id, messages) in &chats_in_results {
            // Get chat title from search results if available
            let chat_title = if let Some(chat) = messages_search.chats.as_ref()
                .and_then(|chats| chats.get(chat_id))
            {
                chat.title.clone()
            } else {
                chat_id.clone()
            };

            println!("  Chat: {}", chat_title);
            println!("  └─ {} message(s) found:", messages.len());

            for (msg_index, message) in messages.iter().enumerate() {
                let sender = message.sender_name.as_deref().unwrap_or("Unknown");
                let text = message.text.as_deref().unwrap_or("[No text content]");
                
                // Truncate long messages
                let display_text = if text.len() > 60 {
                    format!("{}...", &text[..60])
                } else {
                    text.to_string()
                };

                println!("     {}. [{}] {}: {}", 
                    msg_index + 1,
                    message.timestamp,
                    sender,
                    display_text
                );
            }
            println!();
        }
    }

    // Show pagination info if available
    if messages_search.has_more {
        println!("ℹ️  More results available. Use pagination with:");
        println!("    - oldestCursor: {:?}", messages_search.oldest_cursor);
        println!("    - newestCursor: {:?}", messages_search.newest_cursor);
    }

    // Display search summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Search Summary for \"{}\":", search_query);
    println!("  • Chats: {}", chats_search.items.len());
    println!("  • Messages: {}", messages_search.items.len());
    println!("  • Total Results: {}", chats_search.items.len() + messages_search.items.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
