use super::m20221106_182043_create_owner_table::Owner;
use super::m20221106_182605_create_issue_table::Issue;
use sea_orm_migration::prelude::*;
use std::fmt;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Repository::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Repository::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Repository::Name).string().not_null())
                    .col(ColumnDef::new(Repository::FullName).string().not_null())
                    .col(ColumnDef::new(Repository::Description).string().not_null())
                    .col(ColumnDef::new(Repository::Url).string().not_null())
                    .col(ColumnDef::new(Repository::Fork).boolean())
                    .col(ColumnDef::new(Repository::Private).boolean())
                    .col(ColumnDef::new(Repository::CreatedAt).date().not_null())
                    .col(ColumnDef::new(Repository::UpdatedAt).date())
                    .col(ColumnDef::new(Repository::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-repository-owner_id")
                            .from(Repository::Table, Repository::OwnerId)
                            .to(Owner::Table, Owner::Id),
                    )
                    .col(ColumnDef::new(Repository::IssueId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-repository-issue_id")
                            .from(Repository::Table, Repository::IssueId)
                            .to(Issue::Table, Issue::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Repository::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Repository {
    Table,
    Id,
    Name,
    #[iden = "full_name"]
    FullName,
    Description,
    Url,
    Fork,
    Private,
    #[iden = "default_branch"]
    DefaultBranch,
    #[iden = "open_issues"]
    OpenIssues,
    Topics,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "owner_id"]
    OwnerId,
    #[iden = "issue_id"]
    IssueId,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Repository::Table => write!(f, "repository"),
            Repository::Id => write!(f, "id"),
            Repository::Name => write!(f, "name"),
            Repository::FullName => write!(f, "full_name"),
            Repository::Description => write!(f, "description"),
            Repository::Url => write!(f, "url"),
            Repository::Fork => write!(f, "fork"),
            Repository::Private => write!(f, "private"),
            Repository::DefaultBranch => write!(f, "default_branch"),
            Repository::OpenIssues => write!(f, "open_issues"),
            Repository::Topics => write!(f, "topics"),
            Repository::CreatedAt => write!(f, "created_at"),
            Repository::UpdatedAt => write!(f, "updated_at"),
            Repository::OwnerId => write!(f, "owner_id"),
            Repository::IssueId => write!(f, "issue_id"),
        }
    }
}
