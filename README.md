<h1>fastbloom</h1>

[![OSCS Status](https://www.oscs1024.com/platform/badge/yankun1992/fastbloom.svg?size=small)](https://www.oscs1024.com/project/yankun1992/fastbloom?ref=badge_small)
[![docs.rs](https://img.shields.io/docsrs/fastbloom-rs/latest)](https://docs.rs/fastbloom-rs)
[![Test Rust](https://github.com/yankun1992/fastbloom/actions/workflows/test_rust.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/test_rust.yml)
[![Test Python](https://github.com/yankun1992/fastbloom/actions/workflows/test_python.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/test_python.yml)
[![Benchmark](https://github.com/yankun1992/fastbloom/actions/workflows/benchmark.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/benchmark.yml)
[![Crates Latest Release](https://img.shields.io/crates/v/fastbloom-rs)](https://crates.io/crates/fastbloom-rs)
[![PyPI Latest Release](https://img.shields.io/pypi/v/fastbloom-rs)](https://pypi.org/project/fastbloom-rs/)
![Sonatype Nexus (Snapshots)](https://img.shields.io/nexus/s/io.github.yankun1992/fastbloom?server=https%3A%2F%2Fs01.oss.sonatype.org)

A fast [bloom filter](#BloomFilter) | [counting bloom filter](#countingbloomfilter) implemented by Rust for Rust and
Python!

Language: [简体中文](./docs/README.zh_cn.md)

- [setup](#setup)
    - [Python](#python)
        - [requirements](#requirements)
        - [install](#install)
    - [Rust](#rust)
    - [Java](#java)
- [Examples](#examples)
    - [BloomFilter](#bloomfilter)
        - [Python](#python-1)
        - [Rust](#rust-1)
    - [CountingBloomFilter](#countingbloomfilter)
        - [Python](#python-2)
        - [Rust](#rust-2)
- [benchmark](#benchmark)
    - [computer info](#computer-info)
    - [add one str to bloom filter](#add-one-str-to-bloom-filter)
    - [add one million to bloom filter](#add-one-million-to-bloom-filter)
    - [check one contains in bloom filter](#check-one-contains-in-bloom-filter)
    - [check one not contains in bloom filter](#check-one-not-contains-in-bloom-filter)
    - [add one str to counting bloom filter](#add-one-str-to-counting-bloom-filter)
    - [add one million to counting bloom filter](#add-one-million-to-counting-bloom-filter)

# setup

## Python

### requirements

```
Python >= 3.7
```

### install

Install the latest fastbloom version with:

```bash
pip install fastbloom-rs
```

## Rust

```toml
fastbloom-rs = "{latest}"
```

## Java
maven
```xml
<dependency>
    <groupId>io.github.yankun1992</groupId>
    <artifactId>fastbloom</artifactId>
    <version>{latest-version}</version>
</dependency>
```

# Examples

## BloomFilter

A Bloom filter is a space-efficient probabilistic data structure, conceived by Burton Howard
Bloom in 1970, that is used to test whether an element is a member of a set. False positive
matches are possible, but false negatives are not.

**Reference**: Bloom, B. H. (1970). Space/time trade-offs in hash coding with allowable errors.
Communications of the ACM, 13(7), 422-426.
[Full text article](http://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf)

### Python

basic usage

```python
from fastbloom_rs import BloomFilter

bloom = BloomFilter(100_000_000, 0.01)

bloom.add_str('hello')
bloom.add_bytes(b'world')
bloom.add_int(9527)

assert bloom.contains('hello')
assert bloom.contains(b'world')
assert bloom.contains(9527)

assert not bloom.contains('hello world')
```

build bloom filter from bytes or list

```python
from fastbloom_rs import BloomFilter

bloom = BloomFilter(100_000_000, 0.01)
bloom.add_str('hello')
assert bloom.contains('hello')

bloom2 = BloomFilter.from_bytes(bloom.get_bytes(), bloom.hashes())
assert bloom2.contains('hello')

bloom3 = BloomFilter.from_int_array(bloom.get_int_array(), bloom.hashes())
assert bloom3.contains('hello')

```

there are some bulk api for python to reduce ffi cost between python and rust

```python
bloom = BloomFilter(100_000_000, 0.01)
inserts = [1, 2, 3, 4, 5, 6, 7, 9, 18, 68, 90, 100]
checks = [1, 2, 3, 4, 5, 6, 7, 9, 18, 68, 90, 100, 190, 290, 390]
results = [True, True, True, True, True, True, True, True, True, True, True, True, False, False, False]

bloom.add_int_batch(inserts)
contains = bloom.contains_int_batch(checks)
assert contains == results

bloom.add_str_batch(list(map(lambda x: str(x), inserts)))
assert bloom.contains_str_batch(list(map(lambda x: str(x), checks))) == results

bloom.add_bytes_batch(list(map(lambda x: bytes(x), inserts)))
assert bloom.contains_bytes_batch(list(map(lambda x: bytes(x), checks))) == results
```

more examples at [py_tests](py_tests/test_bloom.py).

### Rust

```rust
use fastbloom_rs::{BloomFilter, FilterBuilder};

let mut bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();

bloom.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
assert_eq!(bloom.contains(b"helloworld!"), false);
```

more examples at [docs.rs](https://docs.rs/fastbloom-rs)

## CountingBloomFilter

A Counting Bloom filter works in a similar manner as a regular Bloom filter; however, it is
able to keep track of insertions and deletions. In a counting Bloom filter, each entry in the
Bloom filter is a small counter associated with a basic Bloom filter bit.

**Reference**: F. Bonomi, M. Mitzenmacher, R. Panigrahy, S. Singh, and G. Varghese, “An Improved
Construction for Counting Bloom Filters,” in 14th Annual European Symposium on
Algorithms, LNCS 4168, 2006

### Python

```python
from fastbloom_rs import CountingBloomFilter

cbf = CountingBloomFilter(1000_000, 0.01)
cbf.add('hello')
cbf.add('hello')
assert 'hello' in cbf
cbf.remove('hello')
assert 'hello' in cbf  # because 'hello' added twice. 
# If add same element larger than 15 times, then remove 15 times the filter will not contain the element.
cbf.remove('hello')
assert 'hello' not in cbf
```

A CountingBloomFilter has a four bits counter to save hash index, so when insert an
element repeatedly, the counter will spill over quickly. So, you can set
`enable_repeat_insert` to `False` to check whether the element has added.
if it has added, it will not add again. `enable_repeat_insert` default set to `True`.

```python
from fastbloom_rs import CountingBloomFilter

cbf = CountingBloomFilter(1000_000, 0.01, False)
cbf.add('hello')
cbf.add('hello')  # because enable_repeat_insert=False, this addition will not take effect. 
assert 'hello' in cbf
cbf.remove('hello')
assert 'hello' not in cbf 
```

more examples at [py_tests](py_tests/test_counting_bloom_filter.py).

### Rust

```rust
use fastbloom_rs::{CountingBloomFilter, FilterBuilder};

let mut builder = FilterBuilder::new(100_000, 0.01);
let mut cbf = builder.build_counting_bloom_filter();
cbf.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
```

# benchmark

For detailed performance comparisons between fastbloom-rs and other Python bloom filter libraries, see the [library comparison benchmark](benches/lib_comparison/). This benchmark compares fastbloom-rs against pyprobables and pybloomfilter3 across various configurations and provides comprehensive performance metrics.

## computer info

| CPU                                    | Memory | OS         |
|----------------------------------------|--------|------------|
| AMD Ryzen 7 5800U with Radeon Graphics | 16G    | Windows 10 |

## add one str to bloom filter

Benchmark insert one str to bloom filter:

```text
bloom_add_test          time:   [41.168 ns 41.199 ns 41.233 ns]
                        change: [-0.4891% -0.0259% +0.3417%] (p = 0.91 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
```

## add one million to bloom filter

Benchmark loop insert `(1..1_000_000).map(|n| { n.to_string() })` to bloom filter:

```text
bloom_add_all_test      time:   [236.24 ms 236.86 ms 237.55 ms]
                        change: [-3.4346% -2.9050% -2.3524%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

## check one contains in bloom filter

```text
bloom_contains_test     time:   [42.065 ns 42.102 ns 42.156 ns]
                        change: [-0.7830% -0.5901% -0.4029%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe
```

## check one not contains in bloom filter

```text
bloom_not_contains_test time:   [22.695 ns 22.727 ns 22.773 ns]
                        change: [-3.1948% -2.9695% -2.7268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
```

## add one str to counting bloom filter

```text
counting_bloom_add_test time:   [60.822 ns 60.861 ns 60.912 ns]
                        change: [+0.2427% +0.3772% +0.5579%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe
```

## add one million to counting bloom filter

Benchmark loop insert `(1..1_000_000).map(|n| { n.to_string() })` to counting bloom filter:

```text
counting_bloom_add_million_test
                        time:   [272.48 ms 272.58 ms 272.68 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
```
