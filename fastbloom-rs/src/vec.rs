use std::{fs::File, io::{self, Read, Seek}};

use crate::builder::SUFFIX;

#[inline(always)]
fn get_usize_len() -> usize {
    if cfg!(target_pointer_width = "64") { 64 } else if cfg!(target_pointer_width = "32") { 32 } else { panic!() }
}

/// bitmap only for bloom filter.
#[derive(Debug)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct BloomBitVec {
    /// Internal representation of the bit vector
    pub(crate) storage: Vec<usize>,
    /// The number of valid bits in the internal representation
    pub(crate) nbits: u64,
}

impl BloomBitVec {
    pub fn new(slots: usize) -> Self {
        BloomBitVec {
            storage: vec![0; slots],
            nbits: (slots * get_usize_len()) as u64,
        }
    }

    pub fn from_elem(slots: usize, bit: bool) -> Self {
        BloomBitVec {
            storage: vec![if bit { !0 } else { 0 }; slots],
            nbits: (slots * get_usize_len()) as u64,
        }
    }

    fn as_u8_slice(inp: &mut [usize]) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                inp.as_mut_ptr() as *mut u8,
                std::mem::size_of_val(inp)
            )
        }
    }
    
    pub fn from_file(file: &mut File, seek: u64, bytes_len: u64) -> io::Result<Self> {
        let length: u64 = bytes_len / TryInto::<u64>::try_into(std::mem::size_of::<usize>()).unwrap();

        let nbits = bytes_len * 8;

        let mut storage = vec![0usize; length.try_into().unwrap()];
        let buf = Self::as_u8_slice(&mut storage);
        file.seek(std::io::SeekFrom::Start(seek))?;
        file.read_exact(buf)?;

        Ok(BloomBitVec {
            storage,
            nbits: nbits.try_into().unwrap()
        })
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 6;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 5;
        let b = index & SUFFIX;
        let flag = 1usize << b;
        self.storage[w] = self.storage[w] | flag;
    }

    #[inline]
    pub fn get(&self, index: usize) -> bool {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 6;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 5;
        let b = index & SUFFIX;
        let flag = 1usize << b;
        (self.storage[w] & flag) != 0
    }

    pub fn or(&mut self, other: &BloomBitVec) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m |= *o;
        }
    }

    pub fn xor(&mut self, other: &BloomBitVec) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m ^= *o;
        }
    }

    pub fn nor(&mut self, other: &Self) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m = !(*m | *o);
        }
    }

    pub fn xnor(&mut self, other: &Self) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m = !(*m ^ *o);
        }
    }

    pub fn and(&mut self, other: &BloomBitVec) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m &= *o;
        }
    }

    pub fn nand(&mut self, other: &Self) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m = !(*m & *o);
        }
    }

    pub fn difference(&mut self, other: &Self) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m &= !*o;
        }
    }

    pub fn count_zeros(&self)->u32 {
        self.storage.iter().fold(0, |acc, x| acc + x.count_zeros())
    }

    pub fn clear(&mut self) {
        self.storage.fill(0);
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

/// counter vector for counting bloom filter.
#[derive(Debug)]
#[derive(Clone)]
pub(crate) struct CountingVec {
    /// Internal representation of the vector
    pub(crate) storage: Vec<usize>,
    /// The number of valid counter in the internal representation
    pub(crate) counters: u64,
    /// The number of valid counter in a slot which mean usize.
    pub(crate) counter_per_slot: usize,
}

impl CountingVec {
    /// create a CountingVec
    pub fn new(slots: usize) -> Self {
        let counter_per_slot = get_usize_len() >> 2;
        CountingVec {
            storage: vec![0; slots],
            counters: (slots * counter_per_slot) as u64,
            counter_per_slot,
        }
    }

    #[inline]
    pub fn increment(&mut self, index: usize) {
        let current = self.get(index);
        #[cfg(target_pointer_width = "64")]
        if current != 0b1111 {
            let current = current + 1;
            let w = index >> 4;
            let b = index & 0b1111;
            let move_bits = (15 - b) * 4;
            self.storage[w] =
                (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
        }

        #[cfg(target_pointer_width = "32")]
        if current != 0b111 {
            let current = current + 1;
            let w = index >> 3;
            let b = index & 0b111;
            let move_bits = (7 - b) * 4;
            self.storage[w] =
                (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
        }
    }

    #[inline]
    pub fn decrement(&mut self, index: usize) {
        let current = self.get(index);
        if current > 0 {
            if cfg!(target_pointer_width="64") {
                let current = current - 1;
                let w = index >> 4;
                let b = index & 0b1111;
                let move_bits = (15 - b) * 4;
                self.storage[w] =
                    (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
            } else if cfg!(target_pointer_width="32") {
                let current = current - 1;
                let w = index >> 3;
                let b = index & 0b111;
                let move_bits = (7 - b) * 4;
                self.storage[w] =
                    (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
            }
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> usize {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 4;
        #[cfg(target_pointer_width = "64")]
            let b = index & 0b1111;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 3;
        #[cfg(target_pointer_width = "32")]
            let b = index & 0b111;
        let slot = self.storage[w];
        #[cfg(target_pointer_width = "64")]
        return (slot >> ((15 - b) * 4)) & 0b1111;
        #[cfg(target_pointer_width = "32")]
        return (slot >> ((7 - b) * 4)) & 0b111;
    }

    pub fn clear(&mut self) {
        self.storage.fill(0);
    }
}

#[test]
fn test_vec() {
    let mut vec = BloomBitVec::new(16);
    vec.set(37);
    vec.set(38);
    println!("{:?}", vec);
    assert_eq!(vec.get(37), true);
    assert_eq!(vec.get(38), true);
}

#[test]
fn test_size() {
    println!("{}", get_usize_len());
    #[cfg(target_pointer_width = "64")]
    assert_eq!(get_usize_len(), 64);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(get_usize_len(), 32);
}

#[test]
fn test_count_vec() {
    let mut vec = CountingVec::new(10);
    vec.increment(7);

    assert_eq!(1, vec.get(7))
}

#[test]
fn test_count_zeros() {
    let mut vec = BloomBitVec::new(4);
    vec.set(37);
    vec.set(30);
    vec.set(38);
    println!("{:?}", vec);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(vec.count_zeros(), 253);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(vec.count_zeros(), 125);
}
