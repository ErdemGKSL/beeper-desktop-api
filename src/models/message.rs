//! Message, attachment, and reaction models

use serde::{Deserialize, Serialize};

/// File attachment or media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Attachment type
    #[serde(rename = "type")]
    pub typ: String,
    /// Public URL or local file path
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "srcURL")]
    pub src_url: Option<String>,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    /// Original filename
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileSize")]
    pub file_size: Option<u64>,
    /// True if GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isGif")]
    pub is_gif: Option<bool>,
    /// True if sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isSticker")]
    pub is_sticker: Option<bool>,
    /// True if voice note
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isVoiceNote")]
    pub is_voice_note: Option<bool>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Preview image URL for videos
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "posterImg")]
    pub poster_img: Option<String>,
}

/// Emoji reaction to a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// Reaction ID
    pub id: String,
    /// The reaction key (emoji or shortcode)
    #[serde(rename = "reactionKey")]
    pub reaction_key: String,
    /// URL to reaction image
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imgURL")]
    pub img_url: Option<String>,
    /// User ID of participant who reacted
    #[serde(rename = "participantID")]
    pub participant_id: String,
    /// True if the reactionKey is an emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
}

/// A message in a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: String,
    /// Chat ID this message belongs to
    #[serde(rename = "chatID")]
    pub chat_id: String,
    /// Account ID the message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accountID")]
    pub account_id: Option<String>,
    /// Sender user ID
    #[serde(rename = "senderID")]
    pub sender_id: String,
    /// Sender display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "senderName")]
    pub sender_name: Option<String>,
    /// Message text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Timestamp in ISO 8601 format
    pub timestamp: String,
    /// Sort key for pagination
    #[serde(rename = "sortKey")]
    pub sort_key: String,
    /// Is this message edited?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isEdited")]
    pub is_edited: Option<bool>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// True if the message is unread
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isUnread")]
    pub is_unread: Option<bool>,
    /// Reactions to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<Reaction>>,
    /// Message ID this message replies to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "replyToID")]
    pub reply_to_id: Option<String>,
    /// Is this message from the current user (for previews)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isSender")]
    pub is_sender: Option<bool>,
}

/// Input for sending a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageInput {
    /// Message text
    pub text: String,
    /// ID of message to reply to (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "replyToID")]
    pub reply_to_id: Option<String>,
}

/// Output from sending a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    /// Chat ID where message was sent
    #[serde(rename = "chatID")]
    pub chat_id: String,
    /// Pending message ID
    #[serde(rename = "pendingMessageID")]
    pub pending_message_id: String,
}

/// Output from listing messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMessagesOutput {
    /// Messages in the chat
    pub items: Vec<Message>,
    /// Whether there are more results
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

/// Output from searching messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesOutput {
    /// Matching messages
    pub items: Vec<Message>,
    /// Map of chat ID -> chat details for chats referenced in items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<std::collections::HashMap<String, crate::models::Chat>>,
    /// Whether there are more results
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Cursor for older messages
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oldestCursor")]
    pub oldest_cursor: Option<String>,
    /// Cursor for newer messages
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newestCursor")]
    pub newest_cursor: Option<String>,
}
