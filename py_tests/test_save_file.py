# -*- coding: utf-8 -*-
import os
import shutil

from fastbloom_rs import BloomFilter, FilterBuilder


def test_save():
    bloom = BloomFilter(100_000, 0.01)
    bloom.add_bytes(b'hello')
    bloom.add(87)

    builder = FilterBuilder(100_000, 0.01)
    cbf = builder.build_counting_bloom_filter()  # type: CountingBloomFilter

    cbf.add('hello')
    cbf.add(87)

    if os.path.exists('data'):
        shutil.rmtree('data')
        os.makedirs('data')
        try:
            os.remove('data/bloom.bin')
            os.remove('data/counting.bin')
        except Exception as e:
            pass

    with open('data/bloom.bin', "wb") as f:
        array = bloom.get_bytes()
        f.write(array)
        print(len(array), bloom.hashes())

    with open('data/counting.bin', 'wb') as f:
        array = cbf.get_bytes()
        f.write(array)
        print(len(array), cbf.hashes())


if __name__ == '__main__':
    test_save()
