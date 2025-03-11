use crate::error::{AppError, AppResult};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_address: String,
    pub gemini_api_key: String,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenvy::dotenv().map_err(|e| AppError::Environment(format!("Failed to load .env file: {}", e)))?;

        let server_address = "0.0.0.0:7878".to_string();

        let gemini_api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| AppError::Environment("GEMINI_API_KEY not found in environment".to_string()))?;

        Ok(Self {
            server_address,
            gemini_api_key,
        })
    }
}