use crate::container::Container;
use crate::image::Image;
use crate::registry::Registry;

pub struct Rocker {
  // replicating Docker's path structure
  home_path: String,
  containers_path: String,
  images_path: String,
  registry: Registry,
}

impl Rocker {
  pub fn new() -> Self {
    Rocker {
      home_path: "/var/lib/rocker".to_string(),
      containers_path: "".to_string(),
      images_path: "".to_string(),
      registry: Registry::new(
        "registry.docker.io".to_string(),
        "index.docker.io".to_string(),
        "auth.docker.io".to_string(),
      ),
    }
  }
  pub fn init(&mut self) {
    self.home_path = "/var/lib/rocker".to_string();
    self.containers_path = self.home_path.clone() + "/containers";
    self.images_path = self.home_path.clone() + "/image";
  }

  pub async fn run(&self, image_name: &str) {
    let image = Image::new(image_name, &self.registry);

    let container = Container::new(&image);
    container.init().await;
  }

  pub fn info(&self) {}
}
