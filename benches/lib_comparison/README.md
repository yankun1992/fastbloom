# Bloom Filter Performance Benchmark

This benchmark compares the performance of different Python bloom filter implementations across various configurations. The benchmark was created with assistance from [aider.chat](https://github.com/Aider-AI/aider/).

## Supported Libraries

- **pyprobables** - Pure Python implementation
- **fastbloom-rs** - Rust-based implementation (this library)
- **pybloomfilter3** - C-based implementation with mmap support

## Features

- Benchmarks filter creation, item addition, and lookup operations
- Measures both hit and miss lookup performance
- Tracks memory usage and actual false positive rates
- Configurable test parameters (capacity, false positive rate, item counts)
- Statistical analysis with both mean and median timing results
- Comprehensive logging and result formatting
- Warmup runs to stabilize performance measurements

## Installation

Install the required dependencies:

```bash
pip install click loguru psutil pyprobables fastbloom-rs pybloomfilter3
```

Note: The benchmark will automatically skip any libraries that are not installed.

## Usage

### Basic Usage

Run with default parameters:

```bash
python lib_comparison.py
```

### Custom Configuration

```bash
python lib_comparison.py \
  --capacities "10000,100000,1000000" \
  --fpr "0.01,0.05,0.1" \
  --items "1000,10000,100000" \
  --lookups 10000 \
  --hit-ratio 0.7 \
  --runs 3 \
  --libraries "fastbloom-rs,pyprobables"
```

### Command Line Options

- `--capacities`: Comma-separated list of filter capacities to test
- `--fpr`: Comma-separated list of false positive rates
- `--items`: Comma-separated list of item counts to add to filters
- `--lookups`: Number of lookup operations to perform (default: 10000)
- `--hit-ratio`: Ratio of lookups that should be hits vs misses (0.0-1.0, default: 0.7)
- `--runs`: Number of benchmark runs for each test (default: 3)
- `--warmup`: Number of warmup runs (default: 1)
- `--libraries`: Libraries to test ("all" or comma-separated list, default: "all")
- `--seed`: Random seed for reproducible results (default: 42)
- `--verbose`, `-v`: Enable verbose logging
- `--log-file`: Custom log file path (default: auto-generated timestamp)

## Output

The benchmark produces:

1. **Detailed Results Table**: Shows performance metrics for each configuration
   - Creation time (seconds)
   - Add time (seconds) 
   - Lookup hit time (seconds)
   - Lookup miss time (seconds)
   - Memory usage (MB)
   - Actual false positive rate

2. **Summary Statistics**: Average and median performance across all tests for each library

3. **Log File**: Detailed execution log with configurable verbosity

## Metrics Explained

- **Creation Time**: Time to instantiate the bloom filter
- **Add Time**: Time to add all test items to the filter
- **Hit Time**: Time for lookups of items that are in the filter
- **Miss Time**: Time for lookups of items that are not in the filter
- **Memory Usage**: Approximate memory consumption in MB
- **FP Rate (actual)**: Measured false positive rate vs theoretical rate

## Performance Considerations

- The benchmark uses warmup runs to stabilize measurements
- Memory usage is measured at the process level and may include overhead
- Results can vary based on system load and hardware
- Multiple runs help identify performance variance

## Extending the Benchmark

To add support for additional bloom filter libraries:

1. Add import logic with availability checking
2. Extend `BloomFilterWrapper._create_filter()` method
3. Add appropriate method calls in `add()` and `contains()` methods
4. Update `get_library_version()` function

## Dependencies

- `click` - Command line interface
- `loguru` - Structured logging
- `psutil` - System and process utilities
- `pyprobables` - Pure Python bloom filters (optional)
- `fastbloom-rs` - Rust-based bloom filters (optional)
- `pybloomfilter3` - C-based bloom filters (optional)
