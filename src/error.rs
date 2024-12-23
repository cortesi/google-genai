use std::collections::HashMap;
use thiserror;

pub type Result<T> = std::result::Result<T, GenAiError>;

#[derive(thiserror::Error, Debug)]
pub enum GenAiError {
    #[error("Remote error {status}: {message}")]
    Remote {
        status: u16,
        message: String,
        headers: HashMap<String, String>,
    },

    #[error("Internal error: {0}")]
    Internal(String),
}

impl GenAiError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .collect();
        let message = response.text().await.unwrap_or_default();
        GenAiError::Remote {
            status,
            message,
            headers,
        }
    }
}
