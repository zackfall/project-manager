use super::super::build_octo;
use octocrab::{models::Repository, Page, Result};

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    let repo = build_octo().repos(owner, repo).get().await?;
    Ok(repo)
}

pub async fn get_repos(nickname: Option<&str>) -> Result<Page<Repository>> {
    let mut listed_repos: Page<Repository> = build_octo()
        .current()
        .list_repos_for_authenticated_user()
        .per_page(50)
        .page(1)
        .send()
        .await?;
    if let Some(nick) = nickname {
        let vec_repos: Vec<Repository> = listed_repos
            .take_items()
            .into_iter()
            .filter(|repo| {
                repo.html_url
                    .as_ref()
                    .unwrap()
                    .as_str()
                    .starts_with(format!("https://github.com/{}", nick).as_str())
            })
            .collect();
        listed_repos.items = vec_repos;
    }
    Ok(listed_repos)
}
