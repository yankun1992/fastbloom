# -*- coding: utf-8 -*-

"""
benchmark compare with pybloom_live
"""

import timeit

from pybloom_live import BloomFilter

from fastbloom_rs import BloomFilter as FastFilter

bloom = BloomFilter(capacity=100_000_000, error_rate=0.001)
fast_filter = FastFilter(100_000_000, 0.001)

data = [str(x) for x in range(1_000_000)]


def pybloom_insert():
    for ele in data:
        bloom.add(ele)


def pybloom_check():
    for ele in data:
        check = ele in bloom


def fastbloom_insert():
    for ele in data:
        fast_filter.add_str(ele)


def fastbloom_batch_insert():
    fast_filter.add_str_batch(data)


def fastbloom_check():
    for ele in data:
        check = fast_filter.contains_str(ele)


def test_bench():
    res = timeit.timeit(pybloom_insert, number=1)
    print("\npybloom_insert\ttimeit\t" + str(res))

    res = timeit.timeit(fastbloom_insert, number=1)
    print("fastbloom_insert\ttimeit\t" + str(res))

    fast_filter.clear()

    res = timeit.timeit(fastbloom_batch_insert, number=1)
    print("fastbloom_batch_insert\ttimeit\t" + str(res))

    res = timeit.timeit(pybloom_check, number=1)
    print("pybloom_check\ttimeit\t" + str(res))

    res = timeit.timeit(fastbloom_check, number=1)
    print("fastbloom_check\ttimeit\t" + str(res))
