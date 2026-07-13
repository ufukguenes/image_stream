use image::DynamicImage;
use std::marker::PhantomData;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub struct CompressionStep<T> {
    pub data: T,
}

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

pub struct Client<'a, S: Strategy<T>, T> {
    pub reconstructed_image: DynamicImage,
    pub strategy: &'a S,
    current_step: usize,
    _phantom: PhantomData<T>,
}

impl<'a, S: Strategy<T>, T> Client<'a, S, T> {
    pub fn new(strategy: &'a S, image_width: u32, image_height: u32) -> Client<'a, S, T> {
        let reconstructed_image: DynamicImage = DynamicImage::new_rgb8(image_width, image_height);
        Client {
            reconstructed_image,
            strategy,
            current_step: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, S: Strategy<T>, T> Client<'a, S, T> {
    pub fn update_image(&mut self, compression_step: &CompressionStep<T>) {
        if self.current_step < self.strategy.get_total_number_of_steps() {
            self.strategy
                .merge(&mut self.reconstructed_image, compression_step);
        }
    }
}

pub trait Strategy<T> {
    fn step(&self, image: &DynamicImage, current_step: usize) -> CompressionStep<T>;
    fn merge(&self, current_image: &mut DynamicImage, compression_step: &CompressionStep<T>);
    fn get_total_number_of_steps(&self) -> usize;
}
