use std::io::Write;

use futures::StreamExt;
// Import required modules from the LLM library for OpenAI integration
use llm::{
    backends::azure_openai::AzureOpenAI,
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, ChatProvider},
    chat_stream::{ChatResponseDelta, StreamChatProvider}, // Chat-related structures
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get OpenAI API key from environment variable or use test key as fallback
    let api_key = std::env::var("AZURE_OPENAI_API_KEY").unwrap_or("your api key here".into());
    let api_version =
        std::env::var("AZURE_OPENAI_API_VERSION").unwrap_or("your api version here".into());
    let endpoint =
        std::env::var("AZURE_OPENAI_API_ENDPOINT").unwrap_or("your api endpoint here".into());

    let deployment_id =
        std::env::var("AZURE_OPENAI_DEPLOYMENT").unwrap_or("gpt-4o-mini".to_owned());

    // Initialize and configure the LLM client
    // let llm = LLMBuilder::new()
    //     .backend(LLMBackend::AzureOpenAI) // Use OpenAI as the LLM provider
    //     .base_url(endpoint)
    //     .api_key(api_key) // Set the API key
    //     .api_version(api_version)
    //     .deployment_id(deployment_id)
    //     .model("gpt-4o-mini") // Use GPT-3.5 Turbo model
    //     .max_tokens(512) // Limit response length
    //     .temperature(0.7) // Control response randomness (0.0-1.0)
    //     .stream(false) // Disable streaming responses
    //     .build()
    //     .expect("Failed to build LLM (Azure OpenAI)");
    let llm = AzureOpenAI::new(
        api_key,
        api_version,
        deployment_id,
        endpoint,
        Some("gpt-4o-mini".into()),
        Some(512),
        Some(0.7),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Prepare conversation history with example messages
    let messages = vec![
        ChatMessage::user()
            .content("Tell me that you love cats")
            .build(),
        ChatMessage::assistant()
            .content("I am an assistant, I cannot love cats but I can love dogs")
            .build(),
        ChatMessage::user()
            .content("Tell me that you love dogs in 2000 chars")
            .build(),
    ];

    // Send chat request and handle the response
    match llm.chat_stream(&messages).await {
        Ok(mut stream) => {
            while let Some(delta) = stream.next().await {
                print!(
                    "{}",
                    delta
                        .text()
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("nothing in here")
                );
                std::io::stdout().flush();
            }
        }
        Err(e) => eprintln!("Chat error: {}", e),
    }

    Ok(())
}
