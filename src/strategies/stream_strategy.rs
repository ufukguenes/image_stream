use image::DynamicImage;

use crate::strategies::compression_step::CompressionStep;



pub trait Strategy<T> {
    fn step(&self, image: &DynamicImage, current_step: usize) -> CompressionStep<T>;
    fn merge(&self, current_image: &mut DynamicImage, compression_step: &CompressionStep<T>);
    fn get_total_number_of_steps(&self) -> usize;
}
