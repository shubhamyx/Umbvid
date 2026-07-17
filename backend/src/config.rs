use std::env;

pub struct Config {
    pub _openai_api_key: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub r2_account_id: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket_name: String,
    pub r2_public_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); 
        Config {
            _openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            r2_account_id: env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set"),
            r2_access_key_id: env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set"),
            r2_secret_access_key: env::var("R2_SECRET_ACCESS_KEY").expect(" R2_SECRET_ACCESS_KEY must be set"),
            r2_bucket_name: env::var("R2_BUCKET_NAME").expect("R2_BUCKET_NAME must be set"),
            r2_public_url: env::var("R2_PUBLIC_URL").expect("R2_PUBLIC_URL must be set"),  
        }
    }         
}
