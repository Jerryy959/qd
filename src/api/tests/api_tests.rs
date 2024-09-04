use std::collections::HashMap;
use serde_json::json;
use reqwest::StatusCode;
use tokio;

#[tokio::test]
async fn test_send_request() {
    let url = "http://127.0.0.1";
    let port = 8080;
    let params = HashMap::new();
    match super::send_request(url, port, None, Some(params)).await {
        Ok(response) => {
            assert!(response.contains("Hello, world!"));
        }
        Err(e) => panic!("Failed to send GET request: {}", e),
    }
}

#[tokio::test]
async fn test_send_post() {
    let url = "http://127.0.0.1";
    let port = 8080;
    let params = HashMap::new();
    let json_body = json!({
        "name": "YourName"
    });
    match super::send_post(url, port, Some("/echo"), Some(params), json_body).await {
        Ok(response) => {
            assert!(response.contains("Hello, YourName!"));
        }
        Err(e) => panic!("Failed to send POST request: {}", e),
    }
}
