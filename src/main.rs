mod client;
mod server;
pub mod strategies;
mod utils;

use crate::{
    client::Client,
    server::Server,
    strategies::{jpeg_stream::JpegStream, stream_strategy::Strategy},
};
use image::ImageReader;
use std::{collections::HashSet, thread, time};
use strategies::random_strategy::RandomStream;
use utils::format_bytes;

fn main() {
    let img = ImageReader::open("example_images/osaka.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let compressed_bytes = std::fs::read("example_images/osaka.jpeg").unwrap();
    let strategy = JpegStream::new(0, 10, 10);

    println!("compressed size: {}", format_bytes(compressed_bytes.len()));

    let empty = strategy.generate_empty(&compressed_bytes);

    let mut client = Client::new(&strategy, empty);

    let mut server = Server::new(compressed_bytes, &strategy);

    client
        .get_current_image()
        .clone()
        .save("example_images/test.jpeg")
        .unwrap();

    let mut compression_step;
    let mut total_bytes_send = 0;

    for i in 0..10 {
        compression_step = server.send_step(i);
        let bytes_send = compression_step.current_size_in_bytes();
        total_bytes_send += bytes_send;
        println!("bytes: {}", format_bytes(bytes_send));

        client.update_image(compression_step);
        thread::sleep(time::Duration::from_millis(50));

        client
            .get_current_image()
            .clone()
            .save("example_images/test.jpeg")
            .unwrap()
    }

    println!("total bytes send {}", format_bytes(total_bytes_send))
}
