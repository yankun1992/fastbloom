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

/**
 * A Bloom filter is a space-efficient probabilistic data structure, conceived by Burton Howard Bloom in 1970, that is
 * used to test whether an element is a member of a set. False positive matches are possible, but false negatives
 * are not.
 * <br/>
 * <b>Reference<b/>: Bloom, B. H. (1970). Space/time trade-offs in hash coding with allowable errors. Communications of
 * the ACM, 13(7), 422-426. <a href="http://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf">Full text article</>
 */
public class BloomFilter extends NativeLoader implements AutoCloseable {

    final long raw;

    BloomFilter(long raw) throws IOException {
        super("fastbloom");
        this.raw = raw;
    }

    public int hashes() {
        return hashes0(raw);
    }

    /**
     * Add element to the filter.
     *
     * @param element value to add
     * @apiNote In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public void addInt(int element) {
        addInt0(raw, element);
    }

    public void addIntBatch(int[] array) {
        addIntBatch0(raw, array);
    }

    /**
     * Add element to the filter.
     *
     * @param element value to add
     * @apiNote In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public void addLong(long element) {
        addLong0(raw, element);
    }

    /**
     * Add element to the filter.
     *
     * @param element value to add
     */
    public void addStr(String element) {
        addStr0(raw, element);
    }

    /**
     * Add element to the filter.
     *
     * @param element value to add
     */
    public void addBytes(byte[] element) {
        addBytes0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * @param element to test
     * @return true if element is in this filter.
     * @apiNote In python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
     */
    public boolean containsInt(int element) {
        return containsInt0(raw, element);
    }

    /**
     * Tests whether an element is present in the filter (subject to the specified false positive rate).
     *
     * @param element to test
     * @return true if element is in this filter.
     * @apiNote In Python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
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
     * Removes all elements from the filter (i.e. resets all bits to zero).
     */
    public void clear() {
        clear0(raw);
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
     * Performs the union operation on two compatible bloom filters. This is achieved through a bitwise OR operation
     * on their bit vectors. This operations is lossless, i.e. no elements are lost and the bloom filter is the same
     * that would have resulted if all elements wer directly inserted in just one bloom filter.
     *
     * @param other the other bloom filter
     * @return false if not compatible
     */
    public boolean union(BloomFilter other) {
        return union0(raw, other.raw);
    }

    /**
     * Performs the intersection operation on two compatible bloom filters. This is achieved through a bitwise AND
     * operation on their bit vectors. The operations doesn't introduce any false negatives but it does raise the
     * false positive probability. The the false positive probability in the resulting Bloom filter is at most the
     * false-positive probability in one of the constituent bloom filters
     *
     * @param other the other bloom filter
     * @return false if not compatible
     */
    public boolean intersect(BloomFilter other) {
        return intersect0(raw, other.raw);
    }

    /**
     * @return true if the Bloom filter does not contain any elements
     */
    public boolean isEmpty() {
        return isEmpty0(raw);
    }

    @Override
    public void close() throws Exception {
        close0(raw);
    }


    public static BloomFilter fromBytes(byte[] array, int hashes) throws IOException {
        NativeLoader.load("fastbloom");
        long raw = fromBytes0(array, hashes);

        return new BloomFilter(raw);
    }

    private static native int hashes0(long raw);

    private static native void addInt0(long raw, int element);

    private static native void addIntBatch0(long raw, int[] array);

    private static native void addLong0(long raw, long element);

    private static native void addStr0(long raw, String element);

    private static native void addBytes0(long raw, byte[] element);

    private static native boolean containsInt0(long raw, int element);

    private static native boolean containsLong0(long raw, long element);

    private static native boolean containsStr0(long raw, String element);

    private static native boolean containsBytes0(long raw, byte[] element);

    private static native void clear0(long raw);

    private static native long fromBytes0(byte[] array, int hashes);

    private static native boolean union0(long raw, long other);

    private static native boolean intersect0(long raw, long other);

    private static native boolean isEmpty0(long raw);

    private static native void close0(long raw);

    private static native int getSize0(long raw);

    private static native void copyBytes0(long raw, byte[] bytes);


}
