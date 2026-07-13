use std::cmp::min;

use crate::strategies::stream_strategy::{CompressionStep, Strategy};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;

pub struct RandomStream {
    seed: u64,
    total_number_of_steps: usize,
    min_num_pixel: usize,
}

impl RandomStream {
    pub fn new(seed: u64, total_number_of_steps: usize, min_num_pixel: usize) -> RandomStream {
        RandomStream {
            seed,
            total_number_of_steps,
            min_num_pixel,
        }
    }
}

impl Strategy<Vec<(u32, u32, Rgba<u8>)>> for RandomStream {
    fn step(
        &self,
        image: &DynamicImage,
        current_step: usize,
    ) -> CompressionStep<Vec<(u32, u32, Rgba<u8>)>> {
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(self.seed);
        let width = image.width() as usize;
        let height = image.height() as usize;

        let image_size = width * height;
        let pixel_per_step = image_size / self.total_number_of_steps;

        let mut pixel_idxs: Vec<usize> = (0..image_size).collect();
        pixel_idxs.shuffle(&mut rng);

        let start_idx = pixel_per_step * current_step;
        let end_idx = min(
            image_size,
            pixel_per_step * (current_step + 1) + self.min_num_pixel,
        );

        println!(
            "pixel_per_step {}, start_idx {}, end_idx {}",
            pixel_per_step, start_idx, end_idx
        );

        let data = &pixel_idxs[start_idx..end_idx];

        let new_pixels: Vec<(u32, u32, Rgba<u8>)> = data
            .iter()
            .map(|flattened_pixel| {
                let row = (flattened_pixel / width) as u32;
                let col = (flattened_pixel % width) as u32;
                let pix = image.get_pixel(col, row);
                (col, row, pix)
            })
            .collect();

        CompressionStep { data: new_pixels }
    }

    fn merge(
        &self,
        current_image: &mut DynamicImage,
        compression_step: &CompressionStep<Vec<(u32, u32, Rgba<u8>)>>,
    ) {
        for (col, row, pixel) in &compression_step.data {
                    current_image.put_pixel(*col, *row, *pixel);
                }
    }

    fn get_total_number_of_steps(&self) -> usize {
        self.total_number_of_steps
    }
}
