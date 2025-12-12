//! Account-related API operations

use crate::models::GetAccountsOutput;
use crate::error::Result;
use super::{BeeperClient, handle_response};

impl BeeperClient {
    /// Lists all connected messaging accounts
    ///
    /// Lists chat accounts across networks (WhatsApp, Telegram, Twitter/X, etc.)
    /// actively connected to this Beeper Desktop instance
    pub async fn get_accounts(&self) -> Result<GetAccountsOutput> {
        let url = format!("{}/v1/accounts", self.get_base_url());
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
