# -*- coding: utf-8 -*-

from fastbloom_rs import CountingBloomFilter, FilterBuilder


def test_builder():
    builder = FilterBuilder(100_000, 0.01)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add('hello')
    assert 'hello' in cbf

    cbf.remove('hello')
    assert 'hello' not in cbf


def test_from_builder():
    builder = FilterBuilder(100_000, 0.01)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter
    cbf.add('hello')
    assert 'hello' in cbf

    cbf_copy = CountingBloomFilter.from_bytes(cbf.get_bytes(), cbf.hashes())
    assert 'hello' in cbf_copy

    cbf_copy = CountingBloomFilter.from_int_array(cbf.get_int_array(), cbf.hashes())
    assert 'hello' in cbf_copy


def test_repeat_insert():
    builder = FilterBuilder(100_000, 0.01)
    # enable repeat insert
    builder.enable_repeat_insert(True)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add('hello')
    cbf.add('hello')
    assert 'hello' in cbf

    cbf.remove('hello')
    assert 'hello' in cbf
    cbf.remove('hello')
    assert 'hello' not in cbf

    # not enable repeat insert
    builder.enable_repeat_insert(False)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add('hello')
    cbf.add('hello')
    assert 'hello' in cbf

    cbf.remove('hello')
    assert 'hello' not in cbf


def test_op():
    builder = FilterBuilder(100_000, 0.01)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter
    cbf.add('hello')
    assert 'hello' in cbf

    cbf.clear()
    assert 'hello' not in cbf


def test_hash_indices():
    builder = FilterBuilder(100_000, 0.01)
    # enable repeat insert
    builder.enable_repeat_insert(True)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    builder2 = FilterBuilder(100_000, 0.01)
    builder2.enable_repeat_insert(True)
    cbf2 = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add(b'hello')
    cbf.add(31)
    cbf.add('world')

    cbf2.add('Yan Kun')

    assert cbf.get_hash_indices(b'hello') == cbf2.get_hash_indices(b'hello')

    assert cbf.contains_hash_indices(cbf.get_hash_indices(b'hello'))
    assert cbf.contains_hash_indices(cbf.get_hash_indices(31))
    assert cbf.contains_hash_indices(cbf.get_hash_indices('world'))
    assert not cbf.contains_hash_indices(cbf.get_hash_indices('Yan Kun'))

    assert not cbf2.contains_hash_indices(cbf2.get_hash_indices(b'hello'))
    assert not cbf2.contains_hash_indices(cbf2.get_hash_indices(31))
    assert not cbf2.contains_hash_indices(cbf2.get_hash_indices('world'))
    assert cbf2.contains_hash_indices(cbf2.get_hash_indices('Yan Kun'))


def test_estimate_count():
    builder = FilterBuilder(100_000, 0.01)
    # enable repeat insert
    builder.enable_repeat_insert(True)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add(b'hello')

    assert cbf.estimate_count(b'hello') == 1

    for index in cbf.get_hash_indices(b'hello'):
        assert cbf.counter_at(index) == 1

    cbf.add(b'world')
    for index in cbf.get_hash_indices(b'world'):
        assert cbf.counter_at(index) <= 2

    cbf.add(b'hello')
    assert cbf.estimate_count(b'hello') == 2


def test_batch():
    builder = FilterBuilder(100_000, 0.01)
    # enable repeat insert
    builder.enable_repeat_insert(True)
    bloom = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    inserts = [1, 2, 3, 4, 5, 6, 7, 9, 18, 68, 90]
    checks = [1, 2, 3, 4, 5, 6, 7, 9, 18, 68, 90, 190, 290, 390]
    results = [True, True, True, True, True, True, True, True, True, True, True, False, False, False]

    bloom.add_int_batch(inserts)
    contains = bloom.contains_int_batch(checks)
    assert contains == results

    bloom.add_str_batch(list(map(lambda x: str(x), inserts)))
    assert bloom.contains_str_batch(list(map(lambda x: str(x), checks))) == results

    bloom.add_bytes_batch(list(map(lambda x: bytes(x), inserts)))
    assert bloom.contains_bytes_batch(list(map(lambda x: bytes(x), checks))) == results
