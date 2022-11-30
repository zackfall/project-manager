use octocrab::{models::Repository, Result};

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    octocrab::instance().repos(owner, repo).get().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn get_repos_test() {
        let repo: Result<Repository> = get_repo("zackfall", "project-manager").await;
        match repo {
            Ok(rp) => assert_eq!(
                rp.url,
                Url::parse("https://api.github.com/repos/zackfall/project-manager").unwrap()
            ),
            Err(err) => panic!("{}", err),
        }
    }
}
