use std::hash::{Hash, Hasher};

use crc32fast::Hasher as CRCHasher;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use fastmurmur3::murmur3_x64_128;
use fxhash::{FxHasher64, hash64};
use getrandom::getrandom;
use siphasher::sip::SipHasher13;

use fastbloom_rs::{BloomFilter, FilterBuilder};
use fastbloom_rs::Bloom;

#[inline]
fn sip_new(key: &[u8; 16]) -> SipHasher13 {
    SipHasher13::new_with_key(key)
}

fn bloom_hash<T>(hashes: &mut [u64; 2], item: &T, k_i: u32, sips: &mut [SipHasher13; 2]) -> u64
    where
        T: Hash,
{
    if k_i < 2 {
        let mut sip = &mut sips[k_i as usize].clone();
        item.hash(sip);
        let hash = sip.finish();
        hashes[k_i as usize] = hash;
        hash
    } else {
        (hashes[0] as u128).wrapping_add((k_i as u128).wrapping_mul(hashes[1] as u128)) as u64
            % 0xffffffffffffffc5
    }
}

fn bloom_test(bloom: &mut Bloom<String>, inputs: &[String]) {
    for input in inputs {
        bloom.set(input);
    }
}

fn filter_test(filter: &mut BloomFilter, inputs: &[String]) {
    for input in inputs {
        filter.add(input.as_bytes());
    }
}

fn filter_add_test(filter: &mut BloomFilter, data: &[u8]) {
    filter.add(data);
}

fn as_test(m: u128) -> u64 {
    m as u64
}

fn criterion_benchmark(c: &mut Criterion) {
    let items_count = 100_000_000;
    let mut seed = [0u8; 32];

    getrandom(&mut seed).unwrap();
    let mut bloom: Bloom<String> = Bloom::new_with_seed(Bloom::<String>::compute_bitmap_size(items_count, 0.01), items_count, &seed);
    let inputs: Vec<String> = (1..1_000_000).map(|n| { n.to_string() }).collect();

    let mut hashes = [0u64; 2];

    let mut k1 = [0u8; 16];
    let mut k2 = [0u8; 16];
    k1.copy_from_slice(&seed[0..16]);
    k2.copy_from_slice(&seed[16..32]);
    let mut sips: [SipHasher13; 2] = [sip_new(&k1), sip_new(&k2)];

    let hello = "hellohellohellohello".to_string();

    let mut filter = FilterBuilder::new(items_count as u64, 0.01)
        .build_bloom_filter();

    let mut fxhash = FxHasher64::default();

    let mut crc = CRCHasher::new();


    // c.bench_function("mod_u128", |b| b.iter(|| black_box(43567890u128) % black_box(1024u128)));
    // c.bench_function("mod_u64", |b| b.iter(|| black_box(43567890u64) % black_box(1024u64)));
    // c.bench_function("add_u64", |b| b.iter(|| black_box(43567890u64) + black_box(1024u64)));
    // c.bench_function("crc32fast", |b| b.iter(|| {
    //     crc.update(black_box(hello.as_bytes()));
    //     crc.finish();
    // }));
    // c.bench_function("bloom_hash", |b| b.iter(|| bloom_hash(&mut hashes, &black_box("hellohellohellohello"), black_box(1), &mut sips)));
    c.bench_function("fastmurmur3", |b| b.iter(|| murmur3_x64_128(black_box(b"hellohellohellohello"), 0)));
    c.bench_function("filter_add_test", |b| b.iter(|| filter_add_test(&mut filter, black_box("hellohellohellohello".as_bytes()))));
    c.bench_function("bloom_add_test", |b| b.iter(|| bloom.set(black_box(&hello))));
    // c.bench_function("fxhash", |b| b.iter(|| {
    //     hello.hash(&mut fxhash);
    //     fxhash.finish();
    // }));

    // c.bench_function("as_test", |b| b.iter(|| as_test(black_box(1000000))));

    c.bench_function("bloom_test", |b| b.iter(|| bloom_test(&mut bloom, &inputs[..])));
    c.bench_function("filter_test", |b| b.iter(|| filter_test(&mut filter, &inputs[..])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
