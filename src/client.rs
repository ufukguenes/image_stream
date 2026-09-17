use image::DynamicImage;
use std::marker::PhantomData;

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

pub struct Client<'a, S: Strategy<T, D>, T, D: Default> {
    pub reconstructed_image: DynamicImage,
    pub strategy: &'a S,
    current_step: usize,
    data: D,
    _phantom_0: PhantomData<T>,
}

impl<'a, S: Strategy<T, D>, T, D: Default> Client<'a, S, T, D> {
    pub fn new(strategy: &'a S, image_width: u32, image_height: u32) -> Client<'a, S, T, D> {
        let reconstructed_image: DynamicImage = DynamicImage::new_rgb8(image_width, image_height);
        let data = D::default();
        Client {
            reconstructed_image,
            strategy,
            current_step: 0,
            data,
            _phantom_0: PhantomData,
        }
    }
}

impl<'a, S: Strategy<T, D>, T, D: Default> Client<'a, S, T, D> {
    pub fn update_image(&mut self, compression_step: &CompressionStep<T>) {
        if self.current_step < self.strategy.get_total_number_of_steps() {
            let mut data = self.strategy.to_data(&self.reconstructed_image);
            self.strategy.merge(&mut data, compression_step);
        }
    }
}
