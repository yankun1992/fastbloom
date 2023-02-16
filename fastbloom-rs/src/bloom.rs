use std::clone;
use std::cmp::min;
use std::ptr::slice_from_raw_parts;

use fastmurmur3::murmur3_x64_128;
use xxhash_rust::xxh3::xxh3_64_with_seed;

use crate::{Deletable, Hashes, Membership};
use crate::builder::FilterBuilder;
use crate::vec::{BloomBitVec, CountingVec};

#[inline]
fn bit_set(bit_set: &mut BloomBitVec, value: &[u8], m: u64, k: u64) {
    // let len = m >> 5;
    // let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    // let hash2 = (murmur3_x64_128(value, 32) % m) as u64;
    let hash1 = xxh3_64_with_seed(value, 0) % m;
    let hash2 = xxh3_64_with_seed(value, 32) % m;

    let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        bit_set.set(mo);
    };
    bit_set.set(hash1 as usize);
}

fn bit_set_cache_friendly(bit_set: &mut BloomBitVec, value: &[u8], m: u64, k: u64) {
    let hash1 = xxh3_64_with_seed(value, 0) % m;
    bit_set.set(hash1 as usize);
    for i in 1..k {
        let hash = xxh3_64_with_seed(value, 32 * i) % 64;
        let mo = ((hash1 + hash) % m) as usize;
        bit_set.set(mo);
    };
}

#[inline]
fn bit_check(bit_set: &BloomBitVec, value: &[u8], m: u64, k: u64) -> bool {
    // let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    // let hash2 = (murmur3_x64_128(value, 32) % m) as u64;
    let hash1 = xxh3_64_with_seed(value, 0) % m;
    let hash2 = xxh3_64_with_seed(value, 32) % m;
    let mut res = bit_set.get(hash1 as usize);
    if !res { return false; }
    // let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        res = res && bit_set.get(mo);
        if !res { return false; }
    }
    res
}

#[inline]
fn get_bit_indices(bit_set: &BloomBitVec, value: &[u8], m: u64, k: u64) -> Vec<u64> {
    let mut res = Vec::<u64>::with_capacity(k as usize);
    // let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    // let hash2 = (murmur3_x64_128(value, 32) % m) as u64;
    let hash1 = xxh3_64_with_seed(value, 0) % m;
    let hash2 = xxh3_64_with_seed(value, 32) % m;
    res.push(hash1);
    // let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        res.push(mo as u64);
    }
    res
}

/// A Bloom filter is a space-efficient probabilistic data structure, conceived by Burton Howard
/// Bloom in 1970, that is used to test whether an element is a member of a set. False positive
/// matches are possible, but false negatives are not.
///
/// **Reference**: Bloom, B. H. (1970). Space/time trade-offs in hash coding with allowable errors.
/// Communications of the ACM, 13(7), 422-426.
/// [Full text article](http://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf)
#[derive(Clone)]
#[derive(Debug)]
pub struct BloomFilter {
    config: FilterBuilder,
    bit_set: BloomBitVec,
}

impl Membership for BloomFilter {
    /// Adds the passed value to the filter.
    fn add(&mut self, element: &[u8]) {
        bit_set(&mut self.bit_set, element, self.config.size,
                self.config.hashes as u64);
    }

    /// Tests whether an element is present in the filter (subject to the specified false
    /// positive rate).
    #[inline]
    fn contains(&self, element: &[u8]) -> bool {
        bit_check(&self.bit_set, element, self.config.size,
                  self.config.hashes as u64)
    }

    /// Get the hashes indices of the element in the filter.
    fn get_hash_indices(&self, element: &[u8]) -> Vec<u64> {
        get_bit_indices(&self.bit_set, element, self.config.size,
                        self.config.hashes as u64)
    }

    /// Tests whether a hashes indices is present in the filter
    fn contains_hash_indices(&self, indices: &Vec<u64>) -> bool {
        for x in indices.iter() {
            let index = *x;
            if !self.bit_set.get(index as usize) { return false; }
        }
        true
    }

    /// Removes all elements from the filter (i.e. resets all bits to zero).
    fn clear(&mut self) {
        self.bit_set.clear();
    }
}

impl Hashes for BloomFilter {
    ///  Returns the hash function number of the Bloom filter.
    fn hashes(&self) -> u32 {
        self.config.hashes
    }
}

