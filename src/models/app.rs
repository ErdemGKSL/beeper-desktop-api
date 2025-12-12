//! App control and asset models

use serde::{Deserialize, Serialize};

/// Input for focusing the app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusAppInput {
    /// Chat ID to navigate to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chatID")]
    pub chat_id: Option<String>,
    /// Message ID to navigate to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageID")]
    pub message_id: Option<String>,
    /// Draft text to pre-fill
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
}

/// Output from focusing the app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusAppOutput {
    /// Was the action successful?
    pub success: bool,
}

/// Input for downloading an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadAssetInput {
    /// URL to download
    pub url: String,
}

/// Output from downloading an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadAssetOutput {
    /// Local file URL
    #[serde(rename = "localURL")]
    pub local_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_app_input_minimal() {
        let input = FocusAppInput {
            chat_id: None,
            message_id: None,
            draft: None,
        };
        assert!(input.chat_id.is_none());
        assert!(input.message_id.is_none());
        assert!(input.draft.is_none());
    }

    #[test]
    fn test_focus_app_input_with_chat() {
        let input = FocusAppInput {
            chat_id: Some("chat-1".to_string()),
            message_id: None,
            draft: None,
        };
        assert_eq!(input.chat_id, Some("chat-1".to_string()));
        assert!(input.message_id.is_none());
    }

    #[test]
    fn test_focus_app_input_with_message() {
        let input = FocusAppInput {
            chat_id: Some("chat-1".to_string()),
            message_id: Some("msg-123".to_string()),
            draft: None,
        };
        assert_eq!(input.chat_id, Some("chat-1".to_string()));
        assert_eq!(input.message_id, Some("msg-123".to_string()));
    }

    #[test]
    fn test_focus_app_input_with_draft() {
        let input = FocusAppInput {
            chat_id: Some("chat-1".to_string()),
            message_id: None,
            draft: Some("Hello, world!".to_string()),
        };
        assert_eq!(input.draft, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_focus_app_output_success() {
        let output = FocusAppOutput { success: true };
        assert!(output.success);
    }

    #[test]
    fn test_focus_app_output_failure() {
        let output = FocusAppOutput { success: false };
        assert!(!output.success);
    }

    #[test]
    fn test_download_asset_input() {
        let input = DownloadAssetInput {
            url: "https://example.com/file.png".to_string(),
        };
        assert_eq!(input.url, "https://example.com/file.png");
    }

    #[test]
    fn test_download_asset_output() {
        let output = DownloadAssetOutput {
            local_url: "file:///home/user/.beeper/cache/file.png".to_string(),
        };
        assert!(output.local_url.contains("file://"));
    }

    #[test]
    fn test_focus_app_input_serialization() {
        let input = FocusAppInput {
            chat_id: Some("chat-1".to_string()),
            message_id: Some("msg-1".to_string()),
            draft: Some("Test draft".to_string()),
        };
        let json = serde_json::to_string(&input).expect("Failed to serialize");
        assert!(json.contains("\"chatID\""));
        assert!(json.contains("\"messageID\""));
    }

    #[test]
    fn test_download_asset_input_serialization() {
        let input = DownloadAssetInput {
            url: "https://example.com/asset.jpg".to_string(),
        };
        let json = serde_json::to_string(&input).expect("Failed to serialize");
        assert!(json.contains("https://example.com/asset.jpg"));
    }
}
