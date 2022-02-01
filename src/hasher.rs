//! Shingle hasher implementation.

use std::hash::Hasher;

use murmurhash3::murmurhash3_x86_32;

/// A trait for hasher builder with custom seed value.
pub trait SeedHasher {
    type HasherType: Hasher;

    /// Creates a hasher with provided seed value.
    fn with_seed(seed: u32) -> Self::HasherType;
}
pub struct Murmur3Hasher {
    seed: u32,
    bytes: Vec<u8>,
}

impl Hasher for Murmur3Hasher {
    fn finish(&self) -> u64 {
        return murmurhash3_x86_32(&self.bytes, self.seed) as u64;
    }

    fn write(&mut self, bytes: &[u8]) {
        self.bytes.extend(bytes);
    }
}

impl SeedHasher for Murmur3Hasher {
    type HasherType = Murmur3Hasher;

    fn with_seed(seed: u32) -> Self::HasherType {
        return Murmur3Hasher { seed, bytes: vec![] };
    }
}
