use core::mem::size_of;

use crate::builder::SUFFIX;

#[inline(always)]
fn get_usize_len() -> usize {
    if cfg!(target_pointer_width = "64") { 64 } else if cfg!(target_pointer_width = "32") { 32 } else { panic!() }
}

/// bitmap only for bloom filter.
#[derive(Debug)]
#[derive(Clone)]
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


    pub fn clear(&mut self) {
        self.storage.fill(0);
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
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