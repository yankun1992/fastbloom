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

import org.junit.Assert;
import org.junit.Test;

public class BloomFilterTest {

    @Test
    public void testBloomBuilder() {

        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);

                Assert.assertTrue(bloom.containsInt(87));
                Assert.assertTrue(bloom.containsBytes("hello".getBytes()));
                Assert.assertTrue(bloom.containsStr("hello"));

                Assert.assertFalse(bloom.containsBytes("hello world".getBytes()));

                try (BloomFilter bloom2 = BloomFilter.fromBytes(bloom.getBytes(), bloom.hashes())) {
                    Assert.assertTrue(bloom2.containsInt(87));
                    Assert.assertTrue(bloom2.containsBytes("hello".getBytes()));
                    Assert.assertTrue(bloom2.containsStr("hello"));

                    Assert.assertFalse(bloom2.containsBytes("hello world".getBytes()));
                }

            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    public void testBloomAdd() {
        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                for (int i = -1_000_000; i < 1_000_000; i++) {
                    bloom.addInt(i);
                }
                for (int i = -1_000_000; i < 1_000_000; i++) {
                    Assert.assertTrue(bloom.containsInt(i));
                }

                Assert.assertFalse(bloom.containsInt(1000_000_000));
                Assert.assertFalse(bloom.containsInt(-1000_000_000));
                Assert.assertFalse(bloom.containsStr("hello"));
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    public void testBloomOp() {
        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);

                Assert.assertTrue(bloom.containsInt(87));
                Assert.assertTrue(bloom.containsBytes("hello".getBytes()));
                Assert.assertTrue(bloom.containsStr("hello"));

                bloom.clear();

                Assert.assertFalse(bloom.containsInt(87));
                Assert.assertFalse(bloom.containsBytes("hello".getBytes()));
                Assert.assertFalse(bloom.containsStr("hello"));
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    public void testBloomUnion() {
        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                bloom.addStr("hello");
                Assert.assertTrue(bloom.containsStr("hello"));
                Assert.assertFalse(bloom.containsInt(87));

                try (BloomFilter bloom2 = builder.buildBloomFilter()) {
                    bloom2.addInt(87);
                    Assert.assertFalse(bloom2.containsStr("hello"));
                    Assert.assertTrue(bloom2.containsInt(87));

                    // UNION
                    bloom.union(bloom2);
                    Assert.assertTrue(bloom.containsStr("hello"));
                    Assert.assertTrue(bloom.containsInt(87));
                }
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    public void testBloomIntersect() {
        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);

                Assert.assertTrue(bloom.containsInt(87));
                Assert.assertTrue(bloom.containsBytes("hello".getBytes()));

                try (BloomFilter bloom2 = builder.buildBloomFilter()) {
                    bloom2.addInt(87);
                    Assert.assertFalse(bloom2.containsStr("hello"));
                    Assert.assertTrue(bloom2.containsInt(87));

                    // INTERSECT
                    bloom.intersect(bloom2);
                    Assert.assertTrue(bloom.containsInt(87));
                    Assert.assertFalse(bloom.containsBytes("hello".getBytes()));
                }
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Test
    public void testBloomBatch() {
        try (FilterBuilder builder = new FilterBuilder(100000000, 0.01)) {
            try (BloomFilter bloom = builder.buildBloomFilter()) {
                int[] insert = new int[100_000];
                for (int i = 0; i < insert.length; i++) {
                    insert[i] = i;
                }
                bloom.addIntBatch(insert);
                for (int i = 0; i < insert.length; i++) {
                    Assert.assertTrue(bloom.containsInt(i));
                }
                Assert.assertFalse(bloom.containsInt(100_001));
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
