# -*- coding: utf-8 -*-

from typing import Union

from fastbloom_rs import PyFilterBuilder


class BloomFilter(object):
    """
    A bloom filter powered by rust.

    :param expected_elements: expected elements in the filter
    :param false_positive_probability: tolerable false positive probability
    """

    def __init__(self, expected_elements: int, false_positive_probability: float):
        self._py_builder = PyFilterBuilder(expected_elements, false_positive_probability)
        self._py_bloom = self._py_builder.build_bloom_filter()

    def add(self, element: Union[str, int, bytes]):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        if isinstance(element, int):
            self._py_bloom.add_int(element)
        elif isinstance(element, str):
            self._py_bloom.add_str(element)
        elif isinstance(element, bytes):
            self._py_bloom.add_bytes(element)
        else:
            self._py_bloom.add_str(str(element))

    def add_int(self, element: int):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_bloom.add_int(element)

    def add_str(self, element: str):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_bloom.add_str(element)

    def add_bytes(self, element: bytes):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_bloom.add_bytes(element)

    def contains(self, element: Union[str, int, bytes]) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        if isinstance(element, int):
            return self._py_bloom.contains_int(element)
        elif isinstance(element, str):
            return self._py_bloom.contains_str(element)
        elif isinstance(element, bytes):
            return self._py_bloom.contains_bytes(element)
        else:
            return self._py_bloom.contains_str(str(element))

    def contains_int(self, element: int) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_bloom.contains_int(element)

    def contains_str(self, element: str) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_bloom.contains_str(element)

    def contains_bytes(self, element: bytes) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_bloom.contains_bytes(element)

    def __contains__(self, item: Union[str, int, bytes]):
        return self.contains(item)
