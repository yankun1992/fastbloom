#!/bin/bash

# easier debugging
pwd
ls -la

rustup override set stable
export RUSTFLAGS='-C target-feature=+fxsr,+sse,+sse2,+sse3,+ssse3,+sse4.1+sse4.2,+popcnt,+avx,+fma'

# first the default release
maturin publish \
  --skip-existing \
  --no-sdist \
  --username yankun