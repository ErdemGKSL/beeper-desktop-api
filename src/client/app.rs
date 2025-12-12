//! App control and asset operations

use crate::models::{DownloadAssetInput, DownloadAssetOutput, FocusAppInput, FocusAppOutput};
use crate::error::Result;
use super::{BeeperClient, handle_response};

impl BeeperClient {
    /// Focuses Beeper Desktop and optionally navigates to a specific location
    pub async fn focus_app(&self, input: Option<FocusAppInput>) -> Result<FocusAppOutput> {
        let url = format!("{}/v1/focus", self.get_base_url());

        let response = if let Some(inp) = input {
            self.get_http_client()
                .post(&url)
                .header("Authorization", self.get_auth_header())
                .json(&inp)
                .send()
                .await?
        } else {
            self.get_http_client()
                .post(&url)
                .header("Authorization", self.get_auth_header())
                .json(&serde_json::json!({}))
                .send()
                .await?
        };

        handle_response(response).await
    }

    /// Downloads an asset from a URL
    ///
    /// Downloads a Matrix asset using its mxc:// or localmxc:// URL to the device
    /// running Beeper Desktop and returns the local file URL.
    pub async fn download_asset(&self, url: &str) -> Result<DownloadAssetOutput> {
        let endpoint_url = format!("{}/v1/assets/download", self.get_base_url());
        let input = DownloadAssetInput {
            url: url.to_string(),
        };

        let response = self
            .get_http_client()
            .post(&endpoint_url)
            .header("Authorization", self.get_auth_header())
            .json(&input)
            .send()
            .await?;

        handle_response(response).await
    }
}
