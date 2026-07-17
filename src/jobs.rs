use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_job(pool: &PgPool, user_id: Uuid, prompt: &str) -> Result<Uuid, sqlx::Error> {
    let rec: (Uuid,)= sqlx::query_as(
        r#"
        INSERT INTO jobs (user_id, job_type, model, prompt, status)
        VALUES ($1, 'image', 'gpt-image-2', $2, 'pending')
        RETURNING id
        "#
    )
    .bind(user_id)
    .bind(prompt)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

pub async fn mark_completed(pool: &PgPool, job_id: Uuid, result_url: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE jobs SET status = 'completed', result_url = $1, updated_at = now()
        WHERE id = $2
        "#
    )
    .bind(result_url)
    .bind(job_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_failed(pool: &PgPool, job_id: Uuid, error: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE jobs SET status = 'failed', error_message = $1, updated_at = now()
        WHERE id = $2
        "#
    )
    .bind(error)
    .bind(job_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Serialize)]
pub struct JobRecord{
    pub id:Uuid,
    pub job_type:String,
    pub model:String,
    pub prompt:String,
    pub status:String,
    pub result_url:Option<String>,
    pub error_message:Option<String>,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>,
}

type JobRow=(
    Uuid,
    String,
    String,
    String,
    String,
    Option<String>,
    Option<String>,
    DateTime<Utc>,
    DateTime<Utc>,
);

fn row_to_job(row:JobRow)->JobRecord{
    JobRecord {
        id: row.0, 
        job_type: row.1,
        model: row.2,
        prompt: row.3,
        status: row.4,
        result_url: row.5,
        error_message: row.6,
        created_at: row.7,
        updated_at: row.8
    }
}

pub async fn get_job_by_id(
    pool: &PgPool,
    job_id: Uuid,
    user_id: Uuid,
) -> Result<Option<JobRecord>, sqlx::Error> {
    let row: Option<JobRow> = sqlx::query_as(
        r#"
        SELECT id, job_type, model, prompt, status, result_url, error_message, created_at, updated_at
        FROM jobs
        WHERE id = $1 AND user_id = $2
        "#
    )
    .bind(job_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(row_to_job))
}

pub async fn list_jobs_for_user(pool:&PgPool, user_id:Uuid)->Result<Vec<JobRecord>, sqlx::Error>{
    let rows:Vec<JobRow>=sqlx::query_as(
        r#"
        SELECT id, job_type, model, prompt, status, result_url, error_message, created_at, updated_at
        FROM jobs
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(row_to_job).collect())
}
