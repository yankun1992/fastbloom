<h1>fastbloom</h1>

[![OSCS Status](https://www.oscs1024.com/platform/badge/yankun1992/fastbloom.svg?size=small)](https://www.oscs1024.com/project/yankun1992/fastbloom?ref=badge_small)
[![docs.rs](https://img.shields.io/docsrs/fastbloom-rs/latest)](https://docs.rs/fastbloom-rs)
[![Test Rust](https://github.com/yankun1992/fastbloom/actions/workflows/test_rust.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/test_rust.yml)
[![Test Python](https://github.com/yankun1992/fastbloom/actions/workflows/test_python.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/test_python.yml)
[![Benchmark](https://github.com/yankun1992/fastbloom/actions/workflows/benchmark.yml/badge.svg)](https://github.com/yankun1992/fastbloom/actions/workflows/benchmark.yml)
[![Crates Latest Release](https://img.shields.io/crates/v/fastbloom-rs)](https://crates.io/crates/fastbloom-rs)
[![PyPI Latest Release](https://img.shields.io/pypi/v/fastbloom-rs)](https://pypi.org/project/fastbloom-rs/)

使用 Rust 实现的  [bloom filter](#BloomFilter) | [counting bloom filter](#countingbloomfilter) Python 库及 Rust 库。

Language: [English](../README.md)

- [安装](#setup)
    - [Python](#python)
        - [requirements](#requirements)
        - [install](#install)
    - [Rust](#rust)
- [例子](#examples)
    - [BloomFilter](#bloomfilter)
        - [Python](#python-1)
        - [Rust](#rust-1)
    - [CountingBloomFilter](#countingbloomfilter)
        - [Python](#python-2)
        - [Rust](#rust-2)
- [性能测试报告](#benchmark)
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

使用如下命令安装 fastbloom 最新版本：

```bash
pip install fastbloom-rs
```

## Rust

```toml
fastbloom-rs = "{latest}"
```

# Examples

## BloomFilter
布隆过滤器（Bloom Filter）是1970年由布隆提出的。它实际上是一个很长的二进制向量和一系列随机映射函数。布隆过滤器
可以用于检索一个元素是否在一个集合中。它的优点是空间效率和查询时间都比一般的算法要好的多，缺点是有一定的误识别率和删除困难。

**参考**: Bloom, B. H. (1970). Space/time trade-offs in hash coding with allowable errors.
Communications of the ACM, 13(7), 422-426.
[全文](http://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf)

### Python

基础用法

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

基于 bytes 或者 list 构造布隆过滤器

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

由于python与rust之间的数据转换有一定的性能开销，所以`fastbloom`提供了一些批量操作api用于减少ffi开销

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

更多例子参考 [py_tests](py_tests/test_bloom.py).

### Rust

```rust
use fastbloom_rs::{BloomFilter, FilterBuilder};

let mut bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();

bloom.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
assert_eq!(bloom.contains(b"helloworld!"), false);
```

更多例子参考 [docs.rs](https://docs.rs/fastbloom-rs)

## CountingBloomFilter

计数布隆过滤器的工作方式与常规布隆过滤器类似;但是，它能够跟踪插入和删除。在计数布隆过滤器中，布隆过滤器的每个
条目都是一个与基本布隆过滤器位相关联的小计数器。

**参考**: F. Bonomi, M. Mitzenmacher, R. Panigrahy, S. Singh, and G. Varghese, “An Improved
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

本计数布隆过滤器使用4bit计数器存储hash索引，所以当重复插入同一个元素到过滤器中，计数器很快就会位溢出，
所以可以设置 `enable_repeat_insert` 为 `False` 用于避免重复插入，如果元素已经加入过滤器中，设置
`enable_repeat_insert` 为 `False` 将使元素不会重复插入。 `enable_repeat_insert` 默认为 `True`。

```python
from fastbloom_rs import CountingBloomFilter

cbf = CountingBloomFilter(1000_000, 0.01, False)
cbf.add('hello')
cbf.add('hello')  # because enable_repeat_insert=False, this addition will not take effect. 
assert 'hello' in cbf
cbf.remove('hello')
assert 'hello' not in cbf 
```

更多例子参考 [py_tests](py_tests/test_counting_bloom_filter.py).

### Rust

```rust
use fastbloom_rs::{CountingBloomFilter, FilterBuilder};

let mut builder = FilterBuilder::new(100_000, 0.01);
let mut cbf = builder.build_counting_bloom_filter();
cbf.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
```

# benchmark

## computer info

| CPU                                    | Memory | OS         |
|----------------------------------------|--------|------------|
| AMD Ryzen 7 5800U with Radeon Graphics | 16G    | Windows 10 |

## add one str to bloom filter

测试添加一个字符串到布隆过滤器:

```text
bloom_add_test          time:   [41.168 ns 41.199 ns 41.233 ns]
                        change: [-0.4891% -0.0259% +0.3417%] (p = 0.91 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
```

## add one million to bloom filter

添加一百万字符串（`(1..1_000_000).map(|n| { n.to_string() })`）到布隆过滤器：

```text
bloom_add_all_test      time:   [236.24 ms 236.86 ms 237.55 ms]
                        change: [-3.4346% -2.9050% -2.3524%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

## check one contains in bloom filter

测试布隆过滤器包含的元素：

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

测试布隆过滤器不包含的元素：

```text
bloom_not_contains_test time:   [22.695 ns 22.727 ns 22.773 ns]
                        change: [-3.1948% -2.9695% -2.7268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
```

## add one str to counting bloom filter

测试添加一个字符串到计数布隆过滤器：

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

添加一百万字符串（`(1..1_000_000).map(|n| { n.to_string() })`）到计数布隆过滤器：

```text
counting_bloom_add_million_test
                        time:   [272.48 ms 272.58 ms 272.68 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
```