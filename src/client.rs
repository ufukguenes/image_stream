use std::marker::PhantomData;
use image::DynamicImage;

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

pub struct Client<'a, S: Strategy<T>, T> {
    pub reconstructed_image: DynamicImage,
    pub strategy: &'a S,
    current_step: usize,
    _phantom: PhantomData<T>,
}

impl<'a, S: Strategy<T>, T> Client<'a, S, T> {
    pub fn new(strategy: &'a S, image_width: u32, image_height: u32) -> Client<'a, S, T> {
        let reconstructed_image: DynamicImage = DynamicImage::new_rgb8(image_width, image_height);
        Client {
            reconstructed_image,
            strategy,
            current_step: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, S: Strategy<T>, T> Client<'a, S, T> {
    pub fn update_image(&mut self, compression_step: &CompressionStep<T>) {
        if self.current_step < self.strategy.get_total_number_of_steps() {
            self.strategy
                .merge(&mut self.reconstructed_image, compression_step);
        }
    }
}
