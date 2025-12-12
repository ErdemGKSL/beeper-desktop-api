//! Chat-related API operations

use crate::models::{Chat, CreateChatInput, CreateChatOutput, ListChatsOutput};
use crate::error::Result;
use super::{BeeperClient, handle_response};

impl BeeperClient {
    /// Lists all chats sorted by last activity
    ///
    /// Combines all accounts into a single paginated list.
    pub async fn list_chats(
        &self,
        cursor: Option<&str>,
        direction: Option<&str>,
    ) -> Result<ListChatsOutput> {
        let mut url = format!("{}/v1/chats", self.get_base_url());

        if let Some(c) = cursor {
            url.push_str(&format!("?cursor={}", urlencoding::encode(c)));
            if direction.is_some() {
                url.push('&');
            }
        } else if direction.is_some() {
            url.push('?');
        }

        if let Some(d) = direction {
            url.push_str(&format!("direction={}", d));
        }

        let response = self
            .get_http_client()
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        handle_response(response).await
    }

    /// Retrieves details for a specific chat
    ///
    /// Returns chat metadata, participants, and latest message
    pub async fn get_chat(&self, chat_id: &str) -> Result<Chat> {
        let url = format!("{}/v1/chats/{}", self.get_base_url(), urlencoding::encode(chat_id));
        let response = self
            .get_http_client()
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        handle_response(response).await
    }

    /// Creates a new chat
    ///
    /// Creates a single or group chat on a specific account using participant IDs
    pub async fn create_chat(&self, input: CreateChatInput) -> Result<CreateChatOutput> {
        let url = format!("{}/v1/chats", self.get_base_url());
        let response = self
            .get_http_client()
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .json(&input)
            .send()
            .await?;

        handle_response(response).await
    }

    /// Archives or unarchives a chat
    pub async fn archive_chat(&self, chat_id: &str, archived: bool) -> Result<Chat> {
        let url = format!("{}/v1/chats/{}/archive", self.get_base_url(), urlencoding::encode(chat_id));
        let body = serde_json::json!({ "archived": archived });

        let response = self
            .get_http_client()
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }

    /// Sets a reminder for a chat
    pub async fn set_chat_reminder(&self, chat_id: &str, timestamp: &str) -> Result<Chat> {
        let url = format!(
            "{}/v1/chats/{}/reminders",
            self.get_base_url(),
            urlencoding::encode(chat_id)
        );
        let body = serde_json::json!({ "timestamp": timestamp });

        let response = self
            .get_http_client()
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }

    /// Clears a reminder from a chat
    pub async fn clear_chat_reminder(&self, chat_id: &str) -> Result<Chat> {
        let url = format!(
            "{}/v1/chats/{}/reminders",
            self.get_base_url(),
            urlencoding::encode(chat_id)
        );

        let response = self
            .get_http_client()
            .delete(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        handle_response(response).await
    }
}
