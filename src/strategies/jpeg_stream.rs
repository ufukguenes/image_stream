use std::{collections::HashSet, io::Cursor, process::id};

use image::{DynamicImage, ImageReader};

use crate::strategies::stream_strategy::Strategy;

pub struct JpegStream {
    total_number_of_steps: usize,
    markers: HashSet<(u8, u8)>,
}

impl JpegStream {
    pub fn new(total_number_of_steps: usize) -> Self {
        let markers = Self::init_markers();
        JpegStream {
            total_number_of_steps,
            markers,
        }
    }
}

impl Strategy<Vec<u8>, Vec<u8>> for JpegStream {
    fn step(
        &self,
        data: &Vec<u8>,
        current_step: usize,
    ) -> super::compression_step::CompressionStep<Vec<u8>> {
        todo!()
    }

    fn merge(
        &self,
        current_data: &mut Vec<u8>,
        compression_step: &super::compression_step::CompressionStep<Vec<u8>>,
    ) {
        todo!()
    }

    fn get_total_number_of_steps(&self) -> usize {
        todo!()
    }

    fn to_data(&self, image: &image::DynamicImage) -> Vec<u8> {
        image.as_bytes().to_vec()
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
        let sos_idx = Self::find_sos_marker_index(original_data).unwrap();

        let sos_marker_end = sos_idx.1 + 1; // index of 0xDA
        let sos_len = ((original_data[sos_marker_end + 1] as usize) << 8)
            | original_data[sos_marker_end + 2] as usize;
        let sos_header_end = sos_marker_end + 1 + sos_len; // marker(2) + length(2) + payload
        empty[..sos_header_end].copy_from_slice(&original_data[..sos_header_end]);

        for idx in marker_idxs {
            empty[idx.0] = original_data[idx.0];
            empty[idx.1] = original_data[idx.1];
        }
        empty
    }
}

impl JpegStream {
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
        markers.insert((0xFF, 0xD8));
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
        for i in 1..byte_stream.len() {
            let byte_pair = (byte_stream[i - 1], byte_stream[i]);
            let contains = self.markers.contains(&byte_pair);
            if contains {
                let next = byte_stream.get(i + 1).unwrap_or(&0);
                let next_2 = byte_stream.get(i + 2).unwrap_or(&0);
                println!(
                    "marker found at index: {}, {} - hex: {:x}{:x} - next bytes: {:x}, {:x}, length of previous segment {}",
                    i - 1,
                    i,
                    byte_pair.0,
                    byte_pair.1,
                    next,
                    next_2,
                    segment_length
                );
            }
            segment_length += 1;

            if contains && byte_pair == marker {
                //return true;
            }
        }
        false
    }
}
