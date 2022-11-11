use std::fmt;

use sea_orm_migration::prelude::*;

use super::m20221106_182043_create_owner_table::Owner;
use super::m20221106_182552_create_comment_table::Comment;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Issue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Issue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Issue::Title).string().not_null())
                    .col(ColumnDef::new(Issue::Body).string())
                    .col(ColumnDef::new(Issue::Url).string().not_null())
                    .col(ColumnDef::new(Issue::State).string())
                    .col(ColumnDef::new(Issue::CreatedAt).string())
                    .col(ColumnDef::new(Issue::UpdatedAt).string())
                    .col(ColumnDef::new(Issue::ClosedAt).string())
                    .col(ColumnDef::new(Issue::ClosedById).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issue-closed_by_id")
                            .from(Issue::Table, Issue::ClosedById)
                            .to(Owner::Table, Owner::Id),
                    )
                    .col(ColumnDef::new(Issue::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issue-owner_id")
                            .from(Issue::Table, Issue::OwnerId)
                            .to(Owner::Table, Owner::Id),
                    )
                    .col(ColumnDef::new(Issue::CommentId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issue-comment_id")
                            .from(Issue::Table, Issue::CommentId)
                            .to(Issue::Table, Comment::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Issue::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Issue {
    Table,
    Id,
    Title,
    Body,
    Url,
    State,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "closed_at"]
    ClosedAt,
    #[iden = "closed_by_id"]
    ClosedById,
    #[iden = "owner_id"]
    OwnerId,
    #[iden = "comment_id"]
    CommentId,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Issue::Table => write!(f, "issue"),
            Issue::Id => write!(f, "id"),
            Issue::Title => write!(f, "title"),
            Issue::Body => write!(f, "body"),
            Issue::Url => write!(f, "url"),
            Issue::State => write!(f, "state"),
            Issue::CreatedAt => write!(f, "created_at"),
            Issue::UpdatedAt => write!(f, "updated_at"),
            Issue::ClosedAt => write!(f, "closed_at"),
            Issue::ClosedById => write!(f, "closed_by_id"),
            Issue::OwnerId => write!(f, "owner_id"),
            Issue::CommentId => write!(f, "comment_id"),
        }
    }
}
