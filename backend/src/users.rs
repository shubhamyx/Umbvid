use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<Uuid, sqlx::Error> {
    let rec: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let row: Option<(Uuid, String, String)> = sqlx::query_as(
        r#"
        SELECT id, email, password_hash FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(id, email, password_hash)| UserRecord {
        id,
        email,
        password_hash,
    }))
}