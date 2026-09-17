use image::DynamicImage;
use std::marker::PhantomData;

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

pub struct Client<'a, S: Strategy<T, D>, T, D> {
    pub strategy: &'a S,
    current_step: usize,
    data: D,
    _phantom_0: PhantomData<T>,
}

impl<'a, S: Strategy<T, D>, T, D> Client<'a, S, T, D> {
    pub fn new(strategy: &'a S, image_width: u32, image_height: u32) -> Client<'a, S, T, D> {
        let data = strategy.generate_empty(image_width, image_height);
        Client {
            strategy,
            current_step: 0,
            data,
            _phantom_0: PhantomData,
        }
    }

    pub fn update_image(&mut self, compression_step: &CompressionStep<T>) {
        if self.current_step < self.strategy.get_total_number_of_steps() {
            self.strategy.merge(&mut self.data, compression_step);
        }
    }

    pub fn get_current_image(&self) -> DynamicImage {
        self.strategy.to_image(&self.data)
    }
}
