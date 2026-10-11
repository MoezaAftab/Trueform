mod image_service;

use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[derive(Serialize)]
struct RequirementResult {
    requirement: String,
    status: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    message: String,
    prompt: String,
    image_url: Option<String>,
    requirements: Vec<RequirementResult>,
    note: String,
}

async fn home() -> &'static str {
    "Welcome to Trueform API!"
}

async fn health() -> &'static str {
    "Trueform backend is healthy."
}

async fn generate(
    Json(request): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, String)> {
    let prompt = request.prompt.trim();

    if prompt.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Prompt cannot be empty.".to_string(),
        ));
    }

    if prompt.len() > 2000 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Prompt must be 2000 characters or fewer.".to_string(),
        ));
    }

    let requirements = prompt
        .split([',', '.', ';', '\n'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| RequirementResult {
            requirement: part.to_string(),
            status: "pending".to_string(),
        })
        .collect();

    // REAL AI GENERATION
    let generated_image = image_service::generate_image(prompt)
        .await
        .map_err(|error| {
            eprintln!("Image generation error: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Image generation failed: {}", error),
            )
        })?;

    Ok(Json(GenerateResponse {
        message: "Image generated successfully.".to_string(),
        prompt: prompt.to_string(),
        image_url: Some(generated_image.image_url),
        requirements,
        note: "Generated locally using ComfyUI and Stable Diffusion 1.5.".to_string(),
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/generate", post(generate))
        .route("/health", get(health))
        .fallback_service(ServeDir::new("../frontend"))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to start server");

    println!("Trueform backend running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}