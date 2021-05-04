mod container;
mod image;
mod registry;
mod rocker;
mod utils;

use crate::rocker::Rocker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let command = std::env::args().nth(1);
  let mut rocker = Rocker::new();
  rocker.init();

  match command {
    Some(_) => {
      let image_name = std::env::args().nth(2).unwrap();
      rocker.run(&image_name).await;
    }
    _ => println!("invalid command"),
  };

  Ok(())
}
