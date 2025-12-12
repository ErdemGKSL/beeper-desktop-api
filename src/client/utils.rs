//! Utility functions for API operations

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use crate::error::{BeeperError, Result};
use super::ApiErrorResponse;

pub(super) fn map_request_error(error: reqwest::Error, base_url: &str) -> BeeperError {
    if error.is_connect() {
        BeeperError::ApiNotReachable {
            url: base_url.to_string(),
        }
    } else {
        BeeperError::RequestError(error)
    }
}

pub(super) async fn handle_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED => {
            let data = response.json::<T>().await?;
            Ok(data)
        }
        StatusCode::BAD_REQUEST => {
            let error = response.json::<ApiErrorResponse>().await?;
            Err(BeeperError::ApiError {
                code: error.code,
                message: error.message,
            })
        }
        StatusCode::UNAUTHORIZED => {
            Err(BeeperError::InvalidConfig(
                "Unauthorized - check your bearer token".to_string(),
            ))
        }
        StatusCode::FORBIDDEN => {
            let error = response.json::<ApiErrorResponse>().await?;
            Err(BeeperError::ApiError {
                code: error.code,
                message: error.message,
            })
        }
        StatusCode::NOT_FOUND => {
            Err(BeeperError::InvalidConfig("Resource not found".to_string()))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            Err(BeeperError::InvalidConfig(
                "Rate limit exceeded".to_string(),
            ))
        }
        status => {
            let text = response.text().await.unwrap_or_default();
            Err(BeeperError::ApiError {
                code: status.to_string(),
                message: text,
            })
        }
    }
}
