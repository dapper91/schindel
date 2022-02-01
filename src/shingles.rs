use std::cmp;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use std::marker::PhantomData;

pub use crate::hasher::{Murmur3Hasher, SeedHasher};

fn shift<T: Copy, const L: usize>(arr: &mut [T; L]) {
    for i in 0..L - 1 {
        let next = arr[i + 1];
        arr[i] = next;
    }
}

/// Min-shingle hash. The hash is configurable and can be parametrized by hash function, hash size etc.
///
/// # Generic parameters
/// * `H` - Hasher to be used to hash items from the input.
/// * `N` - Hash size. In min-shingle implementation the parameter sets the number of hash functions.
/// * `L` - N-Gram length. In min-shingle implementation the parameter sets the number contiguous items
///   from the input to be hashed together.
///
/// # Examples
/// ```
/// use schindel::shingles::{MinShingleHash, Murmur3Hasher};
///
/// let text = "text to be hashed...";
/// let original_hash = MinShingleHash::<Murmur3Hasher, 100, 5>::new(text.chars());
/// ```
pub struct MinShingleHash<H, const N: usize, const L: usize>
where
    H: SeedHasher,
{
    hash: [u32; N],

    seed_hasher: PhantomData<H>,
}

impl<H, const N: usize, const L: usize> MinShingleHash<H, N, L>
where
    H: SeedHasher,
{
    /// Creates a hash from the input.
    pub fn new<I>(input: I) -> Self
    where
        I: IntoIterator,
        I::Item: Hash + Copy + Default,
    {
        let mut hash = [u32::MAX; N];

        let mut input_iter = input.into_iter();
        let mut buf = [I::Item::default(); L];
        for i in 0..L - 1 {
            if let Some(item) = input_iter.next() {
                buf[i] = item;
            }
        }

        for item in input_iter {
            buf[L - 1] = item;
            for i in 0..N {
                let mut hasher = H::with_seed(i as u32);
                for item in buf.iter() {
                    item.hash(&mut hasher);
                }
                let shingle_hash = hasher.finish() as u32;
                hash[i] = cmp::min(hash[i], shingle_hash);
            }
            shift(&mut buf);
        }

        return MinShingleHash {
            hash,
            seed_hasher: PhantomData,
        };
    }

    /// Returns an iterator over hash values.
    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.hash.iter()
    }

    /// Returns a distance between two hashes. The distance is a hamming distance.
    pub fn compare(&self, other: &Self) -> f32 {
        let matches = self.iter().zip(other.iter()).filter(|pair| pair.0 == pair.1).count();

        return matches as f32 / N as f32;
    }
}

impl<H, const N: usize, const L: usize> IntoIterator for MinShingleHash<H, N, L>
where
    H: SeedHasher,
{
    type Item = u32;
    type IntoIter = <[Self::Item; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.hash.into_iter()
    }
}

impl<H, const N: usize, const L: usize> PartialEq for MinShingleHash<H, N, L>
where
    H: SeedHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl<H, const N: usize, const L: usize> Eq for MinShingleHash<H, N, L> where H: SeedHasher {}

impl<H, const N: usize, const L: usize> Debug for MinShingleHash<H, N, L>
where
    H: SeedHasher,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.hash.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::{MinShingleHash, Murmur3Hasher};

    #[test]
    fn test_shingle_hash() {
        let data = "hello world";

        let hash = MinShingleHash::<Murmur3Hasher, 10, 4>::new(data.chars());
        let actual_hash = Vec::from_iter(hash);
        let expected_hash = vec![
            507627377, 327820559, 366909560, 1875448240, 273434197, 278282553, 784375550, 314569335, 938527530, 4954098,
        ];

        assert_eq!(actual_hash, expected_hash);
    }
}
