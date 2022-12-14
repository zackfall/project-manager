use actions::get::repos;
use octocrab::{models::Repository, Page, Result};

#[tokio::test]
async fn get_repo_test() {
    let repo = repos::get_repo("zackfall", "project-manager").await;
    match repo {
        Ok(rp) => {
            assert_eq!(rp.name, "project-manager");
            assert_eq!(rp.owner.unwrap().login, "zackfall");
        }
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_repos_test1() {
    let repos: Result<Page<Repository>> = repos::get_repos(Some("zackfall")).await;
    match repos {
        Ok(rps) => {
            assert!(!rps.items.is_empty());
            rps.items.iter().for_each(|repo| {
                assert!(repo
                    .html_url
                    .as_ref()
                    .unwrap()
                    .as_str()
                    .starts_with("https://github.com/zackfall/"))
            });
        }
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_repos_test2() {
    let repos: Result<Page<Repository>> = repos::get_repos(Some("Hint-Box")).await;
    match repos {
        Ok(rps) => {
            assert!(!rps.items.is_empty());
            rps.items.iter().for_each(|repo| {
                assert!(repo
                    .html_url
                    .as_ref()
                    .unwrap()
                    .as_str()
                    .starts_with("https://github.com/Hint-Box/"))
            });
        }
        Err(err) => panic!("{}", err),
    }
}
