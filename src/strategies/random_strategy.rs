use crate::strategies::stream_strategy::{CompressionStep, StreamStrategy};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;

struct RandomStream {
    full_quality_image: DynamicImage,
    pub reconstructed_image: Option< DynamicImage>,
    seed: u64,
    min_num_pixel: usize,
}

impl RandomStream {
    pub fn new(full_quality_image: DynamicImage, seed: u64, min_num_pixel: usize) -> RandomStream {
        let reconstructed_image: Option<DynamicImage> = None;
        RandomStream {
            full_quality_image,
            reconstructed_image,
            seed,
            min_num_pixel,
        }
    }
}

impl StreamStrategy<(u32, u32, Rgba<u8>)> for RandomStream {
    fn strategy(
        &self,
        number_of_total_steps: usize,
        current_step: usize,
    ) -> CompressionStep<(u32, u32, Rgba<u8>)> {
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(self.seed);
        let width = self.full_quality_image.width() as usize;
        let height = self.full_quality_image.height() as usize;

        let image_size = width * height;
        let pixel_per_step = image_size / number_of_total_steps;

        let mut pixel_idxs: Vec<usize> = (0..image_size).collect();
        pixel_idxs.shuffle(&mut rng);

        let start_idx = pixel_per_step * current_step + self.min_num_pixel;

        let data = &pixel_idxs[start_idx..start_idx + pixel_per_step];
        
        let new_pixels: Vec<(u32, u32, Rgba<u8>)> = data
            .iter()
            .map(|flattened_pixel| {
                let row = (flattened_pixel / width) as u32;
                let col = (flattened_pixel % width) as u32;
                let pix = self.full_quality_image.get_pixel(col, row);
                (col, row, pix )
            })
            .collect();

        CompressionStep { data: new_pixels }
    }

    fn merge(&mut self, step: CompressionStep<(u32, u32, Rgba<u8>)>) {
        let width = self.full_quality_image.width() as usize;
        let height = self.full_quality_image.height() as usize;

        match &mut self.reconstructed_image {
            None => {
                let mut img = DynamicImage::new_rgb8(width as u32, height as u32);

                for (col, row, pixel) in step.data {
                    img.put_pixel(col, row, pixel);
                }
                self.reconstructed_image = Some(img)
            },
            Some(img) => {
                for (col, row, pixel) in step.data {
                    img.put_pixel(col, row, pixel);
                }
            },
        }
    }
}