impl BloomFilter {
    /// Build a Bloom filter form [FilterBuilder].
    ///
    /// # Examples:
    ///
    /// ```rust
    /// use fastbloom_rs::{BloomFilter, FilterBuilder};
    ///
    /// let builder = FilterBuilder::new(100_000_000, 0.01);
    /// let bloom = BloomFilter::new(builder);
    /// ```
    pub fn new(mut config: FilterBuilder) -> Self {
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let bit_set = BloomBitVec::new((config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let bit_set = BloomBitVec::new((config.size >> 5) as usize);
        BloomFilter { config, bit_set }
    }

    /// Build a Bloom filter form `&[u8]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::BloomFilter;
    /// let mut array = vec![0u8; 4096];
    /// let bloom = BloomFilter::from_u8_array(array.as_bytes(), 4);
    /// ```
    pub fn from_u8_array(array: &[u8], hashes: u32) -> Self {
        let mut config =
            FilterBuilder::from_size_and_hashes((array.len() * 8) as u64, hashes);
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let mut bit_vec = BloomBitVec::new((config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let mut bit_vec = BloomBitVec::new((config.size >> 5) as usize);

        let ptr = array.as_ptr() as *const usize;
        #[cfg(target_pointer_width = "64")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 5) as usize);

        bit_vec.storage.copy_from_slice(unsafe { &*usize_array });

        BloomFilter { config, bit_set: bit_vec }
    }

    /// Build a Bloom filter form `&[u16]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::BloomFilter;
    /// let mut array = vec![0u16; 2048];
    /// let bloom = BloomFilter::from_u16_array(array.as_bytes(), 4);
    /// ```
    pub fn from_u16_array(array: &[u16], hashes: u32) -> Self {
        let mut config =
            FilterBuilder::from_size_and_hashes((array.len() * 16) as u64, hashes);
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let mut bit_vec = BloomBitVec::new((config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let mut bit_vec = BloomBitVec::new((config.size >> 5) as usize);

        let ptr = array.as_ptr() as *const usize;
        #[cfg(target_pointer_width = "64")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 5) as usize);

        bit_vec.storage.copy_from_slice(unsafe { &*usize_array });

        BloomFilter { config, bit_set: bit_vec }
    }


    /// Build a Bloom filter form `&[u32]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::BloomFilter;
    /// let mut array = vec![0u32; 1024];
    /// let bloom = BloomFilter::from_u32_array(array.as_bytes(), 4);
    /// ```
    pub fn from_u32_array(array: &[u32], hashes: u32) -> Self {
        let mut config =
            FilterBuilder::from_size_and_hashes((array.len() * 32) as u64, hashes);
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let mut bit_vec = BloomBitVec::new((config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let mut bit_vec = BloomBitVec::new((config.size >> 5) as usize);

        let ptr = array.as_ptr() as *const usize;
        #[cfg(target_pointer_width = "64")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 5) as usize);

        bit_vec.storage.copy_from_slice(unsafe { &*usize_array });

        BloomFilter { config, bit_set: bit_vec }
    }

    /// Build a Bloom filter form `&[u64]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::BloomFilter;
    /// let mut array = vec![0u64; 512];
    /// let bloom = BloomFilter::from_u32_array(array.as_bytes(), 4);
    /// ```
    pub fn from_u64_array(array: &[u64], hashes: u32) -> Self {
        let mut config =
            FilterBuilder::from_size_and_hashes((array.len() * 64) as u64, hashes);
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let mut bit_vec = BloomBitVec::new((config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let mut bit_vec = BloomBitVec::new((config.size >> 5) as usize);

        let ptr = array.as_ptr() as *const usize;
        #[cfg(target_pointer_width = "64")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 6) as usize);
        #[cfg(target_pointer_width = "32")]
            let usize_array = slice_from_raw_parts(ptr, (config.size >> 5) as usize);

        bit_vec.storage.copy_from_slice(unsafe { &*usize_array });

        BloomFilter { config, bit_set: bit_vec }
    }

    /// Returns the configuration/builder of the Bloom filter.
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::{BloomFilter, FilterBuilder};
    ///
    /// let bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();
    /// let builder = bloom.config();
    /// ```
    ///
    pub fn config(&self) -> FilterBuilder {
        self.config.clone()
    }

    /// Return the underlying byte vector of the Bloom filter.
    pub fn get_u8_array(&self) -> &[u8] {
        let storage = &self.bit_set.storage;
        let ptr = storage.as_ptr();
        let u8_ptr = ptr as *const u8;
        #[cfg(target_pointer_width = "64")]
            let ptr = slice_from_raw_parts(u8_ptr, storage.len() * 8);
        #[cfg(target_pointer_width = "32")]
            let ptr = slice_from_raw_parts(u8_ptr, storage.len() * 4);
        unsafe { &*ptr }
    }

    /// Return the underlying u16 vector of the Bloom filter.
    pub fn get_u16_array(&self) -> &[u16] {
        let storage = &self.bit_set.storage;
        let ptr = storage.as_ptr() as *const u16;
        #[cfg(target_pointer_width = "64")]
            let ptr = slice_from_raw_parts(ptr, storage.len() * 4);
        #[cfg(target_pointer_width = "32")]
            let ptr = slice_from_raw_parts(ptr, storage.len() * 2);
        unsafe { &*ptr }
    }

    /// Return the underlying u32 vector of the Bloom filter.
    pub fn get_u32_array(&self) -> &[u32] {
        let storage = &self.bit_set.storage;
        let ptr = storage.as_ptr() as *const u32;
        #[cfg(target_pointer_width = "64")]
            let ptr = slice_from_raw_parts(ptr, storage.len() * 2);
        #[cfg(target_pointer_width = "32")]
            let ptr = slice_from_raw_parts(ptr, storage.len());
        unsafe { &*ptr }
    }

    /// Return the underlying u64 vector of the Bloom filter.
    pub fn get_u64_array(&self) -> &[u64] {
        let storage = &self.bit_set.storage;
        let ptr = storage.as_ptr() as *const u64;
        #[cfg(target_pointer_width = "64")]
            let ptr = slice_from_raw_parts(ptr, storage.len());
        if cfg!(target_pointer_width= "32") {
            if storage.len() % 2 != 0 {
                panic!("BloomBitVec with len {} can't export as u64 array!", storage.len())
            }
        }
        #[cfg(target_pointer_width = "32")]
            let ptr = slice_from_raw_parts(ptr, storage.len() / 2usize);

        unsafe { &*ptr }
    }


    /// Performs the union operation on two compatible bloom filters. This is achieved through a
    /// bitwise OR operation on their bit vectors. This operations is lossless, i.e. no elements
    /// are lost and the bloom filter is the same that would have resulted if all elements wer
    /// directly inserted in just one bloom filter.
    pub fn union(&mut self, other: &BloomFilter) -> bool {
        if self.compatible(other) {
            self.bit_set.or(&other.bit_set);
            true
        } else { false }
    }

    /// Performs the intersection operation on two compatible bloom filters. This is achieved
    /// through a bitwise AND operation on their bit vectors. The operations doesn't introduce
    /// any false negatives but it does raise the false positive probability. The the false
    /// positive probability in the resulting Bloom filter is at most the false-positive probability
    /// in one of the constituent bloom filters
    pub fn intersect(&mut self, other: &BloomFilter) -> bool {
        if self.compatible(other) {
            self.bit_set.and(&other.bit_set);
            true
        } else { false }
    }

    /// Returns [true] if the Bloom filter does not contain any elements
    pub fn is_empty(&self) -> bool {
        self.bit_set.is_empty()
    }

    pub(crate) fn set_bit_vec(&mut self, bit_vec: BloomBitVec) {
        assert_eq!(self.config.size, bit_vec.nbits as u64);
        self.bit_set = bit_vec
    }

    /// Checks if two Bloom filters are compatible, i.e. have compatible parameters (hash function,
    /// size, etc.)
    fn compatible(&self, other: &BloomFilter) -> bool {
        self.config.is_compatible_to(&other.config)
    }
}

/// A Counting Bloom filter works in a similar manner as a regular Bloom filter; however, it is
/// able to keep track of insertions and deletions. In a counting Bloom filter, each entry in the
/// Bloom filter is a small counter associated with a basic Bloom filter bit.
///
/// **Reference**: F. Bonomi, M. Mitzenmacher, R. Panigrahy, S. Singh, and G. Varghese, “An Improved
/// Construction for Counting Bloom Filters,” in 14th Annual European Symposium on
/// Algorithms, LNCS 4168, 2006
#[derive(Clone)]
#[derive(Debug)]
pub struct CountingBloomFilter {
    config: FilterBuilder,
    counting_vec: CountingVec,
}

macro_rules! get_array {
    ($name:ident, $native:ty, $len:expr) => {
        impl CountingBloomFilter {
            pub fn $name(&self) -> &[$native] {
                let ptr = self.counting_vec.storage.as_ptr() as *const $native;
                #[cfg(target_pointer_width = "64")]
                    let arr = slice_from_raw_parts(ptr, self.counting_vec.storage.len() * $len);
                #[cfg(target_pointer_width = "32")]
                    let arr = slice_from_raw_parts(ptr, self.counting_vec.storage.len() * $len / 2);
                unsafe { &*arr }
            }
        }
    };
}

get_array!(get_u8_array, u8, 8);
get_array!(get_u16_array, u16, 4);
get_array!(get_u32_array, u32, 2);
get_array!(get_u64_array, u64, 1);

impl CountingBloomFilter {
    pub fn new(mut config: FilterBuilder) -> Self {
        config.complete();
        #[cfg(target_pointer_width = "64")]
            let counting_vec = CountingVec::new((config.size >> 4) as usize);
        #[cfg(target_pointer_width = "32")]
            let counting_vec = CountingVec::new((config.size >> 3) as usize);
        CountingBloomFilter { config, counting_vec }
    }

    pub(crate) fn set_counting_vec(&mut self, counting_vec: CountingVec) {
        assert_eq!(self.config.size, counting_vec.counters as u64);
        self.counting_vec = counting_vec
    }

    /// Checks if two Counting Bloom filters are compatible, i.e. have compatible parameters (hash
    /// function, size, etc.)
    fn compatible(&self, other: &BloomFilter) -> bool {
        self.config.is_compatible_to(&other.config)
    }

    /// Returns the configuration/builder of the Bloom filter.
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::{BloomFilter, FilterBuilder};
    ///
    /// let bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();
    /// let builder = bloom.config();
    /// ```
    ///
    pub fn config(&self) -> FilterBuilder {
        self.config.clone()
    }
}

macro_rules! from_array {
    ($name:ident, $native:ty, $num:expr) => {
        impl CountingBloomFilter {
            pub fn $name(array: &[$native], hashes: u32, enable_repeat_insert:bool) -> Self {
                let mut config =
                    FilterBuilder::from_size_and_hashes((array.len() * $num) as u64, hashes);
                config.enable_repeat_insert(enable_repeat_insert);
                config.complete();
                #[cfg(target_pointer_width = "64")]
                    let mut counting_vec = CountingVec::new((config.size >> 4) as usize);
                #[cfg(target_pointer_width = "32")]
                    let mut counting_vec = CountingVec::new((config.size >> 3) as usize);

                let ptr = array.as_ptr() as *const usize;
                #[cfg(target_pointer_width = "64")]
                    let usize_array = slice_from_raw_parts(ptr, (config.size >> 4) as usize);
                #[cfg(target_pointer_width = "32")]
                    let usize_array = slice_from_raw_parts(ptr, (config.size >> 3) as usize);

                counting_vec.storage.copy_from_slice(unsafe { &*usize_array });

                CountingBloomFilter { config, counting_vec }
            }
        }
    };
}

from_array!(from_u8_array, u8, 2);
from_array!(from_u16_array, u16, 4);
from_array!(from_u32_array, u32, 8);
from_array!(from_u64_array, u64, 16);

impl CountingBloomFilter {
    /// Get the estimate count for element in this counting bloom filter.
    /// See: https://github.com/yankun1992/fastbloom/issues/3
    pub fn estimate_count(&self, element: &[u8]) -> usize {
        let m = self.config.size;
        let hash1 = xxh3_64_with_seed(element, 0) % m;
        let hash2 = xxh3_64_with_seed(element, 32) % m;

        let mut res = self.counting_vec.get(hash1 as usize);
        if res == 0 { return 0; }

        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            let count = self.counting_vec.get(mo);
            if count == 0 { return 0; } else { res = min(count, res) }
        }

        res
    }

    /// Get the underlying counter at index.
    pub fn counter_at(&self, index: u64) -> usize {
        self.counting_vec.get(index as usize)
    }
}

impl Membership for CountingBloomFilter {
    fn add(&mut self, element: &[u8]) {
        let m = self.config.size;
        // let hash1 = (murmur3_x64_128(element, 0) % m) as u64;
        // let hash2 = (murmur3_x64_128(element, 32) % m) as u64;
        let hash1 = xxh3_64_with_seed(element, 0) % m;
        let hash2 = xxh3_64_with_seed(element, 32) % m;

        let mut res = self.counting_vec.get(hash1 as usize) > 0;
        // let m = self.config.size;
        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            res = res && (self.counting_vec.get(mo) > 0);
        }

        // contains and not enable repeat insert
        if res && !self.config.enable_repeat_insert {
            return;
        }

        // insert
        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            self.counting_vec.increment(mo);
        };
        self.counting_vec.increment(hash1 as usize);
    }

    #[inline]
    fn contains(&self, element: &[u8]) -> bool {
        let m = self.config.size;
        // let hash1 = (murmur3_x64_128(element, 0) % m) as u64;
        // let hash2 = (murmur3_x64_128(element, 32) % m) as u64;
        let hash1 = xxh3_64_with_seed(element, 0) % m;
        let hash2 = xxh3_64_with_seed(element, 32) % m;

        let mut res = self.counting_vec.get(hash1 as usize) > 0;
        if !res { return false; }
        // let m = self.config.size;
        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            res = res && (self.counting_vec.get(mo) > 0);
            if !res { return false; }
        }
        res
    }

    fn get_hash_indices(&self, element: &[u8]) -> Vec<u64> {
        let m = self.config.size;
        let mut res = Vec::<u64>::with_capacity(self.config.size as usize);
        // let hash1 = (murmur3_x64_128(element, 0) % m) as u64;
        // let hash2 = (murmur3_x64_128(element, 32) % m) as u64;
        let hash1 = xxh3_64_with_seed(element, 0) % m;
        let hash2 = xxh3_64_with_seed(element, 32) % m;
        res.push(hash1);
        // let m = self.config.size;
        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            res.push(mo as u64);
        }
        res
    }

    fn contains_hash_indices(&self, indices: &Vec<u64>) -> bool {
        for x in indices.iter() {
            let index = *x;
            if self.counting_vec.get(index as usize) == 0 { return false; }
        }
        true
    }

    fn clear(&mut self) {
        self.counting_vec.clear()
    }
}

impl Deletable for CountingBloomFilter {
    fn remove(&mut self, element: &[u8]) {
        let m = self.config.size;
        // let hash1 = (murmur3_x64_128(element, 0) % m) as u64;
        // let hash2 = (murmur3_x64_128(element, 32) % m) as u64;
        let hash1 = xxh3_64_with_seed(element, 0) % m;
        let hash2 = xxh3_64_with_seed(element, 32) % m;

        let mut res = self.counting_vec.get(hash1 as usize) > 0;
        // let m = self.config.size;
        for i in 1..self.config.hashes as u64 {
            let mo = ((hash1 + i * hash2) % m) as usize;
            res = res && (self.counting_vec.get(mo) > 0);
        }

        // contains
        if res {
            for i in 1..self.config.hashes as u64 {
                let mo = ((hash1 + i * hash2) % m) as usize;
                self.counting_vec.decrement(mo);
            };
            self.counting_vec.decrement(hash1 as usize);
        }
    }
}

impl Hashes for CountingBloomFilter {
    fn hashes(&self) -> u32 {
        self.config.hashes
    }
}

/// A Partitioned Bloom Filter is a variation of a classic Bloom Filter.
///
/// This filter works by partitioning the M-sized bit array into k slices of size `m = M/k` bits,
/// `k = nb of hash functions` in the filter. Each hash function produces an index over `m` for its
/// respective slice. Thus, each element is described by exactly `k` bits, meaning the distribution
/// of false positives is uniform across all elements.
///
/// Be careful, as a Partitioned Bloom Filter have much higher collison risks that a classic
/// Bloom Filter on small sets of data.
///
/// **Reference**: Chang, F., Feng, W. C., & Li, K. (2004, March). Approximate caches for packet
/// classification. In INFOCOM 2004. Twenty-third AnnualJoint Conference of the IEEE Computer and
/// Communications Societies (Vol. 4, pp. 2196-2207). IEEE.
/// [Full text article](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.153.6902&rep=rep1&type=pdf)
#[derive(Clone)]
#[derive(Debug)]
pub(crate) struct PartitionedBloomFilter {}

impl PartitionedBloomFilter {}

/// A Scalable Bloom Filter is a variant of Bloom Filters that can adapt dynamically to the number
/// of elements stored, while assuring a maximum false positive probability.
///
/// **Reference**: ALMEIDA, Paulo Sérgio, BAQUERO, Carlos, PREGUIÇA, Nuno, et al. Scalable bloom
/// filters. Information Processing Letters, 2007, vol. 101, no 6, p. 255-261.
/// [Full text article](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.725.390&rep=rep1&type=pdf)
#[derive(Clone)]
#[derive(Debug)]
pub(crate) struct ScalableBloomFilter {}

impl ScalableBloomFilter {}

/// An Invertible Bloom Filters (IBLT), also called Invertible Bloom Lookup Table, is a
/// space-efficient and probabilistic data-structure for solving the set-difference problem
/// efficiently without the use of logs or other prior context. It computes the set difference
/// with communication proportional to the size of the difference between the sets being compared.
/// They can simultaneously calculate D(A−B) and D(B−A) using O(d) space. This data structure
/// encodes sets in a fashion that is similar in spirit to Tornado codes’ construction, in that it
/// randomly combines elements using the XOR function.
///
/// **Reference**: Eppstein, D., Goodrich, M. T., Uyeda, F., & Varghese, G. (2011). What's the
/// difference?: efficient set reconciliation without prior context. ACM SIGCOMM Computer
/// Communication Review, 41(4), 218-229.
/// [Full text article](http://www.sysnet.ucsd.edu/sysnet/miscpapers/EppGooUye-SIGCOMM-11.pdf)
#[derive(Clone)]
#[derive(Debug)]
pub(crate) struct InvertibleBloomFilter {}

impl InvertibleBloomFilter {}

#[derive(Clone)]
#[derive(Debug)]
pub(crate) struct GarbledBloomFilter {}

impl GarbledBloomFilter {}


#[test]
fn bloom_test() {
    let mut builder =
        FilterBuilder::new(10_000_000, 0.01);
    let mut bloom = builder.build_bloom_filter();
    println!("{:?}", bloom.config);
    bloom.add(b"hello");
    println!("{:?}", &bloom.bit_set.storage[0..300]);
    assert_eq!(bloom.contains(b"hello"), true);
    assert_eq!(bloom.contains(b"world"), false);

    let storage = &bloom.bit_set.storage[0..300];
    println!("{:?}", storage);

    let mut bloom2 = BloomFilter::from_u64_array(bloom.get_u64_array(), bloom.hashes());
    assert_eq!(bloom2.compatible(&bloom), true);
    assert_eq!(bloom2.contains(b"hello"), true);
    assert_eq!(bloom2.contains(b"world"), false);

    let mut bloom3 =
        BloomFilter::from_u32_array(bloom.get_u32_array(), bloom.config.hashes);
    assert_eq!(bloom3.compatible(&bloom), true);
    assert_eq!(bloom3.contains(b"hello"), true);
    assert_eq!(bloom3.contains(b"world"), false);

    let u8_array = bloom.get_u8_array();
    let mut bloom4 = BloomFilter::from_u8_array(u8_array, bloom.config.hashes);
    println!("{:?}", &bloom4.bit_set.storage[0..300]);
    assert_eq!(bloom4.compatible(&bloom), true);
    assert_eq!(bloom4.contains(b"hello"), true);
    assert_eq!(bloom4.contains(b"world"), false);

    let bloom5 = BloomFilter::from_u16_array(bloom.get_u16_array(), bloom.hashes());
    assert_eq!(bloom5.compatible(&bloom), true);
    assert_eq!(bloom5.contains(b"hello"), true);
    assert_eq!(bloom5.contains(b"world"), false);

    bloom4.add(b"hello world");

    assert_eq!(bloom.intersect(&bloom4), true);
    assert_eq!(bloom.contains(b"hello"), true);
    assert_eq!(bloom.contains(b"hello world"), false);

    bloom3.add(b"hello world");
    bloom3.add(b"hello yankun");

    assert_eq!(bloom3.union(&bloom4), true);
    assert_eq!(bloom3.contains(b"hello"), true);
    assert_eq!(bloom3.contains(b"hello world"), true);
    assert_eq!(bloom3.contains(b"hello yankun"), true);
}

#[test]
fn bloom_hash_indices_test() {
    let mut builder =
        FilterBuilder::new(10_000, 0.01);
    let mut bloom = builder.build_bloom_filter();
    println!("{:?}", bloom.config);
    bloom.add(b"hello");
    assert_eq!(bloom.contains(b"hello"), true);
    assert_eq!(bloom.contains(b"world"), false);

    let indices = bloom.get_hash_indices(b"hello");
    println!("{:?}", indices);
    assert_eq!(bloom.contains_hash_indices(&indices), true);
    assert_eq!(bloom.contains_hash_indices(&bloom.get_hash_indices(b"world")), false);
}


#[test]
fn counting_bloom_test() {
    let mut builder =
        FilterBuilder::new(10_000, 0.01);
    let mut bloom = builder.build_counting_bloom_filter();

    bloom.add(b"hello");

    assert_eq!(bloom.contains(b"hello"), true);

    bloom.remove(b"hello");
    assert_eq!(bloom.contains(b"hello"), false);
}

#[test]
fn counting_bloom_repeat_test() {
    let mut builder = FilterBuilder::new(100_000, 0.01);
    // enable_repeat_insert is true
    builder.enable_repeat_insert(true);
    let mut cbf = builder.build_counting_bloom_filter();
    cbf.add(b"hello"); // modify underlying vector counter.
    cbf.add(b"hello"); // modify underlying vector counter.
    assert_eq!(cbf.contains(b"hello"), true);
    cbf.remove(b"hello");
    assert_eq!(cbf.contains(b"hello"), true);
    cbf.remove(b"hello");
    assert_eq!(cbf.contains(b"hello"), false);

    // enable_repeat_insert is false
    builder.enable_repeat_insert(false);
    let mut cbf = builder.build_counting_bloom_filter();
    cbf.add(b"hello"); // modify underlying vector counter.
    cbf.add(b"hello"); // not modify underlying vector counter because b"hello" has been added.
    assert_eq!(cbf.contains(b"hello"), true);
    cbf.remove(b"hello");
    assert_eq!(cbf.contains(b"hello"), false);
}

#[test]
fn counting_bloom_from_test() {
    let mut builder = FilterBuilder::new(100_000, 0.01);
    let mut cbf = builder.build_counting_bloom_filter();

    cbf.add(b"hello");
    cbf.add(b"hello");

    let mut cbf_copy = CountingBloomFilter::from_u8_array(cbf.get_u8_array(), builder.hashes, true);
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), false);

    let mut cbf_copy = CountingBloomFilter::from_u16_array(cbf.get_u16_array(), builder.hashes, true);
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), false);

    let mut cbf_copy = CountingBloomFilter::from_u32_array(cbf.get_u32_array(), builder.hashes, true);
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), false);

    let mut cbf_copy = CountingBloomFilter::from_u64_array(cbf.get_u64_array(), builder.hashes, true);
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), true);
    cbf_copy.remove(b"hello");
    assert_eq!(cbf_copy.contains(b"hello"), false);
}

