#!/usr/bin/env python3
"""
Bloom Filter Performance Benchmark

Benchmarks performance of different Python bloom filter implementations:
- pyprobables (pure Python)
- fastbloom-rs (Rust-based)
- pybloomfilter3 (C-based with mmap)

This script was created with assistance from aider.chat.
"""

import time
import random
import string
import gc
import psutil
import os
import statistics
from datetime import datetime
from typing import List, Dict, Any, Tuple, Protocol
from dataclasses import dataclass
from pathlib import Path

import click
from loguru import logger

# Import all bloom filter libraries
try:
    from probables import BloomFilter as PyProbablesBloomFilter

    PYPROBABLES_AVAILABLE = True
except ImportError:
    logger.warning("pyprobables not available")
    PYPROBABLES_AVAILABLE = False

try:
    from fastbloom_rs import BloomFilter as FastBloomFilter

    FASTBLOOM_AVAILABLE = True
except ImportError:
    logger.warning("fastbloom-rs not available")
    FASTBLOOM_AVAILABLE = False

try:
    import pybloomfilter

    PYBLOOMFILTER3_AVAILABLE = True
except ImportError:
    logger.warning("pybloomfilter3 not available")
    PYBLOOMFILTER3_AVAILABLE = False


class BloomFilterProtocol(Protocol):
    """Protocol defining common bloom filter interface for benchmarking."""

    def add(self, item: bytes) -> None:
        """Add an item to the bloom filter."""
        ...

    def contains(self, item: bytes) -> bool:
        """Check if an item is in the bloom filter."""
        ...


@dataclass
class BenchmarkConfig:
    """Configuration for bloom filter benchmarks."""

    # Bloom filter parameters to test
    capacities: List[int]
    false_positive_rates: List[float]

    # Test data parameters
    num_items_to_add: List[int]
    num_lookups: int
    lookup_hit_ratio: float  # Ratio of lookups that should be hits vs misses

    # Test parameters
    num_warmup_runs: int
    num_benchmark_runs: int

    # Data generation
    item_length: int
    random_seed: int


@dataclass
class BenchmarkResult:
    """Results from a single benchmark run."""

    library: str
    capacity: int
    false_positive_rate: float
    num_items: int

    # Timing results (in seconds) - averages
    creation_time: float
    add_time: float
    lookup_hit_time: float
    lookup_miss_time: float

    # Timing results (in seconds) - medians
    creation_time_median: float
    add_time_median: float
    lookup_hit_time_median: float
    lookup_miss_time_median: float

    # Memory usage (in MB)
    memory_usage: float

    # Additional metrics
    false_positive_count: int
    total_lookups: int


class BloomFilterWrapper:
    """Wrapper to provide common interface for different bloom filter implementations."""

    def __init__(self, library: str, capacity: int, false_positive_rate: float):
        self.library = library
        self.capacity = capacity
        self.false_positive_rate = false_positive_rate
        self._filter = self._create_filter()

    def _create_filter(self):
        """Create the appropriate bloom filter based on library."""
        if self.library == "pyprobables":
            if not PYPROBABLES_AVAILABLE:
                raise ImportError("pyprobables not available")
            return PyProbablesBloomFilter(
                est_elements=self.capacity, false_positive_rate=self.false_positive_rate
            )

        elif self.library == "fastbloom-rs":
            if not FASTBLOOM_AVAILABLE:
                raise ImportError("fastbloom-rs not available")
            return FastBloomFilter(self.capacity, self.false_positive_rate)

        elif self.library == "pybloomfilter3":
            if not PYBLOOMFILTER3_AVAILABLE:
                raise ImportError("pybloomfilter3 not available")
            return pybloomfilter.BloomFilter(self.capacity, self.false_positive_rate)

        else:
            raise ValueError(f"Unknown library: {self.library}")

    def add(self, item: bytes) -> None:
        """Add an item to the bloom filter."""
        if self.library == "pyprobables":
            self._filter.add(item.decode("utf-8"))
        elif self.library == "fastbloom-rs":
            self._filter.add_bytes(item)
        elif self.library == "pybloomfilter3":
            self._filter.add(item.decode("utf-8"))

    def contains(self, item: bytes) -> bool:
        """Check if an item is in the bloom filter."""
        if self.library == "pyprobables":
            return self._filter.check(item.decode("utf-8"))
        elif self.library == "fastbloom-rs":
            return self._filter.contains_bytes(item)
        elif self.library == "pybloomfilter3":
            return item.decode("utf-8") in self._filter

    def get_memory_usage(self) -> float:
        """Get approximate memory usage in MB."""
        # This is a rough approximation - actual implementation depends on the library
        process = psutil.Process(os.getpid())
        return process.memory_info().rss / 1024 / 1024


