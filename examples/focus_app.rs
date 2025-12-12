use beeper_desktop_api::{BeeperClient, FocusAppInput};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get authentication token from environment variable
    let token = env::var("BEEPER_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("Error: BEEPER_TOKEN environment variable not set");
            eprintln!("Usage: BEEPER_TOKEN=your_token cargo run --example focus_app [chat_id] [message_id] [draft_text]");
            std::process::exit(1);
        });

    // Get API base URL from environment variable or use default
    let base_url = env::var("BEEPER_API_URL")
        .unwrap_or_else(|_| "http://localhost:23373".to_string());

    println!("Connecting to Beeper Desktop API at: {}", base_url);
    println!();

    // Create a client with the provided token
    let client = BeeperClient::new(&token, &base_url);

    // Get optional arguments
    let args: Vec<String> = env::args().collect();
    let chat_id = args.get(1).cloned();
    let message_id = args.get(2).cloned();
    let draft_text = args.get(3).cloned();

    // Create focus input based on arguments
    let focus_input = if chat_id.is_some() || message_id.is_some() || draft_text.is_some() {
        Some(FocusAppInput {
            chat_id: chat_id.clone(),
            message_id: message_id.clone(),
            draft: draft_text.clone(),
        })
    } else {
        None
    };

    // Determine what we're doing
    if let Some(ref input) = focus_input {
        println!("🔍 Focusing Beeper with parameters:");
        if let Some(ref cid) = input.chat_id {
            println!("  Chat ID: {}", cid);
        }
        if let Some(ref mid) = input.message_id {
            println!("  Message ID: {}", mid);
        }
        if let Some(ref draft) = input.draft {
            println!("  Draft: {}", draft);
        }
        println!();
    } else {
        println!("🎯 Focusing Beeper Desktop (no parameters)");
        println!();
    }

    // Focus the app
    match client.focus_app(focus_input).await {
        Ok(response) => {
            if response.success {
                println!("✅ Successfully focused Beeper Desktop!");
                println!();
                if draft_text.is_some() {
                    println!("💬 Draft text has been pre-filled in the chat");
                }
                if chat_id.is_some() {
                    println!("📍 Navigated to the specified chat");
                }
                if message_id.is_some() {
                    println!("💭 Navigated to the specified message");
                }
            } else {
                println!("⚠️  Focus operation completed but returned success: false");
            }
            println!();
            println!("Response:");
            println!("{:#?}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
