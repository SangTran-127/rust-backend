use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn establish_connection(
    db_url: &String,
    max_connection: u32,
) -> Result<DatabaseConnection, DbErr> {
    let mut connection_options = ConnectOptions::new(db_url);
    connection_options.max_connections(max_connection);

    let db_connect = Database::connect(connection_options)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db_connect, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db_connect)
}
