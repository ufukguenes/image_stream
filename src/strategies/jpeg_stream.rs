use std::{cmp::min, collections::HashSet, io::Cursor, process::id};

use image::{DynamicImage, ImageReader};
use rand::{SeedableRng, rngs::ChaCha8Rng, seq::SliceRandom};

use crate::strategies::{compression_step::CompressionStep, stream_strategy::Strategy};

//todo: send 0xFF 0xEn (application specific meta data) last

//todo check format off huffman tables and then send data accordingly
pub struct JpegStream {
    total_number_of_steps: usize,
    markers: HashSet<(u8, u8)>,
    seed: u64,
    min_num_data: usize,
}

impl JpegStream {
    pub fn new(seed: u64, total_number_of_steps: usize, min_num_data: usize) -> Self {
        let markers = Self::init_markers();
        JpegStream {
            seed,
            total_number_of_steps,
            markers,
            min_num_data,
        }
    }
}

impl Strategy<(Vec<(usize, u8)>), Vec<u8>> for JpegStream {
    fn step(&self, data: &Vec<u8>, current_step: usize) -> CompressionStep<Vec<(usize, u8)>> {
        println!("step");
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(self.seed);

        let until_entropy = Self::get_index_until_entropy(data).unwrap();
        let data_to_send = &data[until_entropy + 1..];

        let data_per_step = data_to_send.len() / self.total_number_of_steps;

        let mut data_idxs: Vec<usize> = (0..data_to_send.len()).collect();
        data_idxs.shuffle(&mut rng);

        let start_idx = data_per_step * current_step + min(current_step, 1) * self.min_num_data;
        let end_idx = min(
            data_to_send.len(),
            data_per_step * (current_step + 1) + self.min_num_data,
        );

        println!(
            "data_per_step {}, start_idx {}, end_idx {}",
            data_per_step, start_idx, end_idx
        );

        let data = &data_idxs[start_idx..end_idx];

        // todo this sends double the data, as the idx is send as well
        let new_data: Vec<(usize, u8)> = data
            .iter()
            .map(|idx| (*idx + until_entropy + 1, data_to_send[*idx]))
            .collect();

        CompressionStep { data: new_data }
    }

    fn merge(
        &self,
        current_data: &mut Vec<u8>,
        compression_step: &CompressionStep<Vec<(usize, u8)>>,
    ) {
        for (idx, data) in &compression_step.data {
            current_data[*idx] = *data;
        }
    }

    fn get_total_number_of_steps(&self) -> usize {
        self.total_number_of_steps
    }

    fn to_data(&self, image: &image::DynamicImage) -> Vec<u8> {
        let mut buf = Cursor::new(Vec::new());
        image
            .write_to(&mut buf, image::ImageFormat::Jpeg)
            .expect("failed to encode image as JPEG");
        buf.into_inner()
    }

    fn to_image(&self, data: &Vec<u8>) -> image::DynamicImage {
        ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    }

    fn generate_empty(&self, original_data: &Vec<u8>) -> Vec<u8> {
        let len = original_data.len();
        let mut empty = vec![0; len];
        let marker_idxs = self.list_marker_idxs(original_data);
        let until_entropy = Self::get_index_until_entropy(original_data).unwrap();
        empty[..until_entropy + 1].copy_from_slice(&original_data[..until_entropy + 1]);

        for idx in marker_idxs {
            empty[idx.0] = original_data[idx.0];
            empty[idx.1] = original_data[idx.1];
        }
        empty
    }
}

impl JpegStream {
    pub fn get_index_until_entropy(byte_stream: &[u8]) -> Option<usize> {
        let sos_idx = Self::find_sos_marker_index(byte_stream).unwrap();

        let sos_marker_end = sos_idx.1 + 1; // index of 0xDA
        let sos_len = ((byte_stream[sos_marker_end] as usize) << 8)
            | byte_stream[sos_marker_end + 1] as usize;
        Some(sos_marker_end + sos_len) // marker(2) + length(2) + payload
    }

    pub fn find_sos_marker_index(byte_stream: &[u8]) -> Option<(usize, usize)> {
        for idx in 1..byte_stream.len() {
            if byte_stream[idx - 1] == 0xFF && byte_stream[idx] == 0xDA {
                return Some((idx - 1, idx));
            }
        }
        None
    }

    pub fn init_markers() -> HashSet<(u8, u8)> {
        let mut markers = HashSet::new();
        markers.insert((0xFF, 0xD8));
        markers.insert((0xFF, 0xC0));
        markers.insert((0xFF, 0xC2));
        markers.insert((0xFF, 0xC4));
        markers.insert((0xFF, 0xDB));
        markers.insert((0xFF, 0xDD));
        markers.insert((0xFF, 0xDA));

        for i in 0xD0..0xD8 {
            markers.insert((0xFF, i));
        }

        for i in 0xE0..0xE8 {
            markers.insert((0xFF, i));
        }

        markers.insert((0xFF, 0xFE));
        markers.insert((0xFF, 0xD9));

        markers
    }

    pub fn list_marker_idxs(&self, byte_stream: &[u8]) -> Vec<(usize, usize)> {
        let mut idxs = Vec::default();
        for i in 1..byte_stream.len() {
            let contains = self.markers.contains(&(byte_stream[i - 1], byte_stream[i]));
            if contains {
                idxs.push((i - 1, i));
            }
        }
        idxs
    }

    pub fn find_marker_index(&self, byte_stream: &[u8], marker: (u8, u8)) -> bool {
        let mut segment_length = 0_usize;
        let mut previous_segment_idx = 0_usize;
        for i in 1..byte_stream.len() {
            let byte_pair = (byte_stream[i - 1], byte_stream[i]);
            let contains = self.markers.contains(&byte_pair);
            if contains {
                let next = byte_stream.get(i + 1).unwrap_or(&0);
                let next_2 = byte_stream.get(i + 2).unwrap_or(&0);
                println!(
                    "marker found at index: {}, {} - hex: {:x}{:x} - next bytes: {}, {}, length of previous segment {}",
                    i - 1,
                    i,
                    byte_pair.0,
                    byte_pair.1,
                    next,
                    next_2,
                    segment_length - previous_segment_idx
                );
                previous_segment_idx = i + 1;
            }
            segment_length += 1;

            if contains && byte_pair == marker {
                //return true;
            }
        }
        false
    }
}
