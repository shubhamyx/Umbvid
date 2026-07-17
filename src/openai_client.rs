use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ImageRequest {
    model: String,
    prompt: String,
    n: u8,
    size: String,
}

#[derive(Deserialize, Debug)]
struct ImageResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize, Debug)]
struct ImageData {
    b64_json: Option<String>,
}

pub async fn generate_image(api_key: &str, prompt: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::Client::new();

    let body = ImageRequest {
        model: "gpt-image-2".to_string(),
        prompt: prompt.to_string(),
        n: 1,
        size: "1024x1024".to_string(),
    };

    let res = client
        .post("https://api.openai.com/v1/images/generations")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let err_text = res.text().await.map_err(|e| e.to_string())?;
        return Err(err_text);
    }

    let parsed: ImageResponse = res.json().await.map_err(|e| e.to_string())?;

    let first = parsed.data.get(0).ok_or("No image data returned")?;

    let b64 = first
        .b64_json
        .as_ref()
        .ok_or("No b64_json in OpenAI response")?;

    STANDARD
        .decode(b64)
        .map_err(|e| format!("failed to decode base64 image: {e}"))
}