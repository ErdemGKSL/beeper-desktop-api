//! Search-related API operations

use crate::models::{SearchChatsOutput, SearchMessagesOutput};
use crate::error::Result;
use super::{BeeperClient, handle_response};

impl BeeperClient {
    /// Searches messages across chats
    ///
    /// Uses Beeper's message index for full-text search.
    pub async fn search_messages(
        &self,
        query: &str,
        cursor: Option<&str>,
        direction: Option<&str>,
    ) -> Result<SearchMessagesOutput> {
        let mut url = format!("{}/v1/messages/search?q={}", self.get_base_url(), urlencoding::encode(query));

        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", urlencoding::encode(c)));
        }

        if let Some(d) = direction {
            url.push_str(&format!("&direction={}", d));
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

    /// Searches chats by title, network, or participants
    ///
    /// Uses Beeper Desktop's renderer algorithm.
    pub async fn search_chats(
        &self,
        query: &str,
        cursor: Option<&str>,
        direction: Option<&str>,
    ) -> Result<SearchChatsOutput> {
        let mut url = format!("{}/v1/chats/search?q={}", self.get_base_url(), urlencoding::encode(query));

        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", urlencoding::encode(c)));
        }

        if let Some(d) = direction {
            url.push_str(&format!("&direction={}", d));
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
}
