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
    pub fn new(strategy: &'a S, empty: D) -> Client<'a, S, T, D> {
        Client {
            strategy,
            current_step: 0,
            data: empty,
            _phantom_0: PhantomData,
        }
    }

    pub fn update_image(&mut self, compression_step: &CompressionStep<T>) {
        if self.current_step < self.strategy.get_total_number_of_steps() {
            self.strategy
                .merge(&mut self.data, compression_step, self.current_step);
            self.current_step += 1;
        }
    }

    pub fn get_current_image(&self) -> DynamicImage {
        self.strategy.to_image(&self.data)
    }
}
