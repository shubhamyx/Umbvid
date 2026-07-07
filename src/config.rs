use std::env;

pub struct Config {
    pub _openai_api_key: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); 
        Config {
            _openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }         
}
