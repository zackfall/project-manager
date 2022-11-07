use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Owner::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Owner::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Owner::Username).string().not_null())
                    .col(ColumnDef::new(Owner::Url).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Owner::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Owner {
    Table,
    Id,
    Username,
    Url,
}
