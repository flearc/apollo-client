use apollo_client::{ApolloClientResult, Client, ClientConfig, Configuration, IpValue, Response};
use std::collections::HashMap;

#[cfg(feature = "xml")]
use serde_derive::Deserialize;

mod common;

#[async_std::test]
async fn test_client_request() -> ApolloClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.yml"],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config.clone())?.request().await?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");
    assert_eq!(&result[1].app_id, "SampleApp");
    assert_eq!(&result[1].cluster, "default");
    assert_eq!(&result[1].namespace_name, "application.yml");
    assert!(&result[1].configurations.contains_key("content"));

    let result: Response = Client::new(client_config.clone())?.request().await?;
    assert_eq!(&result.app_id, "SampleApp");
    assert_eq!(&result.cluster, "default");
    assert_eq!(&result.namespace_name, "application");
    assert_eq!(&result.configurations["timeout"], "100");

    let result: HashMap<String, Response> = Client::new(client_config.clone())?.request().await?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result["application"].app_id, "SampleApp");
    assert_eq!(&result["application"].cluster, "default");
    assert_eq!(&result["application"].namespace_name, "application");
    assert_eq!(&result["application"].configurations["timeout"], "100");
    assert_eq!(&result["application.yml"].app_id, "SampleApp");
    assert_eq!(&result["application.yml"].cluster, "default");
    assert_eq!(&result["application.yml"].namespace_name, "application.yml");
    assert!(&result["application.yml"]
        .configurations
        .contains_key("content"));

    Ok(())
}

#[cfg(not(feature = "yaml"))]
#[async_std::test]
#[should_panic(expected = "You have to enable feature `yaml` for parsing this configuration kind.")]
async fn test_client_request_2() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.yml"],
        ..Default::default()
    };

    let _: Configuration<()> = Client::new(client_config).unwrap().request().await.unwrap();
}

#[cfg(feature = "yaml")]
#[async_std::test]
async fn test_client_request_2() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.yml"],
        ..Default::default()
    };

    let configuration: Configuration<serde_yaml::Value> =
        Client::new(client_config).unwrap().request().await.unwrap();

    assert_eq!(configuration["app"]["id"].as_i64().unwrap(), 5);
    assert_eq!(configuration["app"]["timeout"].as_i64().unwrap(), 100);
}

#[cfg(not(feature = "xml"))]
#[async_std::test]
#[should_panic(expected = "You have to enable feature `xml` for parsing this configuration kind.")]
async fn test_client_request_3() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.xml"],
        ..Default::default()
    };

    let _: Configuration<()> = Client::new(client_config).unwrap().request().await.unwrap();
}

#[cfg(feature = "xml")]
#[async_std::test]
async fn test_client_request_4() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.xml"],
        ..Default::default()
    };

    #[derive(Deserialize)]
    struct App {
        timeout: i32,
    }

    let configuration: Configuration<App> =
        Client::new(client_config).unwrap().request().await.unwrap();

    assert_eq!(configuration.timeout, 100);
}

#[async_std::test]
async fn test_client_request_5() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.json"],
        ..Default::default()
    };

    let configuration: Configuration<serde_json::Value> =
        Client::new(client_config).unwrap().request().await.unwrap();

    assert_eq!(configuration["timeout"].as_i64().unwrap(), 100);
}

#[async_std::test]
async fn test_client_request_6() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp".to_string(),
        namespace_names: vec!["application.txt".to_string()],
        ..Default::default()
    };

    let configuration: Configuration<String> =
        Client::new(client_config).unwrap().request().await.unwrap();

    assert_eq!(&*configuration, "timeout is 100");
}

#[async_std::test]
async fn test_client_request_7() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.properties"],
        ip: Some(IpValue::Custom("test-host-name")),
        ..Default::default()
    };

    let configuration: Configuration<HashMap<String, String>> =
        Client::new(client_config).unwrap().request().await.unwrap();

    assert_eq!(configuration["timeout"], "100");
}

#[async_std::test]
async fn test_client_request_8() {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application.properties"],
        ip: Some(IpValue::Custom("test-host-name")),
        ..Default::default()
    };

    let configuration: Configuration<HashMap<String, String>> = Client::new(client_config)
        .unwrap()
        .request_with_extras_query(Some(&[("noAudit", "1")]))
        .await
        .unwrap();

    assert_eq!(configuration["timeout"], "100");
}
