use beeper_desktop_api::BeeperClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example fetch_accounts");
            std::process::exit(1);
        });

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Fetch all accounts
    println!("👤 Fetching connected accounts...");
    let accounts = client.get_accounts().await?;

    println!("✅ Successfully retrieved {} account(s):", accounts.len());
    println!();

    if accounts.is_empty() {
        println!("No accounts connected.");
    } else {
        // Display accounts with formatting
        for (index, account) in accounts.iter().enumerate() {
            let account_id = &account.account_id;
            let network = &account.network;
            let user = &account.user;

            // Build user identifier
            let user_identifier = if let Some(full_name) = &user.full_name {
                full_name.clone()
            } else if let Some(username) = &user.username {
                format!("@{}", username)
            } else if let Some(phone) = &user.phone_number {
                phone.clone()
            } else {
                user.id.clone()
            };

            println!("  {}. {} Account", index + 1, network);
            println!("     └─ User: {}", user_identifier);
            println!("     └─ Account ID: {}", account_id);
            println!("     └─ User ID: {}", user.id);

            // Show additional user info if available
            if let Some(username) = &user.username {
                println!("     └─ Username: @{}", username);
            }
            if let Some(phone) = &user.phone_number {
                println!("     └─ Phone: {}", phone);
            }
            if let Some(email) = &user.email {
                println!("     └─ Email: {}", email);
            }
            if let Some(cannot_message) = user.cannot_message {
                if cannot_message {
                    println!("     └─ ⚠️  Cannot initiate messages to this user");
                }
            }

            println!();
        }
    }

    println!("Account details (JSON):");
    println!("{:#?}", accounts);

    Ok(())
}
