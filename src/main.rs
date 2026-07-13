mod client;
mod server;
pub mod strategies;

use image::ImageReader;
use strategies::random_strategy::RandomStream;

use crate::{client::Client, server::Server};

fn main() {
    let img = ImageReader::open("example_images/osaka.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let strategy = RandomStream::new(0, 10, 1000);

    let mut client = Client::new(&strategy, img.width(), img.height());

    let mut server = Server {
        full_quality_image: img,
        compression_step_cache: Default::default(),
        strategy: &strategy,
    };

    let mut compression_step;
    for i in 0..11 {
        compression_step = server.send_step(i);
        client.update_image( compression_step);

        client
            .reconstructed_image.clone()
            .save("example_images/test.jpeg")
            .unwrap()
    }
}
