use axum::{extract::State, Json};
use crate::{config::Config, error::AppResult, services};
use crate::models::ai::{AskAIRequest, AskAIResponse};

pub async fn handle_ai_request(
    State(config): State<Config>,
    Json(request): Json<AskAIRequest>,
) -> AppResult<Json<AskAIResponse>> {
    let response = services::ai::process_ai_request(config.gemini_api_key, request).await?;
    Ok(Json(response))
}