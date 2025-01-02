use bumpalo::{Bump, collections::Vec};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct StringArena<'a> {
    buffer: Vec<'a, u8>,              // Contiguous buffer for storing strings
    offsets: Vec<'a, (usize, usize)>, // Offsets for each string (start index, length)
}

impl<'a> StringArena<'a> {
    pub fn new(allocator: &'a Bump) -> Self {
        StringArena {
            buffer: Vec::new_in(allocator),
            offsets: Vec::new_in(allocator),
        }
    }

    /// Interns a string by storing it in the arena and returns an offset
    pub fn intern(&mut self, value: &str) -> usize {
        let start = self.buffer.len();
        self.buffer.extend_from_slice(value.as_bytes());
        let len = value.len();

        // Store the offset (start, length) for the string in the buffer
        self.offsets.push((start, len));

        // Return the index in the offsets vector (not the string itself)
        self.offsets.len() - 1 // Index of this string in the offsets vector
    }

    pub fn get_string(&self, index: usize) -> Option<&str> {
        if index < self.offsets.len() {
            let (start, len) = self.offsets[index];
            Some(unsafe { std::str::from_utf8_unchecked(&self.buffer[start..start + len]) })
        } else {
            None
        }
    }
}
