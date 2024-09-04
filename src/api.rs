use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;


pub async fn send_request(
  url: &str,
  port: u16,
  api_path: Option<&str>,
  params: Option<HashMap<&str, &str>>
) -> Result<String, Box<dyn std::error::Error>> {
  let api_path = api_path.unwrap_or("/");
  let full_url = format!("{}:{}{}", url.trim_end_matches('/'), port, api_path);

  #[cfg(debug_assertions)]
  println!("GET Request URL: {}", full_url);
  
  let client = reqwest::Client::new();
  let response = client
                .get(full_url)
                .query(&params.unwrap_or_default())
                .send()
                .await?
                .text()
                .await?;
  Ok(response)
}

pub async fn send_post(
  url: &str,
  port: u16,
  api_path: Option<&str>,
  params: Option<HashMap<&str, &str>>,
  json_body: Value
) -> Result<String, Box<dyn std::error::Error>> {
  let api_path = api_path.unwrap_or("/");
  let full_url = format!("{}:{}{}", url.trim_end_matches('/'), port, api_path);

  #[cfg(debug_assertions)]
  println!("POST Request URL: {}", full_url);

  let client = Client::new();
  let response = client
                .post(&full_url)
                .json(&json_body)
                .query(&params.unwrap_or_default())
                .send()
                .await?
                .text()
                .await?;
  Ok(response)
}
