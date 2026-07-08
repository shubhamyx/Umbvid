use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_job(pool: &PgPool, user_id: Uuid, prompt: &str) -> Result<Uuid, sqlx::Error> {
    let rec: (Uuid,) = sqlx::query_as(
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