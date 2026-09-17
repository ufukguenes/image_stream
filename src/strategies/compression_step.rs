pub struct CompressionStep<T> {
    pub data: T,
}

pub trait HeapSize {
    fn heap_size_in_bytes(&self) -> usize;
}

impl HeapSize for i32 {
    fn heap_size_in_bytes(&self) -> usize {
        0
    }
}

impl HeapSize for u32 {
    fn heap_size_in_bytes(&self) -> usize {
        0
    }
}

impl HeapSize for u8 {
    fn heap_size_in_bytes(&self) -> usize {
        0
    }
}

impl HeapSize for image::Rgba<u8> {
    fn heap_size_in_bytes(&self) -> usize {
        0
    }
}

impl<A: HeapSize, B: HeapSize, C: HeapSize> HeapSize for (A, B, C) {
    fn heap_size_in_bytes(&self) -> usize {
        self.0.heap_size_in_bytes() + self.1.heap_size_in_bytes() + self.2.heap_size_in_bytes()
    }
}

impl<T: HeapSize> HeapSize for Vec<T> {
    fn heap_size_in_bytes(&self) -> usize {
        self.capacity() * std::mem::size_of::<T>()
            + self.iter().map(|x| x.heap_size_in_bytes()).sum::<usize>()
    }
}

impl<T: HeapSize> CompressionStep<T> {
    pub fn current_size_in_bytes(&self) -> usize {
        std::mem::size_of_val(self) + self.data.heap_size_in_bytes()
    }
}
