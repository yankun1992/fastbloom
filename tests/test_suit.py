# -*- coding: utf-8 -*-

import pandas as pd

from fastbloom_rs import PyFilterBuilder


def py_builder_test():
    pybuilder = PyFilterBuilder(100000000, 0.01)
    bloom = pybuilder.build_bloom_filter()

    bloom.add_bytes(b'hello')
    print(bloom.contains_bytes(b'hello'))
    print(bloom.contains_bytes(b'hello world'))


if __name__ == '__main__':
    # py_builder_test()
    vid = ["hello_Company", "world_Company"]
    vid = pd.Series(vid).map(lambda x: x.split('_')[0])
    print(vid)
