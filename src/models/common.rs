//! Common types and utilities

use serde::{Deserialize, Serialize};

pub type ChatID = String;
pub type AccountID = String;
pub type Cursor = String;

/// Pagination direction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Before,
    After,
}
