use futures::stream::TryStreamExt;
use ipnetwork::IpNetwork;
use rand::Rng;
use rtnetlink;
use tokio;

use crate::image::Image;

pub struct Container<'a> {
  name: String,
  image: &'a Image<'a>,
}

fn rand_name() -> String {
  let mut rng = rand::thread_rng();
  let adj = ["speedy", "furious", "calm", "cunning"];
  let names = ["gorilla", "fox", "lion", "cheetah"];
  format!(
    "{}_{}",
    adj[rng.gen_range(0, 4)],
    names[rng.gen_range(0, 4)]
  )
}

impl<'a> Container<'a> {
  pub fn new(image: &'a Image) -> Self {
    Container {
      name: rand_name(),
      image: image,
    }
  }

  #[tokio::main]
  async fn setup_net_bridge(&self) -> Result<(), String> {
    let (connection, handle, _) = rtnetlink::new_connection().unwrap();
    tokio::spawn(connection);
    handle
      .link()
      .add()
      .bridge("my-bridge-1".into())
      .execute()
      .await
      .map_err(|e| format!("{}", e))
  }

  #[tokio::main]
  async fn add_address(&self, link_name: &str) -> Result<(), String> {
    let (connection, handle, _) = rtnetlink::new_connection().unwrap();
    tokio::spawn(connection);

    let ip = IpNetwork::V4("172.29.0.1/16".parse().unwrap());

    let mut links = handle
      .link()
      .get()
      .set_name_filter(link_name.to_string())
      .execute();

    match links.try_next().await {
      Ok(Some(link)) => handle
        .address()
        .add(link.header.index, ip.ip(), ip.prefix())
        .execute()
        .await
        .map_err(|e| format!("{}", e)),
      Ok(None) => Err("hello".to_string()),
      Err(_) => Err("hello".to_string()),
    }
  }

  pub async fn init(self) {
    // //println!("{:?}", container.setup_net_bridge());
    // println!("{:?}", container.add_address("my-bridge-1"));
    println!("{}", self.name);
    self.image.pull_image("latest").await;
  }
}
