# fastbloom

A fast bloom filter implemented by Rust for Python!

## benchmark

### bloom add

<div>
<section class="plots">
                <table width="100%">
                    <tbody>
                        <tr>
                            <td>
                                <a href="./docs/img/bloom_add_test/pdf.svg">
                                    <img src="./docs/img/bloom_add_test/pdf_small.svg" alt="PDF of Slope" width="450" height="300" />
                                </a>
                            </td>
                            <td>
                                <a href="./docs/img/bloom_add_test/regression.svg">
                                    <img src="./docs/img/bloom_add_test/regression_small.svg" alt="Regression" width="450" height="300" />
                                </a>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </section>
<section class="stats">
                <div class="additional_stats">
                    <h4>Additional Statistics:</h4>
                    <table>
                        <thead>
                            <tr>
                                <th></th>
                                <th title="0.95 confidence level" class="ci-bound">Lower bound</th>
                                <th>Estimate</th>
                                <th title="0.95 confidence level" class="ci-bound">Upper bound</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>Slope</td>
                                <td class="ci-bound">41.095 ns</td>
                                <td>41.146 ns</td>
                                <td class="ci-bound">41.203 ns</td>
                            </tr>
                            <tr>
                                <td>R&#xb2;</td>
                                <td class="ci-bound">0.9959495</td>
                                <td>0.9961648</td>
                                <td class="ci-bound">0.9959083</td>
                            </tr>
                            <tr>
                                <td>Mean</td>
                                <td class="ci-bound">41.157 ns</td>
                                <td>41.207 ns</td>
                                <td class="ci-bound">41.257 ns</td>
                            </tr>
                            <tr>
                                <td title="Standard Deviation">Std. Dev.</td>
                                <td class="ci-bound">226.07 ps</td>
                                <td>261.59 ps</td>
                                <td class="ci-bound">294.01 ps</td>
                            </tr>
                            <tr>
                                <td>Median</td>
                                <td class="ci-bound">41.132 ns</td>
                                <td>41.184 ns</td>
                                <td class="ci-bound">41.247 ns</td>
                            </tr>
                            <tr>
                                <td title="Median Absolute Deviation">MAD</td>
                                <td class="ci-bound">201.13 ps</td>
                                <td>277.13 ps</td>
                                <td class="ci-bound">335.05 ps</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>
</div>

## Python

### requirements

```
Python >= 3.7
```

### setup

Install the latest fastbloom version with:

```bash
pip install fastbloom-rs
```

## Examples

for python

```python
from fastbloom_rs import BloomFilter

bloom = BloomFilter(100_000_000, 0.01)

bloom.add_str('hello')
bloom.add_bytes(b'world')
bloom.add_int(9527)

assert bloom.contains('hello')
assert bloom.contains(b'world')
assert bloom.contains(9527)

assert not bloom.contains('hello world')
```

for rust

```rust
use fastbloom_rs::{BloomFilter, FilterBuilder};

let mut bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();

bloom.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
assert_eq!(bloom.contains(b"helloworld!"), false);
```
