use super::{user::User, RepositoryId};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    id: RepositoryId,
    name: String,
    full_name: Option<String>,
    owner: Option<User>,
    description: Option<String>,
    private: Option<bool>,
    fork: Option<bool>,
    url: Url,
    hmtl_url: Option<Url>,
    issues_url: Option<Url>,
    size: Option<u32>,
    has_issues: Option<bool>,
    pushed_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
