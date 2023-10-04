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

    assert not bloom.add_if_not_contains('hello2')
    assert bloom.contains('hello2')
    assert not bloom.add_if_not_contains(88)
    assert bloom.contains(88)

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


def test_bloom_estimate_set_cardinality():
    bloom = BloomFilter(100_000_000, 0.01)
    for data in range(0, 10_000_000):
        bloom.add_int(data)
        
    assert (bloom.estimate_set_cardinality() < 10_100_000) and (bloom.estimate_set_cardinality() > 9_900_000)


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


def test_hash_indices():
    bloom = BloomFilter(100_000_000, 0.01)
    bloom.add_bytes(b'hello')
    bloom.add_str("world")
    bloom.add(87)

    bloom2 = BloomFilter(100_000_000, 0.01)
    bloom2.add_str("Yan Kun")

    assert bloom.get_hash_indices(b'hello') == bloom2.get_hash_indices(b'hello')

    assert bloom.contains_hash_indices(bloom.get_hash_indices(b'hello'))
    assert bloom.contains_hash_indices(bloom.get_hash_indices(87))
    assert bloom.contains_hash_indices(bloom.get_hash_indices("world"))
    assert not bloom.contains_hash_indices(bloom.get_hash_indices("Yan Kun"))

    assert not bloom2.contains_hash_indices(bloom2.get_hash_indices(b'hello'))
    assert not bloom2.contains_hash_indices(bloom2.get_hash_indices(87))
    assert not bloom2.contains_hash_indices(bloom2.get_hash_indices("world"))
    assert bloom2.contains_hash_indices(bloom2.get_hash_indices("Yan Kun"))


def test_batch_check():
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
