mod client;
mod server;
pub mod strategies;

use image::ImageReader;
use strategies::random_strategy::RandomStream;

use crate::strategies::stream_strategy::{StreamData, StreamStrategy};
fn main() {
    let img = ImageReader::open("example_images/osaka.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    
    let mut stream_data = StreamData::new(img, 1000, 10);
    let stream = RandomStream::new(0);


    let mut data_step;
    for i in 0..11 {
        data_step = stream.step(&stream_data, i);
        stream.merge(&mut stream_data, data_step);

        stream_data
            .reconstructed_image.clone()
            .unwrap()
            .save("example_images/test.jpeg")
            .unwrap()
    }
}
