use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;
use std::cmp::min;
use std::usize;

use crate::strategies::compression_step::CompressionStep;
use crate::strategies::stream_strategy::Strategy;

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

impl Strategy<Vec<Rgba<u8>>, DynamicImage> for RandomStream {
    fn step(&self, image: &DynamicImage, current_step: usize) -> CompressionStep<Vec<Rgba<u8>>> {
        let (pixel_idxs, start_idx, end_idx) =
            self.get_pixel_idx_start_end_idx_for_step(image, current_step);
        let data = &pixel_idxs[start_idx..end_idx];

        let new_pixels: Vec<Rgba<u8>> = data
            .iter()
            .map(|flattened_pixel| {
                let row = *flattened_pixel as u32 / image.width();
                let col = *flattened_pixel as u32 % image.width();
                image.get_pixel(col, row)
            })
            .collect();

        CompressionStep { data: new_pixels }
    }

    fn merge(
        &self,
        current_image: &mut DynamicImage,
        compression_step: &CompressionStep<Vec<Rgba<u8>>>,
        current_step: usize,
    ) {
        let (pixel_idxs, start_idx, _) =
            self.get_pixel_idx_start_end_idx_for_step(current_image, current_step);

        for (i, pix) in compression_step.data.iter().enumerate() {
            let flattened_pixel = pixel_idxs[start_idx + i];
            let row = flattened_pixel as u32 / current_image.width();
            let col = flattened_pixel as u32 % current_image.width();
            current_image.put_pixel(col, row, *pix);
        }
    }

    fn get_total_number_of_steps(&self) -> usize {
        self.total_number_of_steps
    }

    fn to_data(&self, image: &DynamicImage) -> DynamicImage {
        image.clone()
    }

    fn to_image(&self, data: &DynamicImage) -> DynamicImage {
        data.clone()
    }

    fn generate_empty(&self, original_data: &DynamicImage) -> DynamicImage {
        DynamicImage::new_rgb8(original_data.width(), original_data.height())
    }
}

impl RandomStream {
    fn get_pixel_idx_start_end_idx_for_step(
        &self,
        image: &DynamicImage,
        current_step: usize,
    ) -> (Vec<usize>, usize, usize) {
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(self.seed);
        let width = image.width() as usize;
        let height = image.height() as usize;

        let image_size = width * height;
        let pixel_per_step = image_size / self.total_number_of_steps;

        let mut pixel_idxs: Vec<usize> = (0..image_size).collect();
        pixel_idxs.shuffle(&mut rng);

        let start_idx = pixel_per_step * current_step + min(current_step, 1) * self.min_num_pixel;
        let end_idx = min(
            image_size,
            pixel_per_step * (current_step + 1) + self.min_num_pixel,
        );

        println!(
            "pixel_per_step {}, start_idx {}, end_idx {}",
            pixel_per_step, start_idx, end_idx
        );

        (pixel_idxs, start_idx, end_idx)
    }
}
