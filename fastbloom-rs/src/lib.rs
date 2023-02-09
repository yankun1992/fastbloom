extern crate core;

pub use bloom::{BloomFilter, CountingBloomFilter};
pub use builder::FilterBuilder;

mod builder;
mod bloom;
mod vec;
mod cuckoo;
mod sketch;

/// filter for check whether membership.
pub trait Membership {
    fn add(&mut self, element: &[u8]);

    fn contains(&self, element: &[u8]) -> bool;

    fn get_hash_indices(&self, element: &[u8]) -> Vec<u64>;

    fn contains_hash_indices(&self, indices: &Vec<u64>) -> bool;

    fn clear(&mut self);
}

pub trait Hashes {
    fn hashes(&self) -> u32;
}

/// filter which can remove element.
pub trait Deletable {
    /// remove element from this data structures.
    fn remove(&mut self, element: &[u8]);
}


