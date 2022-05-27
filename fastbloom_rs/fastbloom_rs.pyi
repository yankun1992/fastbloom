# -*- coding: utf-8 -*-

from typing import Union, Sequence


class PyFilterBuilder(object):
    def __int__(self, expected_elements: int, false_positive_probability: float):
        ...

    def build_bloom_filter(self) -> "PyBloomFilter":
        ...

    def expected_elements(self) -> int:
        ...

    def false_positive_probability(self) -> float:
        ...

    def size(self) -> int:
        ...

    def hashes(self) -> int:
        ...


class PyBloomFilter(object):
    def add(self, element: Union[str, int, bytes]):
        ...

    def add_int(self, element: int):
        ...

    def add_int_batch(self, array: Sequence[int]):
        ...

    def add_str(self, element: str):
        ...

    def add_str_batch(self, array: Sequence[str]):
        ...

    def add_bytes(self, element: bytes):
        ...

    def contains(self, element: Union[str, int, bytes]) -> bool:
        ...

    def contains_int(self, element: int) -> bool:
        ...

    def contains_str(self, element: str) -> bool:
        ...

    def contains_bytes(self, element: bytes) -> bool:
        ...

    def config(self) -> PyFilterBuilder:
        ...

    def hashes(self) -> int:
        ...

    def get_bytes(self) -> bytes:
        ...

    def get_int_array(self) -> Sequence[int]:
        ...

    def clear(self):
        ...

    def is_empty(self) -> bool:
        ...

    def union(self, other: PyBloomFilter) -> bool:
        ...

    def intersect(self, other: PyBloomFilter) -> bool:
        ...

    @staticmethod
    def from_bytes(array: bytes, hashes: int) -> PyBloomFilter:
        ...

    @staticmethod
    def from_int_array(array: Sequence[int], hashes: int) -> PyBloomFilter:
        ...
