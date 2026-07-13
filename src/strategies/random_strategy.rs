use std::cmp::{max, min};

use crate::strategies::stream_strategy::{CompressionStep, StreamStrategy, StreamData};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;

pub struct RandomStream {
    seed: u64,
}

impl RandomStream {
    pub fn new(seed: u64) -> RandomStream {
        RandomStream { seed }
    }
}

impl StreamStrategy<(u32, u32, Rgba<u8>)> for RandomStream {
    fn step(
        &self,
        stream: &StreamData,
        current_step: usize,
    ) -> CompressionStep<(u32, u32, Rgba<u8>)> {
        let step_capped = min(max(stream.number_of_total_steps, 1), current_step);

        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(self.seed);
        let width = stream.full_quality_image.width() as usize;
        let height = stream.full_quality_image.height() as usize;

        let image_size = width * height;
        let pixel_per_step = image_size / stream.number_of_total_steps;

        let mut pixel_idxs: Vec<usize> = (0..image_size).collect();
        pixel_idxs.shuffle(&mut rng);

        let start_idx = pixel_per_step * step_capped;
        let end_idx = min(
            image_size,
            pixel_per_step * (step_capped + 1) + stream.min_num_pixel,
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
                let pix = stream.full_quality_image.get_pixel(col, row);
                (col, row, pix)
            })
            .collect();

        CompressionStep { data: new_pixels }
    }

    fn merge(&self, stream: &mut StreamData, step: CompressionStep<(u32, u32, Rgba<u8>)>) {
        let width = stream.full_quality_image.width() as usize;
        let height = stream.full_quality_image.height() as usize;

        match &mut stream.reconstructed_image {
            None => {
                let mut img = DynamicImage::new_rgb8(width as u32, height as u32);

                for (col, row, pixel) in step.data {
                    img.put_pixel(col, row, pixel);
                }
                stream.reconstructed_image = Some(img)
            }
            Some(img) => {
                for (col, row, pixel) in step.data {
                    img.put_pixel(col, row, pixel);
                }
            }
        }
    }
}
