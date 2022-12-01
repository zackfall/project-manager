use super::build_octo;
use octocrab::{
    models::{Repository, User},
    Result,
};

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    let repo = build_octo().repos(owner, repo).get().await?;
    Ok(repo)
}

pub async fn get_current_user() -> Result<User> {
    let current_user = build_octo().current().user().await?;
    Ok(current_user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_repos_test() {
        let repo: Result<Repository> = get_repo("zackfall", "project-manager").await;
        match repo {
            Ok(rp) => {
                assert_eq!(rp.name, "project-manager");
                assert_eq!(rp.owner.unwrap().login, "zackfall");
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn current_user_test() {
        let user = get_current_user().await;
        match user {
            Ok(usr) => assert_eq!(usr.login, "zackfall".to_owned()),
            Err(_) => panic!("Nicknames don't match"),
        }
    }
}
