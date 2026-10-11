use reqwest::Client;
use serde_json::{json, Value};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

pub struct GeneratedImage {
    pub image_url: String,
}

pub async fn generate_image(prompt: &str) -> Result<GeneratedImage, String> {
    // Exact location of the ComfyUI API workflow.
    let workflow_path =
        Path::new(r"C:\Users\User\Documents\Trueform\workflow_api.json");

    println!("Looking for workflow at: {:?}", workflow_path);

    // Check that the workflow file actually exists.
    if !workflow_path.exists() {
        return Err(format!(
            "workflow_api.json was not found at: {:?}",
            workflow_path
        ));
    }

    // Read the workflow JSON file.
    let workflow_text = tokio::fs::read_to_string(workflow_path)
        .await
        .map_err(|e| {
            format!(
                "Could not read workflow_api.json at {:?}: {}",
                workflow_path, e
            )
        })?;

    // Convert JSON text into a Rust JSON object.
    let mut workflow: Value = serde_json::from_str(&workflow_text)
        .map_err(|e| format!("Invalid workflow JSON: {}", e))?;

    // Node 6 is the positive CLIP text prompt.
    workflow["6"]["inputs"]["text"] = Value::String(prompt.to_string());

    let client = Client::new();

    println!("Sending workflow to ComfyUI...");

    // Send the workflow to ComfyUI.
    let response = client
        .post("http://127.0.0.1:8188/prompt")
        .json(&json!({
            "prompt": workflow,
            "client_id": "trueform"
        }))
        .send()
        .await
        .map_err(|e| {
            format!(
                "Could not connect to ComfyUI at http://127.0.0.1:8188: {}",
                e
            )
        })?;

    // Check whether ComfyUI accepted the workflow.
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        return Err(format!(
            "ComfyUI rejected the workflow. Status: {} Body: {}",
            status, body
        ));
    }

    // Read ComfyUI's response.
    let queue_response: Value = response
        .json()
        .await
        .map_err(|e| format!("Invalid response from ComfyUI: {}", e))?;

    // ComfyUI gives us an ID for this generation job.
    let prompt_id = queue_response["prompt_id"]
        .as_str()
        .ok_or_else(|| {
            format!(
                "ComfyUI did not return a prompt_id. Response: {}",
                queue_response
            )
        })?;

    println!("ComfyUI job queued: {}", prompt_id);

    // Check ComfyUI every 500 ms for up to 120 seconds.
    for _ in 0..240 {
        sleep(Duration::from_millis(500)).await;

        let history_url =
            format!("http://127.0.0.1:8188/history/{}", prompt_id);

        let history_response = client
            .get(&history_url)
            .send()
            .await
            .map_err(|e| {
                format!("Could not read ComfyUI history: {}", e)
            })?;

        if !history_response.status().is_success() {
            continue;
        }

        let history: Value = history_response
            .json()
            .await
            .map_err(|e| {
                format!("Invalid ComfyUI history response: {}", e)
            })?;

        // Node 9 is the Save Image node in our workflow.
        if let Some(image) = history[prompt_id]["outputs"]["9"]["images"]
            .as_array()
            .and_then(|images| images.first())
        {
            let filename = image["filename"]
                .as_str()
                .ok_or_else(|| {
                    "Generated image has no filename.".to_string()
                })?;

            let subfolder = image["subfolder"]
                .as_str()
                .unwrap_or("");

            let image_type = image["type"]
                .as_str()
                .unwrap_or("output");

            // Build the URL used by ComfyUI to display the generated image.
            let mut image_url =
                reqwest::Url::parse("http://127.0.0.1:8188/view")
                    .map_err(|e| {
                        format!("Could not create image URL: {}", e)
                    })?;

            image_url
                .query_pairs_mut()
                .append_pair("filename", filename)
                .append_pair("subfolder", subfolder)
                .append_pair("type", image_type);

            println!("Image generated successfully!");
            println!("Image URL: {}", image_url);

            return Ok(GeneratedImage {
                image_url: image_url.to_string(),
            });
        }
    }

    Err(
        "Image generation timed out after 120 seconds. Check ComfyUI."
            .to_string(),
    )
}