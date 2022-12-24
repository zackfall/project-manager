use super::{user::User, RepositoryId};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: RepositoryId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmtl_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
