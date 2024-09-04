mod api;
use std::collections::HashMap;
use tokio;
use api::{send_request, send_post};

#[tokio::main]
async fn main() {
  let params = None;
  match send_request("http://127.0.0.1", 8080, None, params.clone()).await {
      Ok(result) => println!("GET Response: {}", result),
      Err(e) => eprintln!("GET Error: {}", e),
  }
  let json_body = serde_json::json!({
      "name": "YourName"
  });
  match send_post("http://127.0.0.1", 8080, Some("/echo"), params, json_body).await {
      Ok(result) => println!("POST Response: {}", result),
      Err(e) => eprintln!("POST Error: {}", e),
  }
}
