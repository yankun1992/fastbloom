# -*- coding: utf-8 -*-

import timeit

from pybloom_live import BloomFilter

from fastbloom_rs import BloomFilter as FastFilter

bloom = BloomFilter(capacity=100_000_000, error_rate=0.01)
fast_filter = FastFilter(100_000_000, 0.01)

data = [str(x) * 4 for x in range(10_000_000)]

data_bytes = [x.encode('UTF-8') for x in data]


def bloom_insert():
    for ele in data:
        bloom.add(ele)


def filter_insert():
    for ele in data:
        fast_filter.add_str(ele)


if __name__ == '__main__':
    # res = timeit.timeit(bloom_insert, number=1)
    # print("bloom_insert " + str(res))

    res = timeit.timeit(filter_insert, number=1)
    print("filter_insert " + str(res))
