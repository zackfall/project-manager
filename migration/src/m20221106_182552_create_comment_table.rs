use sea_orm_migration::prelude::*;

use super::m20221106_182043_create_owner_table::Owner;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::Url).string().not_null())
                    .col(ColumnDef::new(Comment::Body).string().not_null())
                    .col(ColumnDef::new(Comment::IssueUrl).string().not_null())
                    .col(ColumnDef::new(Comment::CreatedAt).string())
                    .col(ColumnDef::new(Comment::UpdatedAt).string())
                    .col(ColumnDef::new(Comment::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-owner_id")
                            .from(Comment::Table, Comment::OwnerId)
                            .to(Owner::Table, Owner::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Comment {
    Table,
    Id,
    Url,
    Body,
    IssueUrl,
    CreatedAt,
    UpdatedAt,
    OwnerId,
}
