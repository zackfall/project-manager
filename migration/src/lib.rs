pub use sea_orm_migration::prelude::*;

pub mod m20221106_182043_create_owner_table;
pub mod m20221106_182552_create_comment_table;
pub mod m20221106_182605_create_issue_table;
pub mod m20221111_162451_create_repository_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221106_182043_create_owner_table::Migration),
            Box::new(m20221106_182552_create_comment_table::Migration),
            Box::new(m20221106_182605_create_issue_table::Migration),
            Box::new(m20221111_162451_create_repository_table::Migration),
        ]
    }
}
