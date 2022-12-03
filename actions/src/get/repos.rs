use super::super::build_octo;
use octocrab::{models::Repository, Result};

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    let repo = build_octo().repos(owner, repo).get().await?;
    Ok(repo)
}
