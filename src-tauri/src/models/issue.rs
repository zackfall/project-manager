use super::{user::User, IssueId};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Issue {
    pub id: IssueId,
    pub url: Url,
    pub html_url: Url,
    pub repository_url: Url,
    pub comments_url: Url,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub user: User,
    pub comments: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
