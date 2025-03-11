use axum::{
  extract::State,
  http::StatusCode,
  routing::{get, post},
  Json, Router,
};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;


// Added missing derive macros for request struct
#[derive(Debug, Deserialize, Serialize)]
struct AskAIRequest {
    content: String,
    command: String,
}

#[tokio::main]
async fn main() {
  // Load environment variables first
  dotenvy::dotenv().expect("Unable to access .env file");
  
  // Read environment variables
  let server_address: String = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  let gemini_api_key: String = std::env::var("GEMINI_API_KEY").expect("API key not found");

  // Create TCP listener
  let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");

  println!("listening on {}", listener.local_addr().unwrap());

  // Create app state to pass the API key
  let app_state = AppState {
    gemini_api_key,
  };

  let app = Router::new()
    .route("/", get(|| async { "Hello world" }))
    .route("/ai/action", post(ask_ai))
    .with_state(app_state)
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());

  //serve the application
  axum::serve(listener, app)
    .await
    .expect("Error serving application");
}

// Added app state struct to properly pass API key
#[derive(Clone)]
struct AppState {
    gemini_api_key: String,
}

async fn ask_ai(
  State(state): State<AppState>,
  Json(task): Json<AskAIRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "system_instruction": {
            "parts": {
                "text": format!("{} the given text and only output the modified text. Do not output anything else", task.command),
            }
        },
        "contents": [{
            "parts": [{
                "text": task.content
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "topK": 1,
            "topP": 1,
            "maxOutputTokens": 2048,
        }
    });

    // Use header() and body() instead of json()
    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            state.gemini_api_key
        ))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             format!("Failed to serialize request body: {}", e))
        })?)
        .send()
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR,
             format!("Failed to send request to Gemini API: {}", e))
        })?;

    if !response.status().is_success() {
        return Err((StatusCode::BAD_GATEWAY,
            format!("Gemini API returned error status: {}", response.status())));
    }

    let response_data = response.json::<serde_json::Value>().await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR,
             format!("Failed to parse Gemini API response: {}", e))
        })?;

    let generated_text = response_data
        .get("candidates")
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .ok_or_else(|| {
            (StatusCode::INTERNAL_SERVER_ERROR,
             "Invalid response structure from Gemini API".to_string())
        })?;

    // Return the full response for more flexibility
    Ok((StatusCode::OK, Json(json!({
        "result": generated_text,
        "command": task.command
    }))))
}