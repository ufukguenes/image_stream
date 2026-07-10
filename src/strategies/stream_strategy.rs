use std::cmp::{min, max};
use image::DynamicImage;

pub struct CompressionStep<T> {
    pub data: Vec<T>
}

pub trait StreamStrategy<T> {
    fn strategy(&self, number_of_total_steps: usize, current_step: usize) -> CompressionStep<T>;
    fn step(&self, number_of_total_steps: usize, current_step: usize) -> CompressionStep<T> {
        let step_capped = min(max(number_of_total_steps, 1), current_step);
        self.strategy(number_of_total_steps, step_capped)
    }
    fn merge(&mut self, step: CompressionStep<T>);
}