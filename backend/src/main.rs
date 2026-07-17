mod auth;
mod config;
mod db;
mod errors;
mod jobs;
mod openai_client;
mod r2;
mod users;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use auth::AuthUser;
use config::Config;
use db::DbPool;
use errors::AppError;
use jobs::JobRecord;
use r2::R2Client;

#[derive(Clone)]
pub struct AppState {
    pool: DbPool,
    openai_key: String,
    jwt_secret: String,
    r2: R2Client,
}

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    image_url: String,
}

#[derive(Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if let Some(_existing) = users::find_by_email(&state.pool, &payload.email).await? {
        return Err(AppError::Conflict("email already registered".to_string()));
    }

    let password_hash = auth::hash_password(&payload.password)?;
    let user_id = users::create_user(&state.pool, &payload.email, &password_hash).await?;
    let token = auth::create_jwt(user_id, &state.jwt_secret)?;

    Ok(Json(AuthResponse { token }))
}

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = users::find_by_email(&state.pool, &payload.email)
        .await?
        .ok_or_else(|| AppError::Auth("invalid email or password".to_string()))?;

    let valid = auth::verify_password(&payload.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Auth("invalid email or password".to_string()));
    }

    let token = auth::create_jwt(user.id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { token }))
}

async fn generate_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let user_id = auth_user.user_id;

    let job_id = jobs::create_job(&state.pool, user_id, &payload.prompt).await?;

    match openai_client::generate_image(&state.openai_key, &payload.prompt).await {
        Ok(bytes) => {
            let key = format!("jobs/{job_id}.png");

            match state.r2.upload_image(&key, bytes).await {
                Ok(url) => {
                    jobs::mark_completed(&state.pool, job_id, &url).await?;
                    Ok(Json(GenerateResponse { image_url: url }))
                }
                Err(e) => {
                    let msg = format!("{e:?}");
                    if let Err(db_err) = jobs::mark_failed(&state.pool, job_id, &msg).await {
                        eprintln!("also failed to record job failure: {db_err}");
                    }
                    Err(e)
                }
            }
        }
        Err(e) => {
            if let Err(db_err) = jobs::mark_failed(&state.pool, job_id, &e).await {
                eprintln!("also failed to record job failure: {db_err}");
            }
            Err(AppError::ImageGeneration(e))
        }
    }
}

async fn get_job_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(job_id): Path<Uuid>,
) -> Result<Json<JobRecord>, AppError> {
    let job = jobs::get_job_by_id(&state.pool, job_id, auth_user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("job".to_string()))?;

    Ok(Json(job))
}

async fn list_jobs_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<JobRecord>>, AppError> {
    let jobs = jobs::list_jobs_for_user(&state.pool, auth_user.user_id).await?;
    Ok(Json(jobs))
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;
    let r2_client = R2Client::new(&cfg);

    let state = AppState {
        pool,
        openai_key: cfg._openai_api_key,
        jwt_secret: cfg.jwt_secret,
        r2: r2_client,
    };

    let app = Router::new()
        .route("/api/signup", post(signup_handler))
        .route("/api/login", post(login_handler))
        .route("/api/generate", post(generate_handler))
        .route("/api/jobs", get(list_jobs_handler))
        .route("/api/jobs/{id}", get(get_job_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}