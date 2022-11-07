pub use sea_orm_migration::prelude::*;

mod m20221106_182043_create_owner_table;
mod m20221106_182552_create_comment_table;
mod m20221106_182605_create_issue_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221106_182043_create_owner_table::Migration),
            Box::new(m20221106_182552_create_comment_table::Migration),
            Box::new(m20221106_182605_create_issue_table::Migration),
        ]
    }
}
