# -*- coding: utf-8 -*-

import os

_TEST_CONDA_ENV = "py37"


def develop():
    # maturin develop --release
    os.system("conda activate %s && maturin develop --release" % _TEST_CONDA_ENV)


def build():
    os.system("conda activate %s && maturin build --release")


if __name__ == '__main__':
    develop()
