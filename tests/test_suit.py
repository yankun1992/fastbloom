# -*- coding: utf-8 -*-

from fastbloom_rs import PyFilterBuilder


def py_builder_test():
    pybuilder = PyFilterBuilder(100000000, 0.01)
    bloom = pybuilder.build_bloom_filter()

    bloom.add_bytes(b'hello')
    print(bloom.contains_bytes(b'hello'))
    print(bloom.contains_bytes(b'hello world'))


if __name__ == '__main__':
    py_builder_test()
