use data_encoding::HEXLOWER;
use reqwest::header;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use crate::registry::Registry;

pub struct Image<'a> {
  name: String,
  registry: &'a Registry,
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
  let mut context = Context::new(&SHA256);
  let mut buffer = [0; 1024];

  loop {
    let count = reader.read(&mut buffer)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }

  Ok(context.finish())
}

impl<'a> Image<'a> {
  pub fn new(name: &str, registry: &'a Registry) -> Self {
    Image {
      name: name.to_string(),
      registry: registry,
    }
  }

  async fn download_digest(
    &self,
    url: &str,
    digest: &str,
    token: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("/tmp/{}", digest);

    if Path::new(&path).exists() {
      let input = File::open(&path)?;
      let file_digest = HEXLOWER.encode(sha256_digest(BufReader::new(input))?.as_ref());

      let len = digest.len();
      if file_digest == digest[7..len] {
        return Ok(());
      }
    }

    let mut headers = header::HeaderMap::new();

    // TODO: avoid using trim_quotes
    let token = format!("Bearer {}", token);
    if let Ok(token_val) = header::HeaderValue::from_str(&token) {
      headers.insert(header::AUTHORIZATION, token_val);
    }

    let mut dest = OpenOptions::new().create(true).append(true).open(path)?;

    let content = reqwest::Client::builder()
      .default_headers(headers.clone())
      .build()?
      .get(url)
      .send()
      .await?
      .bytes()
      .await?;

    dest.write_all(&content[..]).unwrap();

    Ok(())
  }

  pub async fn pull_image(&self, tag: &str) {
    let token = &self.registry.get_token(&self.name).await;
    println!("{}", token);
    let fs_layers = self.registry.get_manifests(&self.name, tag, &token).await;

    for layer in fs_layers {
      let digest = &layer["blobSum"].as_str().unwrap();
      println!("{}", digest);
      let url = format!(
        "https://{}/v2/library/{}/blobs/{}",
        self.registry.service_uri, self.name, digest
      );
      let _ = self.download_digest(&url, &digest, &token).await;
    }
  }
}
