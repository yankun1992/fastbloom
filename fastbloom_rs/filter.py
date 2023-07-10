# -*- coding: utf-8 -*-

from typing import Union, Sequence

from fastbloom_rs import PyFilterBuilder, PyBloomFilter, PyCountingBloomFilter


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

    def enable_repeat_insert(self, enable: bool):
        """
        Whether to allow multiple inserts of the same element. Only use for Counting Bloom Filter.

        :param enable:
        :return:
        """
        self._py_builder.enable_repeat_insert(enable)

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

    def build_counting_bloom_filter(self) -> "CountingBloomFilter":
        """
        Constructs a Counting Bloom filter using the specified parameters and computing missing parameters
        if possible (e.g. the optimal Bloom filter counter size).

        :return:
        """
        return CountingBloomFilter(self._py_builder.build_counting_bloom_filter())


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

    def add_if_not_contains(self, element: Union[str, int, bytes]) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).
        And if it is not in this filter, add it to the filter.

        :param element: value to test
        :return: “False” if this element did not exist in the Bloom filter before, and then this method will insert
        this element into the current filter. “True” if the element is already in the Bloom filter.
        """
        if isinstance(element, int):
            return self._py_bloom.add_int_if_not_contains(element)
        elif isinstance(element, str):
            return self._py_bloom.add_str_if_not_contains(element)
        elif isinstance(element, bytes):
            return self._py_bloom.add_bytes_if_not_contains(element)
        else:
            return self._py_bloom.add_str_if_not_contains(str(element))

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

    def add_bytes_batch(self, elements: Sequence[bytes]):
        """
        Add all bytes to the filter.
        :param elements: byte list
        :return:
        """
        self._py_bloom.add_bytes_batch(elements)

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

    def contains_int_batch(self, elements: Sequence[int], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type int
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, int)
        return self._py_bloom.contains_int_batch(elements)

    def contains_str(self, element: str) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_bloom.contains_str(element)

    def contains_str_batch(self, elements: Sequence[str], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type str
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, str)
        return self._py_bloom.contains_str_batch(elements)

    def contains_bytes(self, element: bytes) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_bloom.contains_bytes(element)

    def contains_bytes_batch(self, elements: Sequence[bytes], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type bytes
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, bytes)
        return self._py_bloom.contains_bytes_batch(elements)

    def contains_hash_indices(self, indices: Sequence[int]) -> bool:
        """
        Tests whether a hashes indices is present in the filter
        :param indices: indices array
        :return:
        """
        return self._py_bloom.contains_hash_indices(indices)

    def get_hash_indices(self, element: Union[str, int, bytes]) -> Sequence[int]:
        """
        Get the hashes indices of the element in the filter.
        :param element: to compute
        :return: indices array
        """
        if isinstance(element, int):
            return self._py_bloom.get_hash_indices_int(element)
        elif isinstance(element, str):
            return self._py_bloom.get_hash_indices_str(element)
        elif isinstance(element, bytes):
            return self._py_bloom.get_hash_indices(element)
        else:
            return self._py_bloom.get_hash_indices_str(str(element))

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


class CountingBloomFilter(object):
    """
    A Counting Bloom Filter powered by rust. The counter is four bits.

    :param expected_elements: expected elements in the filter
    :param false_positive_probability: tolerable false positive probability
    :param enable_repeat_insert: Whether to allow multiple inserts of the same element.
    """

    def __init__(self, expected_elements: Union[int, PyCountingBloomFilter], false_positive_probability: float = 0.01,
                 enable_repeat_insert: bool = True):
        if isinstance(expected_elements, int):
            self._py_builder = PyFilterBuilder(expected_elements, false_positive_probability)
            self._py_builder.enable_repeat_insert(enable_repeat_insert)
            self._py_counting_bloom = self._py_builder.build_counting_bloom_filter()
        elif isinstance(expected_elements, PyCountingBloomFilter):
            self._py_counting_bloom = expected_elements
        else:
            raise Exception("expected_elements must be integer")

    def add(self, element: Union[str, int, bytes]):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        if isinstance(element, int):
            self._py_counting_bloom.add_int(element)
        elif isinstance(element, str):
            self._py_counting_bloom.add_str(element)
        elif isinstance(element, bytes):
            self._py_counting_bloom.add_bytes(element)
        else:
            self._py_counting_bloom.add_str(str(element))

    def remove(self, element: Union[str, int, bytes]):
        """
        Remove element to the filter.

        :param element:
        :return:
        """
        if isinstance(element, int):
            self._py_counting_bloom.remove_int(element)
        elif isinstance(element, str):
            self._py_counting_bloom.remove_str(element)
        elif isinstance(element, bytes):
            self._py_counting_bloom.remove_bytes(element)
        else:
            self._py_counting_bloom.remove_str(str(element))

    def add_int(self, element: int):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_counting_bloom.add_int(element)

    def add_int_batch(self, array: Sequence[int]):
        self._py_counting_bloom.add_int_batch(array)

    def remove_int(self, element: int):
        """
        Remove element from this filter.

        :param element:
        :return:
        """
        self._py_counting_bloom.remove_int(element)

    def add_str(self, element: str):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_counting_bloom.add_str(element)

    def add_str_batch(self, array: Sequence[str]):
        self._py_counting_bloom.add_str_batch(array)

    def remove_str(self, element: str):
        """
        Remove element from this filter.

        :param element:
        :return:
        """
        self._py_counting_bloom.remove_str(element)

    def add_bytes(self, element: bytes):
        """
        Add element to the filter.

        :param element: value to add
        :return:
        """
        self._py_counting_bloom.add_bytes(element)

    def add_bytes_batch(self, elements: Sequence[bytes]):
        """
        Add all bytes to the filter.

        :param elements: byte list
        :return:
        """
        self._py_counting_bloom.add_bytes_batch(elements)

    def remove_bytes(self, element: bytes):
        """
        Remove element from this filter.

        :param element:
        :return:
        """
        self._py_counting_bloom.remove_bytes(element)

    def contains(self, element: Union[str, int, bytes]) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        if isinstance(element, int):
            return self._py_counting_bloom.contains_int(element)
        elif isinstance(element, str):
            return self._py_counting_bloom.contains_str(element)
        elif isinstance(element, bytes):
            return self._py_counting_bloom.contains_bytes(element)
        else:
            return self._py_counting_bloom.contains_str(str(element))

    def contains_int(self, element: int) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_counting_bloom.contains_int(element)

    def contains_int_batch(self, elements: Sequence[int], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type int
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, int)
        return self._py_counting_bloom.contains_int_batch(elements)

    def contains_str(self, element: str) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_counting_bloom.contains_str(element)

    def contains_str_batch(self, elements: Sequence[str], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type str
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, str)
        return self._py_counting_bloom.contains_str_batch(elements)

    def contains_bytes(self, element: bytes) -> bool:
        """
        Tests whether an element is present in the filter (subject to the specified false positive rate).

        :param element: to test
        :return: bool
        """
        return self._py_counting_bloom.contains_bytes(element)

    def contains_bytes_batch(self, elements: Sequence[bytes], check_type: bool = True) -> Sequence[bool]:
        """
        Check all int element whether in the filter.

        :param elements: elements to check
        :param check_type: whether to check elements in elements is type bytes
        :return: a bool sequence for each element in elements is whether in this filter.
        """
        if check_type:
            for ele in elements:
                assert isinstance(ele, bytes)
        return self._py_counting_bloom.contains_bytes_batch(elements)

    def contains_hash_indices(self, indices: Sequence[int]) -> bool:
        """
        Tests whether a hashes indices is present in the filter
        :param indices: indices array
        :return:
        """
        return self._py_counting_bloom.contains_hash_indices(indices)

    def get_hash_indices(self, element: Union[str, int, bytes]) -> Sequence[int]:
        """
        Get the hashes indices of the element in the filter.
        :param element: to compute
        :return: indices array
        """
        if isinstance(element, int):
            return self._py_counting_bloom.get_hash_indices_int(element)
        elif isinstance(element, str):
            return self._py_counting_bloom.get_hash_indices_str(element)
        elif isinstance(element, bytes):
            return self._py_counting_bloom.get_hash_indices(element)
        else:
            return self._py_counting_bloom.get_hash_indices_str(str(element))

    def estimate_count(self, element: Union[str, int, bytes]) -> int:
        """
        Get the estimate count for element in this counting bloom filter.
        See: https://github.com/yankun1992/fastbloom/issues/3

        :param element:
        :return:
        """
        if isinstance(element, int):
            return self._py_counting_bloom.estimate_count_int(element)
        elif isinstance(element, str):
            return self._py_counting_bloom.estimate_count_str(element)
        elif isinstance(element, bytes):
            return self._py_counting_bloom.estimate_count(element)
        else:
            return self._py_counting_bloom.estimate_count_str(str(element))

    def counter_at(self, index: int) -> int:
        """
        Get the underlying counter at index.

        :param index: index of counter slot.
        :return:
        """
        assert index >= 0
        return self._py_counting_bloom.counter_at(index)

    def config(self) -> FilterBuilder:
        """
        Returns the configuration/builder of the Bloom filter.

        :return:
        """
        return FilterBuilder(self._py_counting_bloom.config())

    def hashes(self) -> int:
        """
        Returns the hash function number of the Bloom filter.

        :return:
        """
        return self._py_counting_bloom.hashes()

    def get_bytes(self) -> bytes:
        """
        Return the underlying byte vector of the Bloom filter.

        :return:
        """
        return self._py_counting_bloom.get_bytes()

    def get_int_array(self) -> Sequence[int]:
        """
        Return the underlying u32 vector of the Bloom filter.

        :return:
        """
        return self._py_counting_bloom.get_int_array()

    def clear(self):
        """
        Removes all elements from the filter (i.e. resets all bits to zero).

        :return:
        """
        self._py_counting_bloom.clear()

    def __contains__(self, item: Union[str, int, bytes]):
        return self.contains(item)

    @staticmethod
    def from_bytes(array: bytes, hashes: int, enable_repeat_insert: bool = True) -> "CountingBloomFilter":
        """
        Build a Counting Bloom filter form [u8].

        :param enable_repeat_insert:
        :param array: byte array
        :param hashes: hash function number of the Bloom filter
        :return:
        """
        py_bloom = PyCountingBloomFilter.from_bytes(array, hashes, enable_repeat_insert)
        return CountingBloomFilter(py_bloom)

    @staticmethod
    def from_int_array(array: Sequence[int], hashes: int, enable_repeat_insert: bool = True) -> "CountingBloomFilter":
        """
        Build a Counting Bloom filter form [u32].

        :param enable_repeat_insert:
        :param array: integer(u32) array
        :param hashes: hash function number of the Bloom filter
        :return:
        """
        py_bloom = PyCountingBloomFilter.from_int_array(array, hashes, enable_repeat_insert)
        return CountingBloomFilter(py_bloom)
