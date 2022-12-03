mod utils;
use actions::get::issues;
use octocrab::{models::issues::Issue, params, Page};
use utils::Data;

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
    let data = Data::default();
    let issues = issues::get_issues(data.owner.as_str(), data.repo.as_str()).await;
    match issues {
        Ok(paged_issues) => get_issues_util(paged_issues, data),
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_issues_by_assignee_test() {
    let data = Data::default();
    let issues =
        issues::get_issues_by_assignee(data.owner.as_str(), data.repo.as_str(), "zackfall").await;
    match issues {
        Ok(paged_issues) => get_issues_util(paged_issues, data),
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_issues_by_creator_test() {
    let data = Data::default();
    let issues =
        issues::get_issues_by_creator(data.owner.as_str(), data.repo.as_str(), "zackfall").await;
    match issues {
        Ok(paged_issues) => get_issues_util(paged_issues, data),
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_issues_by_state_test() {
    let data = Data::default();
    let issues = issues::get_issues_by_state(
        data.owner.as_str(),
        data.repo.as_str(),
        params::State::Closed,
    )
    .await;
    match issues {
        Ok(paged_issues) => get_issues_util(paged_issues, data),
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn get_issues_by_labels_test() {
    let data = Data::default();
    let issues = issues::get_issues_by_labels(
        data.owner.as_str(),
        data.repo.as_str(),
        &["foundations".to_owned()],
    )
    .await;
    match issues {
        Ok(paged_issues) => get_issues_util(paged_issues, data),
        Err(err) => panic!("{}", err),
    }
}
