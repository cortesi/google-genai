use futures_util::StreamExt;
use google_genai::datatypes::{Content, GenerateContentParameters, Part};
use std::env;

#[tokio::main]
async fn main() -> google_genai::error::Result<()> {
    let api_key = env::var("GOOGLEAI_API_KEY").expect("GOOGLE_API_KEY must be set");

    let params =
        GenerateContentParameters::default().contents(vec![Content::default().parts(vec![
            Part::default().text("Write a story about a magic backpack."),
        ])]);

    let mut stream = google_genai::generate_content_stream(&api_key, params).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(content) => {
                if let Some(candidates) = content.candidates {
                    for candidate in candidates {
                        if let Some(content) = candidate.content {
                            if let Some(parts) = content.parts {
                                for part in parts {
                                    if let Some(text) = part.text {
                                        print!("\n\n{}\n\n", text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}