use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Config;
use leptos::logging;

async fn create_local_db() -> Config {
    logging::log!("LOCAL Docker DynamoDB connection retrieved");
    env::set_var("AWS_ACCESS_KEY_ID", "DEMO");
    env::set_var("AWS_SECRET_ACCESS_KEY", "DEMO");
    env::set_var("AWS_SESSION_TOKEN", "DEMO");
    env::set_var("AWS_DEFAULT_REGION", "eu-west-1");

    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    aws_sdk_dynamodb::config::Builder::from(&sdk_config)
        .endpoint_url("http://localhost:8000")
        .build()
}

async fn create_remote_db() -> Config {
    logging::log!("AWS Docker DynamoDB connection...");

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    aws_sdk_dynamodb::config::Builder::from(&config).build()
}

pub async fn create_db() -> Config {
    let remote = env::var("local").unwrap_or(String::from("local"));
    match remote.as_str() {
        "true" => create_local_db().await,
        _ => create_remote_db().await,
    }
}