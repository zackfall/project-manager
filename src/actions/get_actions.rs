use super::build_octo;
use octocrab::{
    models::{issues::Issue, Repository, User},
    params::{self, issues::Filter},
    Page, Result,
};

pub async fn get_current_user() -> Result<User> {
    let current_user = build_octo().current().user().await?;
    Ok(current_user)
}

pub async fn get_issues(owner: &str, repo: &str) -> Result<Page<Issue>> {
    let issues = build_octo()
        .issues(owner, repo)
        .list()
        .state(params::State::All)
        .per_page(15)
        .page(1u8)
        .send()
        .await?;
    Ok(issues)
}

pub async fn get_issues_by_assignee(
    owner: &str,
    repo: &str,
    assignee: impl Into<Filter<&str>>,
) -> Result<Page<Issue>> {
    let assignee = build_octo()
        .issues(owner, repo)
        .list()
        .assignee(assignee.into())
        .state(params::State::All)
        .per_page(15)
        .page(1u8)
        .send()
        .await?;
    Ok(assignee)
}

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    let repo = build_octo().repos(owner, repo).get().await?;
    Ok(repo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    struct Data {
        owner: String,
        repo: String,
    }

    impl Data {
        pub fn new() -> Data {
            dotenv().ok();
            Data {
                owner: "zackfall".to_owned(),
                repo: "project-manager".to_owned(),
            }
        }
    }

    #[tokio::test]
    async fn get_repos_test() {
        let data = Data::new();
        let repo: Result<Repository> = get_repo(data.owner.as_str(), data.repo.as_str()).await;
        match repo {
            Ok(rp) => {
                assert_eq!(rp.name, data.repo);
                assert_eq!(rp.owner.unwrap().login, data.owner);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn current_user_test() {
        let data = Data::new();
        let user = get_current_user().await;
        match user {
            Ok(usr) => assert_eq!(usr.login, data.owner),
            Err(_) => panic!("Nicknames don't match"),
        }
    }

    fn get_issues_util(paged_issues: Page<Issue>, data: Data) {
        let issues_vec = paged_issues.items;
        assert!(!issues_vec.is_empty());
        issues_vec.into_iter().for_each(|issue: Issue| {
            assert!(!issue.title.is_empty());
            assert_eq!(issue.user.login, data.owner);
        })
    }

    #[tokio::test]
    async fn get_issuses_test() {
        let data = Data::new();
        let issues = get_issues(data.owner.as_str(), data.repo.as_str()).await;
        match issues {
            Ok(paged_issues) => get_issues_util(paged_issues, data),
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn get_issues_by_assignee_test() {
        let data = Data::new();
        let issues =
            get_issues_by_assignee(data.owner.as_str(), data.repo.as_str(), "zackfall").await;
        match issues {
            Ok(paged_issues) => get_issues_util(paged_issues, data),
            Err(err) => panic!("{}", err),
        }
    }
}
