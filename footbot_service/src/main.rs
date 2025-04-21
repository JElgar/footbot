use std::collections::HashMap;

use lambda_runtime::{Error, LambdaEvent, service_fn, tracing::log::info};
use reqwest::Url;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

const FOOTBALL_CHAT_ID: &'static str = "120363385864136336@g.us";
// Testing chat
// const FOOTBALL_CHAT_ID: &'static str = "120363419447034622@g.us";
const WAPI_URL: &'static str = "https://gate.whapi.cloud/";
const WAPI_TOKEN: &'static str = std::env!("WAPI_TOKEN");

async fn send_message(message: &'static str, client: &reqwest::Client) -> Result<(), Error> {
    let request_url = Url::parse(WAPI_URL)
        .unwrap()
        .join("/messages/text")
        .unwrap();

    let mut body = HashMap::new();
    body.insert("to", FOOTBALL_CHAT_ID);
    body.insert("body", message);

    client
        .post(request_url.to_string())
        .header("accept", "application/json")
        .header("Authorization", format!("Bearer {}", WAPI_TOKEN))
        .json(&body)
        .send()
        .await?;

    Ok(())
}

async fn handler(_event: LambdaEvent<serde_json::Value>) -> Result<(), Error> {
    info!("Running lambda");

    let client = reqwest::Client::new();

    send_message("Thanks for playing everyone! Please send £4.50 and react to this message once sent", &client).await?;
    send_message("Thumbs up if you are playing next week", &client).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    lambda_runtime::run(service_fn(handler)).await
}
