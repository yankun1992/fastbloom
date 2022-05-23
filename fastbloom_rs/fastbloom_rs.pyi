# -*- coding: utf-8 -*-

from typing import Union


class PyFilterBuilder(object):
    def __int__(self, expected_elements: int, false_positive_probability: float):
        ...

    def build_bloom_filter(self) -> "PyBloomFilter":
        ...


class PyBloomFilter(object):
    def add(self, element: Union[str, int, bytes]):
        ...

    def add_int(self, element: int):
        ...

    def add_str(self, element: str):
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
