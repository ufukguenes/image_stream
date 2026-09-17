use image::DynamicImage;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

pub struct Server<'a, S: Strategy<T, D>, T, D: Default> {
    full_quality_image: DynamicImage,
    compression_step_cache: HashMap<usize, CompressionStep<T>>,
    strategy: &'a S,
    full_quality_data: D,
}

impl<'a, S: Strategy<T, D>, T, D: Default> Server<'a, S, T, D> {
    pub fn new(full_quality_image: DynamicImage, strategy: &'a S) -> Self {
        let full_quality_data = strategy.to_data(&full_quality_image);

        Self {
            full_quality_image,
            compression_step_cache: HashMap::default(),
            strategy,
            full_quality_data,
        }
    }

    pub fn send_step(&mut self, current_step: usize) -> &CompressionStep<T> {
        let step_capped = min(
            max(self.strategy.get_total_number_of_steps(), 1),
            current_step,
        );

        self.compression_step_cache
            .entry(step_capped)
            .or_insert(self.strategy.step(&self.full_quality_data, step_capped))
    }
}
