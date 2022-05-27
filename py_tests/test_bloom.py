# -*- coding: utf-8 -*-

from fastbloom_rs import BloomFilter, FilterBuilder


def test_bloom_builder():
    bloom = BloomFilter(100_000_000, 0.01)
    bloom.add_bytes(b'hello')
    bloom.add(87)

    assert bloom.contains_bytes(b'hello')
    assert bloom.contains('hello')
    assert bloom.contains_int(87)
    assert bloom.contains(87)
    assert b'hello' in bloom
    assert not bloom.contains_bytes(b'hello world')

    bloom2 = BloomFilter.from_int_array(bloom.get_int_array(), bloom.hashes())

    assert bloom2.contains_bytes(b'hello')
    assert bloom2.contains('hello')
    assert bloom2.contains_int(87)
    assert bloom2.contains(87)
    assert b'hello' in bloom2
    assert not bloom2.contains_bytes(b'hello world')

    bloom3 = BloomFilter.from_bytes(bloom.get_bytes(), bloom.hashes())

    assert bloom3.contains_bytes(b'hello')
    assert bloom3.contains('hello')
    assert bloom3.contains_int(87)
    assert bloom3.contains(87)
    assert b'hello' in bloom3
    assert not bloom3.contains_bytes(b'hello world')

    builder = FilterBuilder(100_000_000, 0.01)
    bloom4 = builder.build_bloom_filter()
    bloom4.add_bytes(b'hello')
    bloom4.add(87)
    assert bloom4.contains_bytes(b'hello')
    assert bloom4.contains('hello')
    assert bloom4.contains_int(87)
    assert bloom4.contains(87)
    assert b'hello' in bloom4
    assert not bloom4.contains_bytes(b'hello world')


def test_bloom_add():
    bloom = BloomFilter(100_000_000, 0.01)
    for data in range(0, 10_000_000):
        bloom.add_int(data)

    for data in range(0, 10_000_000):
        assert data in bloom

    assert not (1000_000_000 in bloom)
    assert not ('hello' in bloom)


def test_bloom_op():
    bloom = BloomFilter(100_000_000, 0.001)
    bloom.add_bytes(b'hello')
    bloom.add(87)

    assert b'hello' in bloom
    assert 87 in bloom

    bloom.clear()
    assert not (b'hello' in bloom)
    assert not (87 in bloom)


def test_bloom_union():
    bloom = BloomFilter(100_000_000, 0.01)
    bloom.add_bytes(b'hello')
    assert bloom.contains_bytes(b'hello')
    assert not bloom.contains(87)

    bloom2 = BloomFilter(100_000_000, 0.01)
    bloom2.add(87)
    assert not bloom2.contains_bytes(b'hello')
    assert bloom2.contains(87)

    bloom.union(bloom2)
    assert bloom.contains_bytes(b'hello')
    assert bloom.contains(87)


def test_bloom_intersect():
    bloom = BloomFilter(100_000_000, 0.01)
    bloom.add_bytes(b'hello')
    bloom.add(87)
    assert bloom.contains_bytes(b'hello')
    assert bloom.contains(87)

    bloom2 = BloomFilter(100_000_000, 0.01)
    bloom2.add(87)
    assert not bloom2.contains_bytes(b'hello')
    assert bloom2.contains(87)

    bloom.intersect(bloom2)
    assert not bloom.contains_bytes(b'hello')
    assert bloom.contains(87)
