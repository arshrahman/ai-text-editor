use crate::error::{AppError, AppResult};
use crate::models::ai::{AskAIRequest, AskAIResponse};
use serde_json::json;

pub async fn process_ai_request(
    api_key: String,
    request: AskAIRequest,
) -> AppResult<AskAIResponse> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "system_instruction": {
            "parts": {
                "text": format!("{} the given text and only output the modified text. Do not output anything else", request.command.to_string()),
            }
        },
        "contents": [{
            "parts": [{
                "text": request.content
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "topK": 1,
            "topP": 1,
            "maxOutputTokens": 2048,
        }
    });

    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            api_key
        ))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize request body: {}", e)))?
        )
        .send()
        .await
        .map_err(|e| AppError::GeminiApi(format!("Failed to send request to Gemini API: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::GeminiApi(
            format!("Gemini API returned error status: {}", response.status())
        ));
    }

    let response_data = response.json::<serde_json::Value>()
        .await
        .map_err(|e| AppError::Serialization(
            format!("Failed to parse Gemini API response: {}", e)
        ))?;

    let generated_text = response_data
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .ok_or_else(|| AppError::GeminiApi(
            "Invalid response structure from Gemini API".to_string()
        ))?;

    Ok(AskAIResponse {
        result: generated_text.to_string(),
        command: request.command.to_string(),
    })
}