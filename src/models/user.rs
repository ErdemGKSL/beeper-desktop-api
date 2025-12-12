//! User and account models

use serde::{Deserialize, Serialize};

/// A user in the Beeper system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,
    /// Human-readable handle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// User's phone number in E.164 format
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    /// User's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    /// Avatar image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imgURL")]
    pub img_url: Option<String>,
    /// True if Beeper cannot initiate messages to this user
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cannotMessage")]
    pub cannot_message: Option<bool>,
    /// True if this is the current user
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isSelf")]
    pub is_self: Option<bool>,
}

/// A chat account connected to Beeper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Chat account ID
    #[serde(rename = "accountID")]
    pub account_id: String,
    /// Network type
    pub network: String,
    /// User associated with this account
    pub user: User,
}

/// Output for get_accounts
pub type GetAccountsOutput = Vec<Account>;
