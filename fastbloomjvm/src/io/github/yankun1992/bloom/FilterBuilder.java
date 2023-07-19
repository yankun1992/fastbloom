/*
 * Copyright 2023 Yan Kun <yan_kun_1992@foxmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.github.yankun1992.bloom;

import io.github.otavia.jni.loader.NativeLoader;

import java.io.IOException;

public class FilterBuilder extends NativeLoader implements AutoCloseable {

    final long raw;

    /**
     * Constructs a new Bloom Filter Builder by specifying the expected size of the filter and the tolerable false
     * positive probability. The size of the BLoom filter in in bits and the optimal number of hash functions will
     * be inferred from this.
     *
     * @param expected_elements          expected size of the filter
     * @param false_positive_probability tolerable false positive probability
     */
    public FilterBuilder(long expected_elements, double false_positive_probability) throws IOException {
        this(open(expected_elements, false_positive_probability));
    }

    private FilterBuilder(long raw) throws IOException {
        super("fastbloom");
        this.raw = raw;
    }

    /**
     * Constructs a new Bloom Filter Builder by specifying the size of the bloom filter in bits and the number of
     * hashes. The expected size of the filter and the tolerable false positive probability will be inferred from this.
     *
     * @param size   size of the bloom filter in bits
     * @param hashes the number of hashes
     * @return FilterBuilder
     */
    public static FilterBuilder fromSizeAndHashes(long size, int hashes) throws IOException {
        long raw = fromSizeAndHashes0(size, hashes);

        return new FilterBuilder(raw);
    }

    /**
     * Use for CountingBloomFilter.
     */
    public void enableRepeatInsert(boolean enable) {
        enableRepeatInsert0(raw, enable);
    }

    /**
     * Constructs a Bloom filter using the specified parameters and computing missing parameters if
     * possible (e.g. the optimal Bloom filter bit size).
     *
     * @return BloomFilter
     */
    public BloomFilter buildBloomFilter() throws IOException {
        long pointer = buildBloomFilter0(raw);
        return new BloomFilter(pointer);
    }

    /**
     * Constructs a Counting Bloom filter using the specified parameters and computing missing parameters if
     * possible (e.g. the optimal Bloom filter bit size).
     *
     * @return CountingBloomFilter
     */
    public CountingBloomFilter buildCountingBloomFilter() throws IOException {
        long pointer = buildCountingBloomFilter0(raw);
        return new CountingBloomFilter(pointer);
    }

    /**
     * Checks whether a configuration is compatible to another configuration based on the size of the Bloom
     * filter and its hash functions.
     */
    public boolean isCompatibleTo(FilterBuilder builder) {
        return isCompatibleTo0(raw, builder.raw);
    }

    @Override
    public void close() throws Exception {
        close0(raw);
    }

    private static long open(long expected_elements, double false_positive_probability) throws IOException {
        NativeLoader.load("fastbloom");
        return new0(expected_elements, false_positive_probability);
    }

    private static native long new0(long expected_elements, double false_positive_probability);

    private static native long fromSizeAndHashes0(long size, int hashes);

    private static native void enableRepeatInsert0(long raw, boolean enable);

    private static native void complete0(long raw);

    private static native boolean isCompatibleTo0(long raw, long other);

    private static native void close0(long pointer);

    private static native long buildBloomFilter0(long raw);

    private static native long buildCountingBloomFilter0(long raw);


}
