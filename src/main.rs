use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use qdrant_client::{qdrant::CreateCollectionBuilder, Qdrant};

#[tokio::main]
async fn main() -> Result<()> {
    let llm = Client::with_config(OpenAIConfig::new().with_api_base("http://localhost:4000"));
    let model = "gpt-3.5-turbo";

    let qdrant = Qdrant::from_url("http://localhost:6334").build()?;

    chat(&llm, model).await?;

    embedding(&qdrant).await?;

    Ok(())
}

async fn embedding(qdrant: &Qdrant) -> Result<()> {
    let collection_name = "test";

    {
        qdrant.delete_collection(collection_name).await?;
        qdrant
            .create_collection(CreateCollectionBuilder::new(collection_name))
            .await?;
    }

    dbg!(qdrant.collection_info("test").await?);

    Ok(())
}

async fn chat(llm: &Client<OpenAIConfig>, model: &str) -> Result<()> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessage::from("You are a helpful assistant.").into(),
            ChatCompletionRequestUserMessage::from("who won the world series in 2020?").into(),
        ])
        .build()?;

    let response = llm.chat().create(request).await?;

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
