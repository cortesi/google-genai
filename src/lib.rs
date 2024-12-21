pub mod datatypes;
pub mod error;

use error::*;
use futures_util::Stream;
use futures_util::StreamExt;
use std::pin::Pin;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const DEFAULT_MODEL: &str = "gemini-pro";

pub type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<datatypes::GenerateContentResponse>> + Send>>;

pub async fn generate_content_stream(
    api_key: &str,
    params: datatypes::GenerateContentParameters,
) -> Result<ResponseStream> {
    let client = reqwest::Client::new();
    let model = params.model.as_deref().unwrap_or(DEFAULT_MODEL);
    let url = format!(
        "{}/{}:streamGenerateContent?alt=sse&key={}",
        BASE_URL, model, api_key
    );

    let response = client
        .post(&url)
        .json(&params)
        .send()
        .await
        .map_err(|e| GenAiError::Internal(format!("Request failed: {}", e)))?;

    let stream = response
        .bytes_stream()
        .map(|chunk_result| {
            chunk_result
                .map_err(|e| GenAiError::Internal(format!("Stream error: {}", e)))
                .and_then(|chunk| {
                    // Each SSE message starts with "data: " and ends with two newlines
                    let text = String::from_utf8_lossy(&chunk);
                    if text.starts_with("data: ") {
                        let json_str = text.strip_prefix("data: ").unwrap().trim();
                        if json_str == "[DONE]" {
                            return Ok(None);
                        }
                        serde_json::from_str(json_str)
                            .map_err(|e| {
                                GenAiError::Internal(format!(
                                    "JSON parse error: {}\n{}",
                                    e, json_str
                                ))
                            })
                            .map(Some)
                    } else {
                        Ok(None)
                    }
                })
        })
        .filter_map(|r| async move { r.transpose() });

    Ok(Box::pin(stream))
}

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