def generate_random_bytes(count: int, length: int, seed: int) -> List[bytes]:
    """Generate a list of random bytes for testing."""
    random.seed(seed)
    characters = string.ascii_letters + string.digits

    return [
        "".join(random.choices(characters, k=length)).encode("utf-8")
        for _ in range(count)
    ]


def time_operation(func, *args, **kwargs) -> Tuple[Any, float]:
    """Time an operation and return result and elapsed time."""
    start_time = time.perf_counter()
    result = func(*args, **kwargs)
    end_time = time.perf_counter()
    return result, end_time - start_time


def benchmark_bloom_filter(
    library: str,
    capacity: int,
    false_positive_rate: float,
    test_items: List[bytes],
    lookup_items: List[bytes],
    config: BenchmarkConfig,
) -> BenchmarkResult:
    """Benchmark a single bloom filter configuration."""

    logger.info(
        f"Benchmarking {library} - capacity: {capacity}, fpr: {false_positive_rate}"
    )

    # Measure memory before operations
    process = psutil.Process(os.getpid())
    initial_memory = process.memory_info().rss / 1024 / 1024

    # Warmup runs to stabilize performance
    for _ in range(config.num_warmup_runs):
        test_filter = BloomFilterWrapper(library, capacity, false_positive_rate)
        for item in test_items[:100]:  # Small subset for warmup
            test_filter.add(item)
        for item in lookup_items[:100]:
            test_filter.contains(item)
        del test_filter
        gc.collect()

    # Benchmark filter creation time separately
    creation_times = []
    for _ in range(config.num_benchmark_runs):
        _, creation_time = time_operation(
            lambda: BloomFilterWrapper(library, capacity, false_positive_rate)
        )
        creation_times.append(creation_time)

    avg_creation_time = sum(creation_times) / len(creation_times)
    median_creation_time = statistics.median(creation_times)

    # Benchmark adding items (create fresh filter for each run to avoid state issues)
    add_times = []
    final_filter = None

    for _ in range(config.num_benchmark_runs):
        # Create filter outside of timing
        test_filter = BloomFilterWrapper(library, capacity, false_positive_rate)

        # Time only the add operations
        _, add_time = time_operation(
            lambda: [test_filter.add(item) for item in test_items]
        )
        add_times.append(add_time)

        # Keep the last filter for lookup tests
        final_filter = test_filter

    avg_add_time = sum(add_times) / len(add_times)
    median_add_time = statistics.median(add_times)

    # Prepare lookup data - split into hits and misses based on config
    num_hits = int(config.num_lookups * config.lookup_hit_ratio)
    num_misses = config.num_lookups - num_hits

    hit_items = test_items[:num_hits]  # Items we know are in the filter
    miss_items = lookup_items[:num_misses]  # Items that should not be in the filter

    # Benchmark lookup hits using the populated filter
    hit_times = []
    for _ in range(config.num_benchmark_runs):
        _, hit_time = time_operation(
            lambda: [final_filter.contains(item) for item in hit_items]
        )
        hit_times.append(hit_time)

    avg_hit_time = sum(hit_times) / len(hit_times)
    median_hit_time = statistics.median(hit_times)

    # Benchmark lookup misses and count false positives
    miss_times = []
    false_positives = 0

    for _ in range(config.num_benchmark_runs):
        results, miss_time = time_operation(
            lambda: [final_filter.contains(item) for item in miss_items]
        )
        miss_times.append(miss_time)
        false_positives += sum(results)  # Count how many were incorrectly found

    avg_miss_time = sum(miss_times) / len(miss_times)
    median_miss_time = statistics.median(miss_times)
    avg_false_positives = false_positives // config.num_benchmark_runs

    # Measure final memory usage
    final_memory = process.memory_info().rss / 1024 / 1024
    memory_usage = final_memory - initial_memory

    return BenchmarkResult(
        library=library,
        capacity=capacity,
        false_positive_rate=false_positive_rate,
        num_items=len(test_items),
        creation_time=avg_creation_time,
        add_time=avg_add_time,
        lookup_hit_time=avg_hit_time,
        lookup_miss_time=avg_miss_time,
        creation_time_median=median_creation_time,
        add_time_median=median_add_time,
        lookup_hit_time_median=median_hit_time,
        lookup_miss_time_median=median_miss_time,
        memory_usage=memory_usage,
        false_positive_count=avg_false_positives,
        total_lookups=config.num_lookups,
    )


