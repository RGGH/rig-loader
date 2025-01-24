use dotenv::dotenv;

use rig::{
    agent::AgentBuilder,
    completion::Prompt,
    loaders::FileLoader,
    providers::openai::{self, GPT_4O},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let openai_client = openai::Client::from_env();

    let model = openai_client.completion_model(GPT_4O);

    // Load in all the rust examples
    // this pipeline produces an iterable stream of (path, content) pairs
    let examples = FileLoader::with_glob("*.rs")?
        .read_with_path()
        .ignore_errors()
        .into_iter();

    // Create an agent with multiple context documents
    let agent = examples
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("Rust Example {:?}:\n{}", path, content).as_str())
        })
        .build();

        // Prompt the agent for managing containers in a cloud environment
    let response = agent
        .prompt("Which Rust code example would be best suited to only start a container in a cloud environment? Provide reasoning - show the part of the code or function that is most relevant")
        .await?;

    println!("{}", response);

    Ok(())
}

#[tokio::test]
async fn test_container_management_prompt() {
    dotenv().ok();
    let openai_client = openai::Client::from_env();
    let model = openai_client.completion_model(GPT_4O);

    // Mocked Rust file for container management example
    let rust_file_content = r#"
    // Container lifecycle management
    use incus_client::IncusClient;

    pub fn start_container(container_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = IncusClient::new()?;
        client.start(container_id)?;
        Ok(())
    }

    pub fn stop_container(container_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = IncusClient::new()?;
        client.stop(container_id)?;
        Ok(())
    }
    "#;

    // Simulate loading this as an example file
    let example = vec![("container_management.rs".to_string(), rust_file_content.to_string())];

    // Build the agent
    let agent = example.into_iter().fold(AgentBuilder::new(model), |builder, (path, content)| {
        builder.context(format!("Rust Example {:?}:\n{}", path, content).as_str())
    }).build();

    // Prompt the agent for which code example is suitable for container operations
    let response = agent
        .prompt("Which Rust code example would be best suited to stop a container in a cloud environment?")
        .await
        .expect("Agent failed to generate response");

    // Check that the response mentions this example
    assert!(response.contains("container_management.rs"));
    println!("{}", response);
}
