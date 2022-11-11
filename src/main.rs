#![warn(missing_docs)]

//! A CLI task manager writen with tui-rs, for personal use.

use futures::executor::block_on;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};

/// This struct manage the connection with the database
/// it also run queries and actions to interact with the database
/// It will be moved to another file in the future.
struct DatabaseConn<'a> {
    db_uri: &'a str,
    /// Is an Optional Value because, there is the case (the actual case) where you
    /// want to use SQLite, so you don't need this.
    db_name: Option<&'a str>,
}

impl<'a> DatabaseConn<'a> {
    /// Creates a new instance of DatabaseConn
    pub fn new(db_uri: &'a str, db_name: Option<&'a str>) -> Self {
        Self { db_uri, db_name }
    }

    /// Connects to the database.
    ///
    /// We use a match statement, in the case we want to use another dabatase
    /// backend in the future.
    pub async fn connect(self) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(self.db_uri).await?;
        let db = match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", self.db_name.unwrap()),
                ))
                .await?;

                let url = format!("{}/{}", self.db_uri, self.db_name.unwrap());
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", self.db_name.unwrap()),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", self.db_name.unwrap()),
                ))
                .await?;

                let url = format!("{}/{}", self.db_uri, self.db_name.unwrap());
                Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        };
        Migrator::up(&db, None).await?;

        Ok(db)
    }
}

fn main() {
    let db_uri = std::env::var("DATABASE_URI").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    let db = DatabaseConn::new(db_uri.as_str(), None);
    if let Err(err) = block_on(db.connect()) {
        panic!("{}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_to_database() {
        let db_uri = "sqlite::memory:";
        let db = DatabaseConn::new(db_uri, None);
        let res: Result<DatabaseConnection, DbErr> = db.connect().await;
        assert!(res.is_ok());
    }
}
