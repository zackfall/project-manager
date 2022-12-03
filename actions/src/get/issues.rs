use super::super::build_octo;
use octocrab::{
    models::{issues::Issue, Repository},
    params::{self, issues::Filter},
    Page, Result,
};

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

pub async fn get_issues_by_creator(owner: &str, repo: &str, creator: &str) -> Result<Page<Issue>> {
    let creator = build_octo()
        .issues(owner, repo)
        .list()
        .creator(creator.to_owned())
        .state(params::State::All)
        .per_page(15)
        .page(1u8)
        .send()
        .await?;
    Ok(creator)
}

pub async fn get_issues_by_state(
    owner: &str,
    repo: &str,
    state: params::State,
) -> Result<Page<Issue>> {
    let creator = build_octo()
        .issues(owner, repo)
        .list()
        .state(state)
        .state(params::State::All)
        .per_page(15)
        .page(1u8)
        .send()
        .await?;
    Ok(creator)
}

pub async fn get_issues_by_labels(
    owner: &str,
    repo: &str,
    labels: &[String],
) -> Result<Page<Issue>> {
    let creator = build_octo()
        .issues(owner, repo)
        .list()
        .labels(labels)
        .state(params::State::All)
        .per_page(15)
        .page(1u8)
        .send()
        .await?;
    Ok(creator)
}

pub async fn get_repo(owner: &str, repo: &str) -> Result<Repository> {
    let repo = build_octo().repos(owner, repo).get().await?;
    Ok(repo)
}
