# -*- coding: utf-8 -*-

# build script for local develop.

import os

_TEST_CONDA_ENV = os.environ['BLOOM_CONDA_ENV'] if 'BLOOM_CONDA_ENV' in os.environ else 'py37'


def develop():
    # maturin develop --release
    os.system("conda activate %s && maturin develop --release" % _TEST_CONDA_ENV)


def build():
    os.system("conda activate %s && maturin build --release")


if __name__ == '__main__':
    develop()
