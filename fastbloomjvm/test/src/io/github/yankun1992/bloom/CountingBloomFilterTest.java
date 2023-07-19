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

public class CountingBloomFilterTest {

    @Test
    public void testBuilder() throws Exception {
        try (FilterBuilder builder = new FilterBuilder(100_000_000, 0.01)) {
            builder.enableRepeatInsert(false);
            try (CountingBloomFilter bloom = builder.buildCountingBloomFilter()) {
                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);

                Assert.assertTrue(bloom.containsInt(87));
                Assert.assertTrue(bloom.containsBytes("hello".getBytes()));
                Assert.assertTrue(bloom.containsStr("hello"));

                Assert.assertFalse(bloom.containsBytes("hello world".getBytes()));

                try (CountingBloomFilter bloom2 = CountingBloomFilter.fromBytes(bloom.getBytes(), bloom.hashes(), false)) {
                    Assert.assertTrue(bloom2.containsInt(87));
                    Assert.assertTrue(bloom2.containsBytes("hello".getBytes()));
                    Assert.assertTrue(bloom2.containsStr("hello"));

                    Assert.assertFalse(bloom2.containsBytes("hello world".getBytes()));
                }
            }
        }
    }

    @Test
    public void testFilter() throws Exception {
        try (FilterBuilder builder = new FilterBuilder(100_000_000, 0.01)) {
            builder.enableRepeatInsert(false);
            try (CountingBloomFilter bloom = builder.buildCountingBloomFilter()) {
                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);
                bloom.addStr("world");
                bloom.addLong(88);

                Assert.assertTrue(bloom.containsBytes("hello".getBytes()));
                Assert.assertTrue(bloom.containsInt(87));
                Assert.assertTrue(bloom.containsStr("world"));
                Assert.assertTrue(bloom.containsLong(88));

                Assert.assertFalse(bloom.containsInt(88));
                Assert.assertFalse(bloom.containsLong(87));

                bloom.removeBytes("hello".getBytes());
                bloom.removeInt(87);
                bloom.removeStr("world");
                bloom.removeLong(88);

                Assert.assertFalse(bloom.containsBytes("hello".getBytes()));
                Assert.assertFalse(bloom.containsInt(87));
                Assert.assertFalse(bloom.containsStr("world"));
                Assert.assertFalse(bloom.containsLong(88));

                bloom.addBytes("hello".getBytes());
                bloom.addInt(87);
                bloom.addStr("world");
                bloom.addLong(88);
                bloom.clear();
                Assert.assertFalse(bloom.containsBytes("hello".getBytes()));
                Assert.assertFalse(bloom.containsInt(87));
                Assert.assertFalse(bloom.containsStr("world"));
                Assert.assertFalse(bloom.containsLong(88));

            }
        }
    }

}
