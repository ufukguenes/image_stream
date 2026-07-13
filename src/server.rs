use std::{cmp::{max, min}, collections::HashMap};
use image::DynamicImage;

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

pub struct Server<'a, S: Strategy<T>, T> {
    pub full_quality_image: DynamicImage,
    pub compression_step_cache: HashMap<usize, CompressionStep<T>>,
    pub strategy: &'a S,
}

impl<'a, S: Strategy<T>, T> Server<'a, S, T> {
    pub fn send_step(&mut self, current_step: usize) -> &CompressionStep<T> {
        let step_capped = min(
            max(self.strategy.get_total_number_of_steps(), 1),
            current_step,
        );

        self.compression_step_cache
            .entry(step_capped)
            .or_insert(self.strategy.step(&self.full_quality_image, step_capped))
    }
}