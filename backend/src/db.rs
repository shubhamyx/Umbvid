use sqlx::{Pool, Postgres}; 
use sqlx::postgres::PgPoolOptions;

pub type DbPool=Pool<Postgres>;

pub async fn init_pool(database_url:&str)->DbPool{
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to postgres")
}