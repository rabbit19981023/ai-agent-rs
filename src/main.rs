use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_config(OpenAIConfig::new().with_api_base("http://localhost:4000"));
    let model = "gpt-3.5-turbo";

    chat(&client, model).await
}

async fn chat(client: &Client<OpenAIConfig>, model: &str) -> Result<(), Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessage::from("You are a helpful assistant.").into(),
            ChatCompletionRequestUserMessage::from("who won the world series in 2020?").into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        match choice.message.content {
            Some(content) => println!(
                "[{}] Role: {}, Response: {}",
                choice.index, choice.message.role, content
            ),
            None => println!("{}: <empty>", choice.index),
        }
    }

    Ok(())
}
