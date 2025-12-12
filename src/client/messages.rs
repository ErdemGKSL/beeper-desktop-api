//! Message-related API operations

use crate::models::{ListMessagesOutput, SendMessageInput, SendMessageOutput};
use crate::error::Result;
use super::{BeeperClient, handle_response};

impl BeeperClient {
    /// Lists all messages in a chat
    ///
    /// Paginated message list sorted by timestamp.
    pub async fn list_messages(
        &self,
        chat_id: &str,
        cursor: Option<&str>,
        direction: Option<&str>,
    ) -> Result<ListMessagesOutput> {
        let mut url = format!(
            "{}/v1/chats/{}/messages",
            self.get_base_url(),
            urlencoding::encode(chat_id)
        );

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
            .await
            .map_err(|e| super::utils::map_request_error(e, self.get_base_url()))?;

        handle_response(response).await
    }

    /// Sends a message to a chat
    ///
    /// Sends a text message to a specific chat. Supports replying to existing messages.
    /// Returns the sent message ID.
    pub async fn send_message(&self, chat_id: &str, input: SendMessageInput) -> Result<SendMessageOutput> {
        let url = format!(
            "{}/v1/chats/{}/messages",
            self.get_base_url(),
            urlencoding::encode(chat_id)
        );

        let response = self
            .get_http_client()
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .json(&input)
            .send()
            .await
            .map_err(|e| super::utils::map_request_error(e, self.get_base_url()))?;

        handle_response(response).await
    }
}
