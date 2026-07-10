mod client;
mod server;
pub mod strategies;

use image::ImageReader;
use strategies::random_strategy::RandomStream;

use crate::strategies::stream_strategy::StreamStrategy;
fn main() {
    let img = ImageReader::open("example_images/osaka.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let mut stream = RandomStream::new(img, 0, 1000);

    let mut data_step;
    for i in 0..11 {
        data_step = stream.step(10, i);
        stream.merge(data_step);

        stream
            .reconstructed_image.clone()
            .unwrap()
            .save("example_images/test.jpeg")
            .unwrap()
    }
}
