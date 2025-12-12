//! Rust client library for Beeper Desktop API
//!
//! This library provides a simple and ergonomic interface to interact with the Beeper Desktop API.
//! It handles authentication via bearer tokens and provides methods for managing accounts, chats,
//! messages, and more.
//!
//! # Example
//!
//! ```no_run
//! use beeper_desktop_api::BeeperClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BeeperClient::new("your-token-here", "http://localhost:23373");
//!     let accounts = client.get_accounts().await?;
//!     println!("{:?}", accounts);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod models;
pub mod error;

pub use client::BeeperClient;
pub use error::{BeeperError, Result};
pub use models::*;