#[test]
fn counting_bloom_hash_indices_test() {
    let mut builder =
        FilterBuilder::new(10_000, 0.01);
    let mut bloom = builder.build_counting_bloom_filter();

    bloom.add(b"hello");

    assert_eq!(bloom.contains(b"hello"), true);
    assert_eq!(bloom.contains_hash_indices(&bloom.get_hash_indices(b"hello")), true);
    assert_eq!(bloom.contains_hash_indices(&bloom.get_hash_indices(b"world")), false);


    bloom.remove(b"hello");
    assert_eq!(bloom.contains(b"hello"), false);
    assert_eq!(bloom.contains_hash_indices(&bloom.get_hash_indices(b"hello")), false);
}

#[test]
fn counting_bloom_estimate_count() {
    let mut builder =
        FilterBuilder::new(10_000, 0.01);
    let mut bloom = builder.build_counting_bloom_filter();

    bloom.add(b"hello");
    bloom.add(b"world");

    assert_eq!(bloom.estimate_count(b"hello"), 1);
    let indices = bloom.get_hash_indices(b"hello");

    for index in indices {
        assert_eq!(bloom.counter_at(index), 1)
    }

    assert_eq!(bloom.estimate_count(b"world"), 1);
    for index in bloom.get_hash_indices(b"world") {
        assert!(bloom.counter_at(index) <= 2);
    }
}
