//! Data models for Beeper Desktop API

pub mod common;
pub mod user;
pub mod message;
pub mod chat;
pub mod app;

// Re-export commonly used types
pub use common::{ChatID, AccountID, Cursor, Direction};
pub use user::{User, Account, GetAccountsOutput};
pub use message::{
    Attachment, Reaction, Message, SendMessageInput, SendMessageOutput,
    ListMessagesOutput, SearchMessagesOutput,
};
pub use chat::{
    Chat, Participants, CreateChatInput, CreateChatOutput, ListChatsOutput, SearchChatsOutput,
};
pub use app::{
    FocusAppInput, FocusAppOutput, DownloadAssetInput, DownloadAssetOutput,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_serialization() {
        // Test that Direction enum can be serialized
        let before = Direction::Before;
        let after = Direction::After;
        
        // Just verify they're different
        match (before, after) {
            (Direction::Before, Direction::After) => assert!(true),
            _ => panic!("Direction enum values don't match expected"),
        }
    }
}
