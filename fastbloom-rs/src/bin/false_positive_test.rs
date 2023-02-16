use fastbloom_rs::{FilterBuilder, Membership};

fn main() {
    let false_positive_probability = 0.01;
    let excepts = vec![100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
    for except in excepts {
        let mut builder =
            FilterBuilder::new(except, false_positive_probability);
        let mut bloom = builder.build_bloom_filter();

        for x in 1..except {
            bloom.add(&u64::to_le_bytes(x));
        }

        let mut error = 0;
        for x in 1..except {
            let check = x + except;
            if bloom.contains(&u64::to_le_bytes(check)) {
                error += 1;
            }
        }

        println!("[{:?}]\terror count is {} except is {}", bloom.config(),
                 error, except as f64 * false_positive_probability);
    }
}