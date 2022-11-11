use migration::m20221106_182043_create_owner_table::Owner;
use migration::m20221106_182552_create_comment_table::Comment;
use migration::m20221106_182605_create_issue_table::Issue;

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
