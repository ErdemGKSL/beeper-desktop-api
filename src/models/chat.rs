//! Chat models

use serde::{Deserialize, Deserializer, Serialize};
use super::message::Message;
use super::user::User;

fn deserialize_optional_u64_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U64OrString {
        U64(u64),
        String(String),
    }

    let value = Option::<U64OrString>::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(U64OrString::U64(v)) => Ok(Some(v)),
        Some(U64OrString::String(s)) => s
            .parse::<u64>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// Chat participants with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participants {
    /// List of participants
    pub items: Vec<User>,
    /// Whether there are more participants not included
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Total number of participants in the chat
    pub total: u32,
}

/// A chat or conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    /// Unique chat ID
    pub id: String,
    /// Local chat ID specific to this Beeper Desktop installation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localChatID")]
    pub local_chat_id: Option<String>,
    /// Account ID this chat belongs to, generaly "whatsapp" etc.
    #[serde(rename = "accountID")]
    pub account_id: String,
    /// Display-only human-readable network name (e.g., 'WhatsApp', 'Messenger')
    pub network: String,
    /// Display title of the chat
    pub title: String,
    /// Chat type: 'single' for direct messages, 'group' for group chats
    #[serde(rename = "type")]
    pub chat_type: String,
    /// Chat participants information
    pub participants: Participants,
    /// Timestamp of last activity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastActivity")]
    pub last_activity: Option<String>,
    /// Number of unread messages
    #[serde(rename = "unreadCount")]
    pub unread_count: u32,
    /// Last read message sortKey
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "lastReadMessageSortKey")]
    #[serde(deserialize_with = "deserialize_optional_u64_from_string_or_number")]
    pub last_read_message_sort_key: Option<u64>,
    /// True if chat is archived
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    /// True if chat notifications are muted
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    /// True if chat is pinned
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    /// Last message preview for this chat, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Box<Message>>,
}

impl Chat {
    /// Get a display name for the chat
    /// 
    /// For direct messages ('single'), returns the participant's full name or username.
    /// For group chats, returns the chat title.
    pub fn display_name(&self) -> String {
        if self.chat_type == "single" {
            // For direct messages, try to add the other person's name
            if let Some(first_participant) = self.participants.items.iter().filter(|p| !p.is_self.unwrap_or(false)).next() {
                if let Some(full_name) = &first_participant.full_name {
                    return full_name.clone();
                }
                if let Some(username) = &first_participant.username {
                    return username.clone();
                }
            }
        }
        
        // Return the chat title as-is
        self.title.clone()
    }
}

/// Input for creating a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatInput {
    /// Account ID to create chat on
    #[serde(rename = "accountID")]
    pub account_id: String,
    /// Participant IDs for the chat
    #[serde(rename = "participantIDs")]
    pub participant_ids: Vec<String>,
    /// Optional chat title for group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Output from creating a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatOutput {
    /// Newly created chat ID
    #[serde(rename = "chatID")]
    pub chat_id: String,
}

/// Output from listing chats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChatsOutput {
    /// List of chats
    pub items: Vec<Chat>,
    /// Whether there are more chats
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Cursor for fetching older results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oldestCursor")]
    pub oldest_cursor: Option<String>,
    /// Cursor for fetching newer results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newestCursor")]
    pub newest_cursor: Option<String>,
}

/// Output from searching chats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchChatsOutput {
    /// Matching chats
    pub items: Vec<Chat>,
    /// Map of chat ID -> chat details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<std::collections::HashMap<String, Chat>>,
    /// Whether there are more results
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Cursor for older results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oldestCursor")]
    pub oldest_cursor: Option<String>,
    /// Cursor for newer results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newestCursor")]
    pub newest_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ListChatsOutput;

    fn list_chats_payload_with_sort_key(sort_key_json: &str) -> String {
        format!(
            r#"{{
                "items": [
                    {{
                        "id": "chat-1",
                        "accountID": "account-1",
                        "network": "WhatsApp",
                        "title": "Alice",
                        "type": "single",
                        "participants": {{
                            "items": [],
                            "hasMore": false,
                            "total": 0
                        }},
                        "unreadCount": 0,
                        "lastReadMessageSortKey": {sort_key_json},
                        "isArchived": false,
                        "isMuted": false,
                        "isPinned": false
                    }}
                ],
                "hasMore": false
            }}"#
        )
    }

    #[test]
    fn list_chats_deserializes_numeric_sort_key() {
        let payload = list_chats_payload_with_sort_key("453400065536");
        let output: ListChatsOutput = serde_json::from_str(&payload).expect("should parse");
        assert_eq!(output.items[0].last_read_message_sort_key, Some(453400065536));
    }

    #[test]
    fn list_chats_deserializes_string_sort_key() {
        let payload = list_chats_payload_with_sort_key("\"453400065536\"");
        let output: ListChatsOutput = serde_json::from_str(&payload).expect("should parse");
        assert_eq!(output.items[0].last_read_message_sort_key, Some(453400065536));
    }

    #[test]
    fn list_chats_deserializes_null_sort_key() {
        let payload = list_chats_payload_with_sort_key("null");
        let output: ListChatsOutput = serde_json::from_str(&payload).expect("should parse");
        assert_eq!(output.items[0].last_read_message_sort_key, None);
    }

    #[test]
    fn list_chats_deserializes_missing_sort_key() {
        let payload = r#"{
            "items": [
                {
                    "id": "chat-1",
                    "accountID": "account-1",
                    "network": "WhatsApp",
                    "title": "Alice",
                    "type": "single",
                    "participants": {
                        "items": [],
                        "hasMore": false,
                        "total": 0
                    },
                    "unreadCount": 0,
                    "isArchived": false,
                    "isMuted": false,
                    "isPinned": false
                }
            ],
            "hasMore": false
        }"#;

        let output: ListChatsOutput = serde_json::from_str(payload).expect("should parse");
        assert_eq!(output.items[0].last_read_message_sort_key, None);
    }

    #[test]
    fn list_chats_rejects_invalid_sort_key_string() {
        let payload = list_chats_payload_with_sort_key("\"not-a-number\"");
        let result: Result<ListChatsOutput, _> = serde_json::from_str(&payload);
        assert!(result.is_err());
    }
}
