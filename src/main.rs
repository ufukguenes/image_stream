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
    
    print(img.size)
    let mut compression_step;
    for i in 0..11 {
        compression_step = server.send_step(i);
        let bytes_send = compression_step.current_size_in_bytes();
        println!("bytes: {}", format_bytes(bytes_send));
        client.update_image(compression_step);

        client
            .reconstructed_image
            .clone()
            .save("example_images/test.jpeg")
            .unwrap()
    }
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
