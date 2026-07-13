use image::DynamicImage;

pub struct CompressionStep<T> {
    pub data: Vec<T>,
}

pub struct StreamData {
    pub full_quality_image: DynamicImage,
    pub reconstructed_image: Option<DynamicImage>,
    pub min_num_pixel: usize,
    pub number_of_total_steps: usize,
}

impl StreamData {
    pub fn new(
        full_quality_image: DynamicImage,
        min_num_pixel: usize,
        number_of_total_steps: usize,
    ) -> StreamData {
        StreamData { full_quality_image, reconstructed_image: None, min_num_pixel, number_of_total_steps }
    }
}

pub trait StreamStrategy<T> {
    fn step(&self, stream: &StreamData, current_step: usize) -> CompressionStep<T>;
    fn merge(&self, stream: &mut StreamData, step: CompressionStep<T>);
}
