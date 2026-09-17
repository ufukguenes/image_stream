use std::collections::HashSet;

use crate::strategies::stream_strategy::Strategy;

pub struct JpegStream {
    total_number_of_steps: usize,
    markers: HashSet<u16>,
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
        todo!()
    }

    fn to_image(&self, data: &Vec<u8>) -> image::DynamicImage {
        todo!()
    }
}

impl JpegStream {
    pub fn init_markers() -> HashSet<u16> {
        let mut markers = HashSet::new();
        markers.insert(merge_u8s(0xFF, 0xD8));
        markers.insert(merge_u8s(0xFF, 0xC0));
        markers.insert(merge_u8s(0xFF, 0xC2));
        markers.insert(merge_u8s(0xFF, 0xC4));
        markers.insert(merge_u8s(0xFF, 0xD8));
        markers.insert(merge_u8s(0xFF, 0xDD));
        markers.insert(merge_u8s(0xFF, 0xDA));

        for i in 0xD0..0xD8 {
            markers.insert(merge_u8s(0xFF, i));
        }

        for i in 0xE0..0xE8 {
            markers.insert(merge_u8s(0xFF, i));
        }

        markers.insert(merge_u8s(0xFF, 0xFE));
        markers.insert(merge_u8s(0xFF, 0xD9));

        markers
    }

    pub fn find_marker_index(&self, byte_stream: &[u8], marker: u16) -> bool {
        for i in 1..byte_stream.len() {
            let two_bytes = merge_u8s(byte_stream[i - 1], byte_stream[i]);
            let contains = self.markers.contains(&two_bytes);
            if contains {
                let next = byte_stream.get(i + 1).unwrap_or(&0);
                let next_2 = byte_stream.get(i + 2).unwrap_or(&0);
                println!(
                    "marker found at index: {}, {} hex: {:x} next 2 bytes {:x}, {:x}",
                    i - 1,
                    i,
                    two_bytes,
                    next,
                    next_2
                );
            }

            if contains && two_bytes == marker {
                //return true;
            }
        }
        false
    }
}

fn merge_u8s(a: u8, b: u8) -> u16 {
    let mut number: u16 = a as u16;
    number <<= 8;
    number |= b as u16;
    number
}
