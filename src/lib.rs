pub mod datatypes;
pub mod error;

use error::*;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const DEFAULT_MODEL: &str = "gemini-pro";

pub async fn generate_content(
    api_key: &str,
    params: datatypes::GenerateContentParameters,
) -> Result<datatypes::GenerateContentResponse> {
    let client = reqwest::Client::new();
    let model = params.model.as_deref().unwrap_or(DEFAULT_MODEL);
    let url = format!("{}/{}:generateContent?key={}", BASE_URL, model, api_key);
    let response = client
        .post(&url)
        .json(&params)
        .send()
        .await
        .map_err(|e| GenAiError::Internal(format!("Request failed: {}", e)))?;
    response
        .json()
        .await
        .map_err(|e| GenAiError::Internal(format!("Failed to deserialize response: {}", e)))
}
