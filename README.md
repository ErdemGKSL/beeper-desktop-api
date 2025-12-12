# Beeper Desktop API - Rust Client Library

A type-safe, ergonomic Rust client library for the [Beeper Desktop API](https://developers.beeper.com/desktop-api). Manage chats, messages, and accounts across multiple messaging networks (WhatsApp, Telegram, Signal, Matrix, and more) through a unified interface.

## Features

- 🔐 **Authentication**: Bearer token and OAuth2 PKCE flow support
- 💬 **Chat Management**: List, create, search, and archive chats across all networks
- 📨 **Message Operations**: Send, fetch, and search messages with pagination support
- 👥 **Account Management**: List and manage connected messaging accounts
- 🎯 **Type-Safe**: Full type safety with serde serialization/deserialization
- ⚡ **Async**: Built on tokio for high-performance async operations
- 🧪 **Well-Tested**: Comprehensive unit and integration tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
beeper_desktop_api = { version = "0.1.1" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### 1. Create a Client

```rust
use beeper_desktop_api::BeeperClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with token and default URL (http://localhost:23373)
    let client = BeeperClient::new("your-token-here", "http://localhost:23373");
    
    Ok(())
}
```

### 2. Fetch Accounts

```rust
// List all connected messaging accounts
let accounts = client.get_accounts().await?;

for account in accounts {
    println!("Account: {} ({})", account.user.full_name.unwrap_or_default(), account.network);
    println!("  User ID: {}", account.user.id);
    println!("  Account ID: {}", account.account_id);
}
```

### 3. Fetch Chats

```rust
// List all chats
let chats_response = client.list_chats(None, None).await?;

for chat in &chats_response.items {
    println!("Chat: {} ({} type)", chat.title, chat.chat_type);
    println!("  Unread: {}", chat.unread_count);
    println!("  Last Activity: {:?}", chat.last_activity);
}
```

### 4. Fetch Messages

```rust
// Get first chat
if let Some(chat) = chats_response.items.first() {
    // Fetch messages from this chat
    let messages_response = client.list_messages(&chat.id, None, None).await?;
    
    for message in &messages_response.items {
        println!("[{}] {}: {}", 
            message.timestamp, 
            message.sender_name.as_deref().unwrap_or("Unknown"),
            message.text.as_deref().unwrap_or("[No text]")
        );
    }
}
```

## Examples

Ready-to-run examples are provided in the `examples/` directory:

### Run Examples

Set environment variables and run:

```bash
export BEEPER_TOKEN="your-token-here"
export BEEPER_API_URL="http://localhost:23373"  # optional, uses this by default

# Fetch and display all accounts
cargo run --example fetch_accounts

# Fetch and display all chats
cargo run --example fetch_chats

# Fetch and display messages from the first chat
cargo run --example fetch_messages

# Search chats and messages
cargo run --example search "your-search-query"
# Or use default search term
cargo run --example search
```

### Example Output

#### `fetch_accounts`
```
👤 Fetching connected accounts...
✅ Successfully retrieved 3 account(s):

  1. Beeper (Matrix) Account
     └─ User: Erdem Göksel
     └─ Account ID: hungryserv
     └─ User ID: @erdemdev:beeper.com
     └─ Phone: +905325965386
     └─ Email: zahrakon06@gmail.com

  2. Instagram Account
     └─ User: myinsta
     └─ Account ID: instagramgo
     └─ User ID: 17842413400511926

  3. WhatsApp Account
     └─ User: erdem
     └─ Account ID: whatsapp
     └─ User ID: 905325965386
```

#### `fetch_chats`
```
📋 Fetching chats from Beeper...
✅ Successfully retrieved 25 chats:

  1. Beeper Developer Community
  2. DM: Annem
  3. DM: umut
  4. DM: ozgen
  ...
```

#### `fetch_messages`
```
📬 Fetching messages from first chat: 'Beeper Developer Community'

✅ Successfully retrieved 20 messages:

  1. [2025-12-12T09:36:18.075Z] erdemdev: i will send its link when its finished
  2. [2025-12-12T09:36:10.876Z] erdemdev: also i am currently making and api wrapper for rust
  3. [2025-12-12T09:33:26.464Z] erdemdev: i just want to setup main beeper communication at my server
  ...
```

#### `search`
```
Searching for: "api"

🔍 Searching chats...
✅ Found 50 matching chat(s):

  1. Beeper Developer Community (group)
     └─ Network: Beeper (Matrix)
     └─ Participants: 345
     └─ Unread: 0

  2. @erdemdev:beeper.com (single)
     └─ Network: WhatsApp
     └─ Participants: 2
     └─ Unread: 0

  3. Annem (single)
     └─ Network: WhatsApp
     └─ Participants: 2
     └─ Unread: 0

📨 Searching messages...
✅ Found 20 matching message(s):

  Chat: Beeper Developer Community
  └─ 8 message(s) found:
     1. [2025-12-12T09:36:18.075Z] erdemdev: i will send its link when its finished
     2. [2025-12-12T09:36:10.876Z] erdemdev: also i am currently making and api wrapper for rust
     3. [2025-12-12T09:33:26.464Z] erdemdev: i just want to setup main beeper communication...

📊 Search Summary for "api":
  • Chats: 50
  • Messages: 20
  • Total Results: 70
```

## API Overview

### Core Types

#### Account
Represents a connected messaging account.

```rust
pub struct Account {
    pub account_id: String,      // e.g., "whatsapp", "telegram"
    pub network: String,          // Display name (e.g., "WhatsApp")
    pub user: User,              // User info
}
```

#### Chat
Represents a conversation (direct message or group).

```rust
pub struct Chat {
    pub id: String,              // Unique chat ID
    pub account_id: String,      // Account this chat belongs to
    pub title: String,           // Chat name/display title
    pub chat_type: String,       // "single" or "group"
    pub participants: Participants, // Chat members
    pub unread_count: u32,       // Unread message count
    pub last_activity: Option<String>, // ISO 8601 timestamp
    pub is_archived: bool,
    pub is_muted: bool,
    pub is_pinned: bool,
}
```

#### Message
Represents a message in a chat.

```rust
pub struct Message {
    pub id: String,              // Message ID
    pub chat_id: String,         // Which chat this is in
    pub sender_id: String,       // Who sent it
    pub sender_name: Option<String>, // Display name
    pub text: Option<String>,    // Message content
    pub timestamp: String,       // ISO 8601 timestamp
    pub attachments: Option<Vec<Attachment>>, // Media files
    pub reactions: Option<Vec<Reaction>>, // Emoji reactions
    pub is_sender: Option<bool>, // True if current user sent it
    pub is_unread: Option<bool>, // Unread status
}
```

#### User
Represents a person on the messaging platform.

```rust
pub struct User {
    pub id: String,              // Stable user ID
    pub username: Option<String>, // Handle (e.g., "@alice")
    pub phone_number: Option<String>, // E.164 format
    pub email: Option<String>,
    pub full_name: Option<String>, // Display name
    pub img_url: Option<String>,  // Avatar URL
    pub is_self: Option<bool>,   // True if current user
    pub cannot_message: Option<bool>, // Can't initiate messages
}
```

### Client Methods

#### Account Operations

```rust
// Get all connected accounts
let accounts: Vec<Account> = client.get_accounts().await?;
```

#### Chat Operations

```rust
// List all chats with pagination
let chats = client.list_chats(cursor: Option<&str>, direction: Option<&str>).await?;

// Get a specific chat
let chat: Chat = client.get_chat(chat_id: &str).await?;

// Create a new chat
let input = CreateChatInput {
    account_id: "whatsapp".to_string(),
    participant_ids: vec!["user-id-1".to_string()],
    title: Some("My Chat".to_string()),
};
let output = client.create_chat(input).await?;

// Archive/unarchive a chat
let chat = client.archive_chat(chat_id: &str, archived: true).await?;

// Search chats
let results = client.search_chats(query: &str).await?;
```

#### Message Operations

```rust
// List messages in a chat with pagination
let messages = client.list_messages(
    chat_id: &str,
    cursor: Option<&str>,
    direction: Option<&str>
).await?;

// Send a message
let input = SendMessageInput {
    text: "Hello!".to_string(),
    reply_to_id: None,
};
let output = client.send_message(chat_id: &str, input).await?;

// Search for messages with pagination
let results = client.search_messages(
    query: &str,
    cursor: Option<&str>,
    direction: Option<&str>
).await?;

// Search for chats with pagination
let results = client.search_chats(
    query: &str,
    cursor: Option<&str>,
    direction: Option<&str>
).await?;
```

## Authentication

### Bearer Token

Get a token from Beeper Desktop (Settings > API) and use it:

```rust
let client = BeeperClient::new("your-token", "http://localhost:23373");
```

### OAuth2 PKCE Flow

For MCP servers and applications requiring user authorization:

```
GET http://localhost:23373/oauth/authorize?
  client_id=your-client-id&
  response_type=code&
  scope=read%20write&
  redirect_uri=http://localhost:3000/callback
```

Exchange authorization code for token:

```
POST http://localhost:23373/oauth/token
  code=authorization-code&
  client_id=your-client-id&
  grant_type=authorization_code&
  redirect_uri=http://localhost:3000/callback
```

## Pagination

Many list endpoints support pagination:

```rust
// Get first page
let page1 = client.list_chats(None, None).await?;

// Use cursor and direction for next page
if page1.has_more {
    let page2 = client.list_chats(
        Some(&page1.newest_cursor.unwrap()),
        Some("after")
    ).await?;
}
```

For messages, use `message.sort_key` as the cursor:

```rust
let messages = client.list_messages(chat_id, None, None).await?;

if messages.has_more {
    let older_messages = client.list_messages(
        chat_id,
        Some(&messages.items.last().unwrap().sort_key),
        Some("before")
    ).await?;
}
```

## Error Handling

All operations return `Result<T, BeeperError>`:

```rust
match client.list_chats(None, None).await {
    Ok(chats) => println!("Got {} chats", chats.items.len()),
    Err(BeeperError::Unauthorized) => eprintln!("Invalid token"),
    Err(BeeperError::NotFound) => eprintln!("Resource not found"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Development

### Building

```bash
# Build library
cargo build

# Build with optimizations
cargo build --release

# Build examples
cargo build --examples
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests (requires BEEPER_TEST_TOKEN env var)
BEEPER_TEST_TOKEN="token" cargo test --test integration_tests
```

### Code Quality

The library follows Rust best practices:
- Comprehensive error handling
- Type-safe abstractions
- Proper use of Options and Results
- Async/await patterns
- Idiomatic Rust naming conventions

## Supported Networks

Beeper Desktop API provides access to these messaging networks:

- WhatsApp
- Telegram
- Signal
- Matrix
- Instagram (Direct Messages)
- Twitter/X (Direct Messages)
- Slack
- Discord
- Email (experimental)
- And more...

## API Reference

For detailed API documentation, see:
- [Beeper Desktop API Docs](https://developers.beeper.com/desktop-api)
- [OpenAPI Schema](docs/beeper-api.json)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Changes include tests and documentation

## Support

- API Issues: [Beeper Support](https://support.beeper.com)
- Library Issues: Open an issue on GitHub
- Documentation: See `/docs` folder and inline code comments

## Changelog

### v0.1.0 (2025-12-12)
- Initial release
- Support for accounts, chats, and messages
- Bearer token authentication
- Pagination support
- Three working examples (fetch_accounts, fetch_chats, fetch_messages)
- Comprehensive test suite
