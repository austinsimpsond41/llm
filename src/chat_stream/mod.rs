use async_trait::async_trait;
use futures::Stream;

use crate::{
    chat::{ChatMessage, Tool},
    error::LLMError,
};

pub struct ToolCallDelta {
    pub index: usize,
    pub id: Option<String>,
}

pub struct FunctionCallDelta {
    pub arguments: Option<String>,
    pub name: String,
}

pub trait ChatResponseDelta: std::fmt::Debug + std::fmt::Display {
    fn text(&self) -> Option<String>;
    fn tool_call(&self) -> Option<ToolCallDelta>;
}

#[async_trait]
pub trait StreamChatProvider {
    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
    ) -> Result<impl Stream<Item = Result<Box<impl ChatResponseDelta>, LLMError>>, LLMError> {
        self.chat_stream_with_tools(messages, None).await
    }

    async fn chat_stream_with_tools(
        &self,
        messages: &[ChatMessage],
        tools: Option<&[Tool]>,
    ) -> Result<impl Stream<Item = Result<Box<impl ChatResponseDelta>, LLMError>>, LLMError>;
}
