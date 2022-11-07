#![warn(missing_docs)]

//! A CLI task manager writen with tui-rs, for personal use.

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbErr, Statement};
use migration::{Migrator, MigratorTrait};

const DATABASE_URI: &str = "postgres://postgres@localhost:5432";
const DB_NAME: &str = "task_manager";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URI).await?;

    let db = {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
        ))
        .await?;
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE \"{}\";", DB_NAME),
        ))
        .await?;

        let url = format!("{}/{}", DATABASE_URI, DB_NAME);
        Database::connect(&url).await?
    };
    Migrator::up(&db, None).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_to_database() {
        let res = run().await;

        assert!(res.is_ok());
    }
}
