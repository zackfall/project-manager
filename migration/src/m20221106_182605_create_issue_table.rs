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
                            .to(Comment::Table, Comment::Id),
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
    CreatedAt,
    UpdatedAt,
    ClosedAt,
    ClosedById,
    OwnerId,
    CommentId,
}
