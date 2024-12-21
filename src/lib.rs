use hyper::Client;

pub mod datatypes;
pub mod error;

use error::*;

const BASE_URL: &str ="https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse&key=${GOOGLE_API_KEY}";

pub async fn generate_content(
    api_key: &str,
    params: datatypes::GenerateContentParameters,
) -> Result<datatypes::GenerateContentResponse> {
}
