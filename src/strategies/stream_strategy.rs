use image::DynamicImage;

use crate::strategies::compression_step::CompressionStep;

pub trait Strategy<T, D> {
    fn step(&self, data: &D, current_step: usize) -> CompressionStep<T>;
    fn merge(&self, current_data: &mut D, compression_step: &CompressionStep<T>);
    fn get_total_number_of_steps(&self) -> usize;
    fn to_data(&self, image: &DynamicImage) -> D;
    fn to_image(&self, data: &D) -> DynamicImage;
    fn generate_empty(&self, image_width_: u32, image_heith: u32) -> D;
}
