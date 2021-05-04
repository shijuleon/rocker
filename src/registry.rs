use reqwest::header;
use serde_json;

use crate::utils;

pub struct Registry {
  pub service_uri: String,
  pub index_uri: String,
  pub auth_uri: String,
}

impl Registry {
  pub fn new(service_uri: String, index_uri: String, auth_uri: String) -> Self {
    Registry {
      service_uri,
      index_uri,
      auth_uri,
    }
  }

  pub async fn get_token(&self, image_name: &str) -> String {
    let url = format!(
      "https://{}/token?service={}&scope=repository:library/{}:pull",
      self.auth_uri, self.service_uri, image_name
    );

    utils::make_request(&url, &header::HeaderMap::new())
      .await
      .unwrap()["token"]
      .as_str()
      .unwrap()
      .to_string()
  }

  pub async fn get_manifests(
    &self,
    image_name: &str,
    tag: &str,
    token: &str,
  ) -> Vec<serde_json::Value> {
    let mut headers = header::HeaderMap::new();

    // TODO: avoid using trim_quotes
    let token = format!("Bearer {}", &token);
    if let Ok(token_val) = header::HeaderValue::from_str(&token) {
      headers.insert(header::AUTHORIZATION, token_val);
    }

    let url = format!(
      "https://{}/v2/library/{}/manifests/{}",
      self.index_uri, image_name, tag
    );
    let resp = utils::make_request(&url, &headers).await.unwrap();

    resp["fsLayers"].as_array().unwrap().to_vec()
  }
}
