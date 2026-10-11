mod image_service;
pub struct GeneratedImage {
pub image_url: String,
}

pub async fn generate_image(
_prompt: &str,
) -> Result<GeneratedImage, String> {
Err(
"Image generation provider is not configured yet. 
Connect a free image-generation provider to enable this feature."
.to_string(),
)
}
