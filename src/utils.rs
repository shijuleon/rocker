use reqwest::header;
use serde_json;

pub async fn make_request(
  url: &str,
  headers: &header::HeaderMap,
) -> Result<Box<serde_json::Value>, Box<dyn std::error::Error>> {
  let client = reqwest::Client::builder()
    .default_headers(headers.clone())
    .build()?;

  // TODO: avoid using serde_json::Value and use derive
  let resp = client
    .get(url)
    .send()
    .await?
    .json::<serde_json::Value>()
    .await?;

  Ok(Box::new(resp))
}

// pub fn trim_quotes(value: &str) -> String {
//     return value[1..value.len()-1].to_string()
// }