def get_available_libraries() -> List[str]:
    """Get list of available bloom filter libraries."""
    libraries = []
    if PYPROBABLES_AVAILABLE:
        libraries.append("pyprobables")
    if FASTBLOOM_AVAILABLE:
        libraries.append("fastbloom-rs")
    if PYBLOOMFILTER3_AVAILABLE:
        libraries.append("pybloomfilter3")
    return libraries


def print_benchmark_results(results: List[BenchmarkResult]) -> None:
    """Print formatted benchmark results."""

    if not results:
        logger.error("No benchmark results to display")
        return

    # Group results by configuration for easier comparison
    configs = {}
    for result in results:
        key = (result.capacity, result.false_positive_rate, result.num_items)
        if key not in configs:
            configs[key] = []
        configs[key].append(result)

    print("\n" + "=" * 100)
    print("BLOOM FILTER PERFORMANCE BENCHMARK RESULTS")
    print("=" * 100)

    for (capacity, fpr, num_items), config_results in configs.items():
        print(f"\nConfiguration: Capacity={capacity:,}, FPR={fpr}, Items={num_items:,}")
        print("-" * 110)

        # Table header
        print(
            f"{'Library':<15} {'Create Time':<12} {'Add Time':<12} {'Hit Time':<12} {'Miss Time':<12} {'Memory':<10} {'FP Rate':<10}"
        )
        print(
            f"{'':.<15} {'(seconds)':<12} {'(seconds)':<12} {'(seconds)':<12} {'(seconds)':<12} {'(MB)':<10} {'(actual)':<10}"
        )
        print("-" * 110)

        # Sort by total time (creation + add) for easy comparison
        config_results.sort(key=lambda x: x.creation_time + x.add_time)

        for result in config_results:
            actual_fpr = (
                result.false_positive_count / result.total_lookups
                if result.total_lookups > 0
                else 0
            )

            print(
                f"{result.library:<15} "
                f"{result.creation_time:<12.6f} "
                f"{result.add_time:<12.4f} "
                f"{result.lookup_hit_time:<12.6f} "
                f"{result.lookup_miss_time:<12.6f} "
                f"{result.memory_usage:<10.1f} "
                f"{actual_fpr:<10.4f}"
            )

    # Summary statistics
    print("\n" + "=" * 100)
    print("SUMMARY")
    print("=" * 100)

    for library in sorted(set(r.library for r in results)):
        lib_results = [r for r in results if r.library == library]
        if lib_results:
            avg_creation_time = sum(r.creation_time for r in lib_results) / len(
                lib_results
            )
            avg_add_time = sum(r.add_time for r in lib_results) / len(lib_results)
            avg_hit_time = sum(r.lookup_hit_time for r in lib_results) / len(
                lib_results
            )
            avg_miss_time = sum(r.lookup_miss_time for r in lib_results) / len(
                lib_results
            )
            avg_memory = sum(r.memory_usage for r in lib_results) / len(lib_results)

            median_creation_time = statistics.median(
                r.creation_time_median for r in lib_results
            )
            median_add_time = statistics.median(r.add_time_median for r in lib_results)
            median_hit_time = statistics.median(
                r.lookup_hit_time_median for r in lib_results
            )
            median_miss_time = statistics.median(
                r.lookup_miss_time_median for r in lib_results
            )
            median_memory = statistics.median(r.memory_usage for r in lib_results)

            print(f"\n{library}:")
            print(
                f"  Average creation time: {avg_creation_time:.6f}s (median: {median_creation_time:.6f}s)"
            )
            print(
                f"  Average add time: {avg_add_time:.4f}s (median: {median_add_time:.4f}s)"
            )
            print(
                f"  Average hit time: {avg_hit_time:.6f}s (median: {median_hit_time:.6f}s)"
            )
            print(
                f"  Average miss time: {avg_miss_time:.6f}s (median: {median_miss_time:.6f}s)"
            )
            print(
                f"  Average memory usage: {avg_memory:.1f}MB (median: {median_memory:.1f}MB)"
            )


