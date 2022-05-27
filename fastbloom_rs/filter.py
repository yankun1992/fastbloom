# -*- coding: utf-8 -*-

from typing import Union, Sequence

from fastbloom_rs import PyFilterBuilder, PyBloomFilter


class FilterBuilder(object):
    """
    Constructs a new Bloom Filter Builder by specifying the expected size of the filter and the
    tolerable false positive probability. The size of the BLoom filter in in bits and the
    optimal number of hash functions will be inferred from this.

    :param expected_elements: expected size of the filter
    :param false_positive_probability: tolerable false positive probability
    """

    def __init__(self, expected_elements: Union[int, PyFilterBuilder], false_positive_probability: float = 0.01):
        if isinstance(expected_elements, int):
            self._py_builder = PyFilterBuilder(expected_elements, false_positive_probability)
        elif isinstance(expected_elements, PyFilterBuilder):
            self._py_builder = expected_elements
        else:
            raise Exception("expected_elements must be integer")

    def expected_elements(self) -> int:
        """
        expected size of the filter

        :return:
        """
        return self._py_builder.expected_elements()

    def false_positive_probability(self) -> float:
        """
        tolerable false positive probability

        :return:
        """
        return self._py_builder.false_positive_probability()

    def size(self) -> int:
        """
        the size of the bloom filter in bits.

        :return:
        """
        return self._py_builder.size()

    def hashes(self) -> int:
        """
         the hash function number of the Bloom filter.

        :return:
        """
        return self._py_builder.hashes()

    def build_bloom_filter(self) -> "BloomFilter":
        """
        Constructs a Bloom filter using the specified parameters and computing missing parameters
        if possible (e.g. the optimal Bloom filter bit size).

        :return:
        """
        return BloomFilter(self._py_builder.build_bloom_filter())


class BloomFilter(object):
    """
    A bloom filter powered by rust.

    :param expected_elements: expected elements in the filter
    :param false_positive_probability: tolerable false positive probability
    """

    def __init__(self, expected_elements: Union[int, PyBloomFilter], false_positive_probability: float = 0.01):
        if isinstance(expected_elements, int):
            self._py_builder = PyFilterBuilder(expected_elements, false_positive_probability)
            self._py_bloom = self._py_builder.build_bloom_filter()
        elif isinstance(expected_elements, PyBloomFilter):
            self._py_bloom = expected_elements
        else:
            raise Exception("expected_elements must be integer")

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

    def add_int_batch(self, array: Sequence[int]):
        self._py_bloom.add_int_batch(array)

    def add_str(self, element: str):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_bloom.add_str(element)

    def add_str_batch(self, array: Sequence[str]):
        self._py_bloom.add_str_batch(array)

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

    def config(self) -> FilterBuilder:
        """
        Returns the configuration/builder of the Bloom filter.

        :return:
        """
        return FilterBuilder(self._py_bloom.config())

    def hashes(self) -> int:
        """
        Returns the hash function number of the Bloom filter.

        :return:
        """
        return self._py_bloom.hashes()

    def get_bytes(self) -> bytes:
        """
        Return the underlying byte vector of the Bloom filter.

        :return:
        """
        return self._py_bloom.get_bytes()

    def get_int_array(self) -> Sequence[int]:
        """
        Return the underlying u32 vector of the Bloom filter.

        :return:
        """
        return self._py_bloom.get_int_array()

    def clear(self):
        """
        Removes all elements from the filter (i.e. resets all bits to zero).

        :return:
        """
        self._py_bloom.clear()

    def is_empty(self) -> bool:
        """
        Returns [true] if the Bloom filter does not contain any elements

        :return:
        """
        return self._py_bloom.is_empty()

    def union(self, other: "BloomFilter") -> bool:
        """
        Performs the union operation on two compatible bloom filters. This is achieved through a
        bitwise OR operation on their bit vectors. This operations is lossless, i.e. no elements
        are lost and the bloom filter is the same that would have resulted if all elements wer
        directly inserted in just one bloom filter.

        :param other:
        :return:
        """
        return self._py_bloom.union(other._py_bloom)

    def intersect(self, other: "BloomFilter") -> bool:
        """
        Performs the intersection operation on two compatible bloom filters. This is achieved
        through a bitwise AND operation on their bit vectors. The operations doesn't introduce
        any false negatives but it does raise the false positive probability. The the false
        positive probability in the resulting Bloom filter is at most the false-positive probability
        in one of the constituent bloom filters

        :param other:
        :return:
        """
        return self._py_bloom.intersect(other._py_bloom)

    def __contains__(self, item: Union[str, int, bytes]):
        return self.contains(item)

    @staticmethod
    def from_bytes(array: bytes, hashes: int) -> "BloomFilter":
        """
        Build a Bloom filter form [u8].

        :param array: byte array
        :param hashes: hash function number of the Bloom filter
        :return:
        """
        py_bloom = PyBloomFilter.from_bytes(array, hashes)
        return BloomFilter(py_bloom)

    @staticmethod
    def from_int_array(array: Sequence[int], hashes: int) -> "BloomFilter":
        """
        Build a Bloom filter form [u32].

        :param array: integer(u32) array
        :param hashes: hash function number of the Bloom filter
        :return:
        """
        py_bloom = PyBloomFilter.from_int_array(array, hashes)
        return BloomFilter(py_bloom)
