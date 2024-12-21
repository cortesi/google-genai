use google_genai::datatypes::{Content, GenerateContentParameters, Part};

#[tokio::main]
async fn main() -> google_genai::error::Result<()> {
    let api_key = std::env::var("GOOGLEAI_API_KEY")
        .expect("GOOGLEAI_API_KEY environment variable must be set");

    let params = GenerateContentParameters::default()
        .contents(vec![Content {
            parts: Some(vec![
                Part::default().text("Tell me a joke about programming.")
            ]),
            role: Some("user".to_string()),
        }])
        .model("gemini-exp-1206");

    let response = google_genai::generate_content(&api_key, params).await?;

    println!("Response: {:#?}", response);

    Ok(())
}
