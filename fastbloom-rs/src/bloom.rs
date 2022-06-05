use std::clone;
use std::ptr::slice_from_raw_parts;

use fastmurmur3::murmur3_x64_128;

use crate::builder::FilterBuilder;
use crate::vec::BloomBitVec;

#[inline]
fn bit_set(bit_set: &mut BloomBitVec, value: &[u8], m: u128, k: u64) {
    // let len = m >> 5;
    let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    let hash2 = (murmur3_x64_128(value, 32) % m) as u64;

    let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        bit_set.set(mo);
    };
    bit_set.set(hash1 as usize);
}

#[inline]
fn bit_check(bit_set: &BloomBitVec, value: &[u8], m: u128, k: u64) -> bool {
    let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    let hash2 = (murmur3_x64_128(value, 32) % m) as u64;
    let mut res = bit_set.get(hash1 as usize);
    if !res { return false; }
    let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        res = res && bit_set.get(mo);
        if !res { return false; }
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

    ///  Returns the hash function number of the Bloom filter.
    pub fn hashes(&self) -> u32 {
        self.config.hashes
    }

    /// Adds the passed value to the filter.
    pub fn add(&mut self, element: &[u8]) {
        bit_set(&mut self.bit_set, element, self.config.size as u128,
                self.config.hashes as u64);
    }

    /// Removes all elements from the filter (i.e. resets all bits to zero).
    pub fn clear(&mut self) {
        self.bit_set.clear();
    }

    /// Tests whether an element is present in the filter (subject to the specified false
    /// positive rate).
    #[inline]
    pub fn contains(&self, element: &[u8]) -> bool {
        bit_check(&self.bit_set, element, self.config.size as u128,
                  self.config.hashes as u64)
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
            let ptr = slice_from_raw_parts(u8_ptr, storage.len() * 2);
        unsafe { &*ptr }
    }

    /// Return the underlying u32 vector of the Bloom filter.
    pub fn get_u32_array(&self) -> &[u32] {
        let storage = &self.bit_set.storage;
        let ptr = storage.as_ptr() as *const u32;
        #[cfg(target_pointer_width = "64")]
            let ptr = slice_from_raw_parts(ptr, storage.len() * 2);
        #[cfg(target_pointer_width = "32")]
            let ptr = slice_from_raw_parts(u8_ptr, storage.len());
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
            let ptr = slice_from_raw_parts(u8_ptr, storage.len() / 2usize);

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


#[test]
fn bloom_test() {
    let mut builder =
        FilterBuilder::new(10_000, 0.01);
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