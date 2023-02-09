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
