pub mod datatypes;
pub mod error;

use error::*;
use futures_util::Stream;
use futures_util::StreamExt;
use std::pin::Pin;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

use reqwest_eventsource::{Event, RequestBuilderExt};

pub type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<datatypes::GenerateContentResponse>> + Send>>;

/// Generates streaming content from the API.
///
/// Returns a stream of `GenerateContentResponse` objects that can be consumed asynchronously.
/// Uses Server-Sent Events (SSE) to stream the responses.
pub async fn generate_content_stream(
    api_key: &str,
    req: datatypes::GenerateContentReq,
) -> Result<ResponseStream> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}:streamGenerateContent?alt=sse&key={}",
        BASE_URL, req.model, api_key
    );
    let es = client
        .post(&url)
        .json(&req)
        .eventsource()
        .map_err(|e| GenAiError::Internal(format!("Failed to create event source: {}", e)))?;
    let stream = es
        .map(|event| match event {
            Ok(Event::Message(message)) => {
                let data = message.data;
                if data == "[DONE]" {
                    return Ok(Some(datatypes::GenerateContentResponse {
                        candidates: None,
                        prompt_feedback: None,
                        model_version: None,
                        usage_metadata: None,
                    }));
                }
                serde_json::from_str(&data)
                    .map_err(|e| GenAiError::Internal(format!("JSON parse error: {}\n{}", e, data)))
                    .map(Some)
            }
            Ok(Event::Open) => Ok(None),
            Err(reqwest_eventsource::Error::StreamEnded) => {
                Ok(Some(datatypes::GenerateContentResponse {
                    candidates: None,
                    prompt_feedback: None,
                    model_version: None,
                    usage_metadata: None,
                }))
            }
            Err(e) => Err(GenAiError::Internal(format!("Stream error: {}", e))),
        })
        .filter_map(|r| async move { r.transpose() });

    Ok(Box::pin(stream))
}

/// Generates content from the API in a single request.
///
/// Makes a single POST request to the API and returns the complete response.
pub async fn generate_content(
    api_key: &str,
    req: datatypes::GenerateContentReq,
) -> Result<datatypes::GenerateContentResponse> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}:generateContent?key={}", BASE_URL, req.model, api_key);
    let response = client
        .post(&url)
        .json(&req)
        .send()
        .await
        .map_err(|e| GenAiError::Internal(format!("Request failed: {}", e)))?;
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(GenAiError::Remote(error_text));
    }
    response
        .json()
        .await
        .map_err(|e| GenAiError::Internal(format!("Failed to deserialize response: {}", e)))
}
