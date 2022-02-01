//! Rust min-shingle hashing implementation.
//! This crate implements simple min-shingle hashing algorithm.
//! For more information see [W-shingling](https://en.wikipedia.org/wiki/W-shingling).
//!
//! # Algorithm
//!
//! Shingle hash (or w-shingle) is a set of n-grams each of which composed of contiguous tokens within an input sequence
//! shifted by one element. For example, the document:
//!
//! `to be or not to be that is the question`
//!
//! has the following set of 2-grams (shingles):
//!
//! `(to, be)`, `(be, or)`, `(or, not)`, `(not, to)`, `(be, that)`, `(that, is)`, `(is, the)`, `(the, question)`
//!
//! *note*: 2-gram `(to, be)` occurs twice.
//!
//! The 2-gram set is a document shingle hash.
//! That hash can be used to measure two documents resemblance using Jaccard coefficient:
//!
//! `R(doc1, doc2) = (H(doc1) ⋂ H(doc2)) / (H(doc1) ⋃ H(doc2))`
//!
//! where:
//! - `R` - resemblance
//! - `H` - shingle hash
//!
//! The previous algorithm is not scalable to large documents because an n-gram set could grow very fast.
//! For example, if 3-grams is used and input sequence alphabet is 255 symbols then the set could be of size
//! `255 ^ 3` or `~16 * 10 ^ 6` in worst case which consumes a lot of memory.
//!
//! To resolve that problem min-shingle algorithm is used. It exploits special optimisation technic:
//! instead of storing all sequence n-grams n-gram hashes are calculated and a minimal hash value is saved.
//! Because the minimal value of a data stream can be calculated on the fly (without saving all the values),
//! memory consumption is drastically reduced. Repeating that process with several hash functions
//! (or several hash function seeds) shingle hash is produced.
//! As well as shingle hash min-shingle hash can be used to measure distance (or resemblance) between documents.
//!
//! # Examples:
//!
//! ```
//! use schindel::shingles::{MinShingleHash, Murmur3Hasher};
//!
//! fn main() {
//!     let original = "\
//!         “My sight is failing,” she said finally. “Even when I was young I could not have read what was written there. \
//!         But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be, \
//!         Benjamin?” For once Benjamin consented to break his rule, and he read out to her what was written on the wall. \
//!         There was nothing there now except a single Commandment. It ran:\
//!         ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";
//!
//!     let plagiarism = "\
//!         “My sight is failing,” she said finally. “When I was young I could not have read what was written there. \
//!         But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be” \
//!         Benjamin read out to her what was written. There was nothing there now except a single Commandment. \
//!         It ran: ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";
//!
//!     let other = "\
//!         Throughout the spring and summer they worked a sixty-hour week, and in August Napoleon announced that there \
//!         would be work on Sunday afternoons as well. This work was strictly voluntary, but any animal who absented \
//!         himself from it would have his rations reduced by half. Even so, it was found necessary to leave certain \
//!         tasks undone. The harvest was a little less successful than in the previous year, and two fields which \
//!         should have been sown with roots in the early summer were not sown because the ploughing had not been \
//!         completed early enough. It was possible to foresee that the coming winter would be a hard one.";
//!
//!     const HASH_LEN: usize = 100;
//!     const NGRAM_LEN: usize = 5;
//!
//!     let original_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(original.chars());
//!
//!     let plagiarism_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(plagiarism.chars());
//!     println!("plagiarism similarity: {}", original_hash.compare(&plagiarism_hash));
//!
//!     let other_hash = MinShingleHash::<Murmur3Hasher, HASH_LEN, NGRAM_LEN>::new(other.chars());
//!     println!("other text similarity: {}", original_hash.compare(&other_hash));
//! }
//! ```

pub mod hasher;
pub mod shingles;
