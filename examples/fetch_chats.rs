use beeper_desktop_api::BeeperClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example fetch_chats");
            std::process::exit(1);
        });

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Fetch all chats
    println!("📋 Fetching chats from Beeper...");
    let chats_response = client.list_chats(None, None).await?;

    // Extract chat names into a vector using the display_name method
    let chat_names: Vec<String> = chats_response
        .items
        .iter()
        .map(|chat| chat.display_name())
        .collect();

    // Display results
    println!("✅ Successfully retrieved {} chats:", chat_names.len());
    println!();

    if chat_names.is_empty() {
        println!("No chats found.");
    } else {
        for (index, name) in chat_names.iter().enumerate() {
            println!("  {}. {}", index + 1, name);
        }
    }

    println!();
    println!("Chat names as array:");
    println!("{:#?}", chat_names);

    Ok(())
}
