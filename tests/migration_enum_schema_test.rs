use migration::m20221106_182043_create_owner_table::Owner;
use migration::m20221106_182552_create_comment_table::Comment;
use migration::m20221106_182605_create_issue_table::Issue;
use migration::m20221111_162451_create_repository_table::Repository;

#[test]
fn owner_enum_test() {
    assert_eq!(Owner::Table.to_string(), "owner");
    assert_eq!(Owner::Id.to_string(), "id");
    assert_eq!(Owner::Username.to_string(), "username");
    assert_eq!(Owner::Url.to_string(), "url");
}

#[test]
fn comment_enum_test() {
    assert_eq!(Comment::Table.to_string(), "comment");
    assert_eq!(Comment::Id.to_string(), "id");
    assert_eq!(Comment::Url.to_string(), "url");
    assert_eq!(Comment::Body.to_string(), "body");
    assert_eq!(Comment::IssueUrl.to_string(), "issue_url");
    assert_eq!(Comment::CreatedAt.to_string(), "created_at");
    assert_eq!(Comment::UpdatedAt.to_string(), "updated_at");
    assert_eq!(Comment::OwnerId.to_string(), "owner_id");
}

#[test]
fn issue_enum_test() {
    assert_eq!(Issue::Table.to_string(), "issue");
    assert_eq!(Issue::Id.to_string(), "id");
    assert_eq!(Issue::Title.to_string(), "title");
    assert_eq!(Issue::Body.to_string(), "body");
    assert_eq!(Issue::Url.to_string(), "url");
    assert_eq!(Issue::State.to_string(), "state");
    assert_eq!(Issue::CreatedAt.to_string(), "created_at");
    assert_eq!(Issue::UpdatedAt.to_string(), "updated_at");
    assert_eq!(Issue::ClosedAt.to_string(), "closed_at");
    assert_eq!(Issue::ClosedById.to_string(), "closed_by_id");
    assert_eq!(Issue::OwnerId.to_string(), "owner_id");
    assert_eq!(Issue::CommentId.to_string(), "comment_id");
}

#[test]
fn repository_enum_test() {
    assert_eq!(Repository::Table.to_string(), "repository");
    assert_eq!(Repository::Id.to_string(), "id");
    assert_eq!(Repository::Name.to_string(), "name");
    assert_eq!(Repository::FullName.to_string(), "full_name");
    assert_eq!(Repository::Description.to_string(), "description");
    assert_eq!(Repository::Url.to_string(), "url");
    assert_eq!(Repository::Fork.to_string(), "fork");
    assert_eq!(Repository::Private.to_string(), "private");
    assert_eq!(Repository::DefaultBranch.to_string(), "default_branch");
    assert_eq!(Repository::OpenIssues.to_string(), "open_issues");
    assert_eq!(Repository::Topics.to_string(), "topics");
    assert_eq!(Repository::CreatedAt.to_string(), "created_at");
    assert_eq!(Repository::UpdatedAt.to_string(), "updated_at");
    assert_eq!(Repository::OwnerId.to_string(), "owner_id");
    assert_eq!(Repository::IssueId.to_string(), "issue_id");
}
