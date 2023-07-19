package io.github.yankun1992.bloom;

import org.junit.Assert;
import org.junit.Test;

import java.io.FileInputStream;
import java.io.IOException;

public class CrossLanguageTest {
    @Test
    public void testLoadBloomFilter() throws IOException {
        int size = 119816;
        int hashes = 7;
        FileInputStream bloomStream = new FileInputStream("data/bloom.bin");
        // FileInputStream bloomStream = new FileInputStream("../../../data/bloom.bin");
        byte[] array = new byte[size];
        int read = bloomStream.read(array);

        Assert.assertTrue(read == size);

        try (BloomFilter filter = BloomFilter.fromBytes(array, hashes)) {
            Assert.assertTrue(filter.containsBytes("hello".getBytes()));

            // In Python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
            Assert.assertTrue(filter.containsLong(87));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }

    }

    @Test
    public void testLoadCountingFilter() throws IOException {
        int size = 479264;
        int hashes = 7;
        FileInputStream stream = new FileInputStream("data/counting.bin");
        // FileInputStream stream = new FileInputStream("../../../data/counting.bin");
        byte[] array = new byte[size];
        int read = stream.read(array);

        Assert.assertTrue(read == size);

        try (CountingBloomFilter filter = CountingBloomFilter.fromBytes(array, hashes)) {
            Assert.assertTrue(filter.containsBytes("hello".getBytes()));
            // In Python API, `add_int` is same as `addLong` in java, because python `int` type is `i64` in Rust
            Assert.assertTrue(filter.containsLong(87));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
