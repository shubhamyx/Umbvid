mod config;
mod db;
mod jobs;
mod openai_client;
use uuid::Uuid;
use axum::{
    routing::post,
    Router,
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use config::Config;
use db::DbPool;

#[derive(Clone)]
struct AppState {
    pool: DbPool,
    openai_key: String,
}

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    image_url: String,
}

async fn generate_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateRequest>,
) -> Json<GenerateResponse> {
    let user_id = Uuid::parse_str("4bb9f07a-401d-4806-a261-c77321493930").unwrap();

    let job_id = jobs::create_job(&state.pool, user_id, &payload.prompt)
        .await
        .expect("failed to create job");

    match openai_client::generate_image(&state.openai_key, &payload.prompt).await {
        Ok(url) => {
            jobs::mark_completed(&state.pool, job_id, &url)
                .await
                .expect("failed to mark completed");
            Json(GenerateResponse { image_url: url })
        }
        Err(e) => {
            jobs::mark_failed(&state.pool, job_id, &e)
                .await
                .expect("failed to mark failed");
            panic!("generation failed: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;

    let state = AppState {
        pool,
        openai_key: cfg._openai_api_key,
    };

    let app = Router::new()
        .route("/api/generate", post(generate_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}