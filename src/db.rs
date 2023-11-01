use sqlx::{Connection, SqliteConnection};

use dotenvy::dotenv;
use std::env;

pub async fn establish_sqlite_conn() -> SqliteConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return SqliteConnection::connect(&database_url).await.expect("failed to connect to database");
}