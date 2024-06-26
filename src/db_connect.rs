use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbConn, DbErr};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "postgresql://postgres@localhost:5432/rust_be".to_owned(),
    };

    let db_connect = Database::connect(database_url)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db_connect, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db_connect)
}