@click.command()
@click.option(
    "--capacities",
    default="10000,100000,1000000",
    help="Comma-separated list of capacities to test",
)
@click.option(
    "--fpr",
    "--false-positive-rates",
    default="0.01,0.05,0.1",
    help="Comma-separated list of false positive rates",
)
@click.option(
    "--items",
    default="1000,10000,100000",
    help="Comma-separated list of item counts to add",
)
@click.option("--lookups", default=10000, help="Number of lookup operations to perform")
@click.option(
    "--hit-ratio", default=0.7, help="Ratio of lookups that should be hits (0.0-1.0)"
)
@click.option("--runs", default=3, help="Number of benchmark runs for each test")
@click.option("--warmup", default=1, help="Number of warmup runs")
@click.option(
    "--libraries",
    default="all",
    help='Comma-separated list of libraries to test (or "all")',
)
@click.option("--seed", default=42, help="Random seed for reproducible results")
@click.option("--verbose", "-v", is_flag=True, help="Enable verbose logging")
@click.option(
    "--log-file",
    default=None,
    help="Log file path (default: benchmark_YYYYMMDD_HHMMSS.log)",
)
def main(
    capacities: str,
    fpr: str,
    items: str,
    lookups: int,
    hit_ratio: float,
    runs: int,
    warmup: int,
    libraries: str,
    seed: int,
    verbose: bool,
    log_file: str,
) -> None:
    """
    Benchmark bloom filter implementations.

    This tool benchmarks the performance of different Python bloom filter libraries
    across various configurations. Results include timing for add/lookup operations,
    memory usage, and actual false positive rates.
    """

    # Configure logging
    logger.remove()
    log_level = "DEBUG" if verbose else "INFO"

    # Add console logging
    logger.add(lambda msg: click.echo(msg, err=True), level=log_level)

    # Add file logging with default filename based on timestamp
    if log_file is None:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        log_file = f"benchmark_{timestamp}.log"

    log_file_path = Path(log_file)
    logger.add(
        log_file_path,
        level=log_level,
        format="{time:YYYY-MM-DD HH:mm:ss} | {level} | {message}",
        rotation="10 MB",
        retention="7 days",
    )

    logger.info(f"Logging to file: {log_file_path.absolute()}")

    # Parse parameters
    try:
        capacity_list = [int(x.strip()) for x in capacities.split(",")]
        fpr_list = [float(x.strip()) for x in fpr.split(",")]
        items_list = [int(x.strip()) for x in items.split(",")]
    except ValueError as e:
        logger.error(f"Error parsing parameters: {e}")
        return

    # Determine which libraries to test
    available_libs = get_available_libraries()
    if not available_libs:
        logger.error("No bloom filter libraries are available!")
        logger.info("Install with: pip install pyprobables fastbloom-rs pybloomfilter3")
        return

    if libraries.lower() == "all":
        test_libraries = available_libs
    else:
        test_libraries = [lib.strip() for lib in libraries.split(",")]
        # Validate requested libraries are available
        for lib in test_libraries:
            if lib not in available_libs:
                logger.error(
                    f"Library '{lib}' is not available. Available: {available_libs}"
                )
                return

    logger.info(f"Testing libraries: {test_libraries}")
    logger.info(f"Capacities: {capacity_list}")
    logger.info(f"False positive rates: {fpr_list}")
    logger.info(f"Item counts: {items_list}")

    # Create benchmark configuration
    config = BenchmarkConfig(
        capacities=capacity_list,
        false_positive_rates=fpr_list,
        num_items_to_add=items_list,
        num_lookups=lookups,
        lookup_hit_ratio=hit_ratio,
        num_warmup_runs=warmup,
        num_benchmark_runs=runs,
        item_length=10,  # Length of generated test strings
        random_seed=seed,
    )

    # Generate test data
    max_items = max(items_list)
    logger.info(f"Generating {max_items * 2} test bytes...")

    # Generate items to add to filters
    test_items = generate_random_bytes(max_items, config.item_length, seed)

    # Generate separate items for lookup tests (to ensure we have items not in the filter)
    lookup_items = generate_random_bytes(
        config.num_lookups, config.item_length, seed + 1
    )

    # Run benchmarks
    all_results = []
    total_tests = (
        len(test_libraries) * len(capacity_list) * len(fpr_list) * len(items_list)
    )
    current_test = 0

    for library in test_libraries:
        for capacity in capacity_list:
            for false_positive_rate in fpr_list:
                for num_items in items_list:
                    current_test += 1
                    logger.info(f"Progress: {current_test}/{total_tests}")

                    try:
                        result = benchmark_bloom_filter(
                            library=library,
                            capacity=capacity,
                            false_positive_rate=false_positive_rate,
                            test_items=test_items[:num_items],
                            lookup_items=lookup_items,
                            config=config,
                        )
                        all_results.append(result)

                    except Exception as e:
                        logger.error(f"Error benchmarking {library}: {e}")
                        continue

    # Print results
    print_benchmark_results(all_results)

    logger.info("Benchmark complete!")


if __name__ == "__main__":
    main()
