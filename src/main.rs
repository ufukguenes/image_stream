mod client;
mod server;
pub mod strategies;

use image::ImageReader;
use strategies::random_strategy::RandomStream;

use crate::{
    client::Client,
    server::Server,
    strategies::{jpeg_stream::JpegStream, stream_strategy::Strategy},
};

fn main() {
    let img = ImageReader::open("example_images/osaka.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let compressed_bytes = std::fs::read("example_images/osaka.jpeg").unwrap();
    let jpeg_stream = JpegStream::new(0);
    jpeg_stream.find_marker_index(&compressed_bytes, 0);

    //return;
    println!("compressed size: {}", format_bytes(compressed_bytes.len()));

    let strategy = RandomStream::new(0, 10, 1000);

    let empty = strategy.generate_empty(&img);
    let mut client = Client::new(&strategy, empty);

    let mut server = Server::new(img, &strategy);

    let mut compression_step;
    let mut total_bytes_send = 0;
    for i in 0..11 {
        compression_step = server.send_step(i);
        let bytes_send = compression_step.current_size_in_bytes();
        total_bytes_send += bytes_send;
        println!("bytes: {}", format_bytes(bytes_send));
        client.update_image(compression_step);

        client
            .get_current_image()
            .clone()
            .save("example_images/test.jpeg")
            .unwrap()
    }

    println!("total bytes send {}", format_bytes(total_bytes_send))
}

fn format_bytes(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}
