use futures::StreamExt;
// Import required modules from the LLM library
use llm::chat_stream::ChatResponseDelta;
use llm::{
    backends::ollama::Ollama,
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
    chat_stream::StreamChatProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Ollama server URL from environment variable or use default localhost
    let base_url = std::env::var("OLLAMA_URL").unwrap_or("http://127.0.0.1:11434".into());

    // Initialize and configure the LLM client
    let llm = Ollama::new(
        base_url,
        Some("ollama".into()),
        Some("llama3.1:latest".into()),
        Some(1000),
        Some(0.7),
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
            .content("Hello, how do I run a local LLM in Rust?")
            .build(),
        ChatMessage::assistant()
            .content("One way is to use Ollama with a local model!")
            .build(),
        ChatMessage::user()
            .content("Tell me more about that")
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
                )
            }
        }
        Err(e) => eprintln!("Chat error: {}", e),
    }

    Ok(())
}
