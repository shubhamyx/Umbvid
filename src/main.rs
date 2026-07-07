mod config;
mod db;
mod openai_client;

use config::Config;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;
    println!("DB connected!");

    // quick test query
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .expect("query failed");
    println!("User count: {}", row.0);

    //test openai image generation
    let image_url = openai_client::generate_image(&cfg._openai_api_key, "a red bicycle on a white background")
        .await
        .expect("image generation failed");
    println!("Image URL: {}", image_url);
}