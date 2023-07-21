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
import java.nio.ByteBuffer;

/**
 * A Counting Bloom filter works in a similar manner as a regular Bloom filter; however, it is able to keep track of
 * insertions and deletions. In a counting Bloom filter, each entry in the Bloom filter is a small counter associated
 * with a basic Bloom filter bit.
 * <br/>
 * <b>Reference</b>: F. Bonomi, M. Mitzenmacher, R. Panigrahy, S. Singh, and G. Varghese, "An Improved Construction
 * for Counting Bloom Filters," in 14th Annual European Symposium on Algorithms, LNCS 4168, 2006
 */
public class CountingBloomFilter extends NativeLoader implements AutoCloseable {

    long raw;

    CountingBloomFilter(long raw) throws IOException {
        super("fastbloom");
        this.raw = raw;
    }

    public int hashes() {
        return hashes0(raw);
    }

    /**
     * Add element to the filter.
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public void addInt(int element) {
        addInt0(raw, element);
    }

    /**
     * Remove element from this filter.
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public void removeInt(int element) {
        removeInt0(raw, element);
    }

    /**
     * Add element to the filter.
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     *
     * @param element value to add
     */
    public void addLong(long element) {
        addLong0(raw, element);
    }

    /**
     * Remove element from this filter.
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public void removeLong(long element) {
        removeLong0(raw, element);
    }

    /**
     * Add element to the filter.
     */
    public void addStr(String element) {
        addStr0(raw, element);
    }

    /**
     * Remove element from this filter.
     */
    public void removeStr(String element) {
        removeStr0(raw, element);
    }

    /**
     * Add element to the filter.
     */
    public void addBytes(byte[] element) {
        addBytes0(raw, element);
    }

    /**
     * Remove element from this filter.
     */
    public void removeBytes(byte[] element) {
        removeBytes0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     *
     * @param element to test
     * @return true if element is in this filter.
     */
    public boolean containsInt(int element) {
        return containsInt0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * <b>notice</b>: In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     *
     * @param element to test
     * @return true if element is in this filter.
     */
    public boolean containsLong(long element) {
        return containsLong0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * @param element to test
     * @return true if element is in this filter.
     */
    public boolean containsStr(String element) {
        return containsStr0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * @param element to test
     * @return true if element is in this filter.
     */
    public boolean containsBytes(byte[] element) {
        return containsBytes0(raw, element);
    }


    /**
     * Return the underlying byte array of the Bloom filter.
     */
    public byte[] getBytes() {
        int size = getSize0(raw);
        byte[] bytes = new byte[size];
        copyBytes0(raw, bytes);

        return bytes;
    }

    /**
     * Removes all elements from the filter (i.e. resets all bits to zero).
     */
    public void clear() {
        clear0(raw);
    }

    @Override
    public void close() throws Exception {
        close0(raw);
    }

    /**
     * Build a Counting Bloom filter form [u8].
     *
     * @param array                byte array
     * @param hashes               hash function number of the Bloom filter
     * @param enable_repeat_insert
     * @return CountingBloomFilter
     */
    public static CountingBloomFilter fromBytes(byte[] array, int hashes, boolean enable_repeat_insert) throws IOException {
        NativeLoader.load("fastbloom");
        long raw = fromBytes0(array, hashes, enable_repeat_insert);
        return new CountingBloomFilter(raw);
    }

    /**
     * Build a Counting Bloom filter form [u8].
     *
     * @param array  byte array
     * @param hashes hash function number of the Bloom filter
     * @return CountingBloomFilter
     */
    public static CountingBloomFilter fromBytes(byte[] array, int hashes) throws IOException {
        return fromBytes(array, hashes, true);
    }

    private static native int hashes0(long raw);

    private static native void addInt0(long raw, int element);

    private static native void removeInt0(long raw, int element);

    private static native void addLong0(long raw, long element);

    private static native void removeLong0(long raw, long element);

    private static native void addStr0(long raw, String element);

    private static native void removeStr0(long raw, String element);

    private static native void addBytes0(long raw, byte[] element);

    private static native void removeBytes0(long raw, byte[] element);

    private static native boolean containsInt0(long raw, int element);

    private static native boolean containsLong0(long raw, long element);

    private static native boolean containsStr0(long raw, String element);

    private static native boolean containsBytes0(long raw, byte[] element);

    private static native ByteBuffer getByteBuffer0(long raw);

    private static native int getSize0(long raw);

    private static native void copyBytes0(long raw, byte[] bytes);


    private static native void clear0(long raw);


    private static native void close0(long raw);

    private static native long fromBytes0(byte[] array, int hashes, boolean enable_repeat_insert);


}
