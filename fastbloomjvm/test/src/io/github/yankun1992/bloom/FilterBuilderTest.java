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

import org.junit.After;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

import java.io.IOException;

public class FilterBuilderTest {

    private FilterBuilder builder;
    private BloomFilter bloomFilter;
    private CountingBloomFilter countingBloomFilter;

    @Before
    public void create() throws IOException {
        builder = new FilterBuilder(10000000, 0.01);
        bloomFilter = builder.buildBloomFilter();
        countingBloomFilter = builder.buildCountingBloomFilter();
    }

    @After
    public void close() throws Exception {
        builder.close();
        bloomFilter.close();
        countingBloomFilter.close();
        builder = null;
        bloomFilter = null;
        countingBloomFilter = null;
    }

    @Test
    public void testBloom() {
        bloomFilter.addInt(1);
        Assert.assertTrue(bloomFilter.containsInt(1));
        Assert.assertFalse(bloomFilter.containsInt(2));

        bloomFilter.addStr("hello");
        Assert.assertTrue(bloomFilter.containsStr("hello"));
        Assert.assertFalse(bloomFilter.containsStr("world"));

        byte[] arr = {0, 1, 2, 5};
        byte[] arr2 = {0, 1, 2, 5, 6};
        bloomFilter.addBytes(arr);
        Assert.assertTrue(bloomFilter.containsBytes(arr));
        Assert.assertFalse(bloomFilter.containsBytes(arr2));
    }

    @Test
    public void testCounting() {
        countingBloomFilter.addInt(1);
        Assert.assertTrue(countingBloomFilter.containsInt(1));
        Assert.assertFalse(countingBloomFilter.containsInt(2));

        countingBloomFilter.addStr("hello");
        Assert.assertTrue(countingBloomFilter.containsStr("hello"));
        Assert.assertFalse(countingBloomFilter.containsStr("world"));

        byte[] arr = {0, 1, 2, 5};
        byte[] arr2 = {0, 1, 2, 5, 6};
        countingBloomFilter.addBytes(arr);
        Assert.assertTrue(countingBloomFilter.containsBytes(arr));
        Assert.assertFalse(countingBloomFilter.containsBytes(arr2));
    }

    @Test
    public void testRepeat() {
        try (FilterBuilder builder1 = FilterBuilder.fromSizeAndHashes(1000000, 7)) {
            builder1.enableRepeatInsert(true);
            try (CountingBloomFilter filter = builder1.buildCountingBloomFilter()) {
                filter.addStr("hello");
                filter.addStr("hello");
                filter.removeStr("hello");
                Assert.assertTrue(filter.containsStr("hello"));
                filter.removeStr("hello");
                Assert.assertFalse(filter.containsStr("hello"));
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }

        try (FilterBuilder builder1 = FilterBuilder.fromSizeAndHashes(1000000, 7)) {
            builder1.enableRepeatInsert(false);
            try (CountingBloomFilter filter = builder1.buildCountingBloomFilter()) {
                filter.addStr("hello");
                filter.addStr("hello");
                Assert.assertTrue(filter.containsStr("hello"));
                filter.removeStr("hello");
                Assert.assertFalse(filter.containsStr("hello"));
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }

    }
}
