//! Criterion benchmark to compare the speed of different HashSet implementations.
use ahash::AHashSet;
use comparing_hashsets::*;
use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashSet as HashBrownHashSet;
use rustc_hash::FxHashSet;
use std::collections::{BTreeSet, HashSet};

fn bench_hashset<H: Set<u64>>(c: &mut Criterion) {
    let random_state = 6542378476216389_u64;

    c.bench_function(&format!("insert_value {}", H::name()), |b| {
        b.iter(|| {
            let mut set = H::create_new();
            let mut random_state = splitmix64(random_state);
            for _ in 0..100_000 {
                random_state = splitmix64(random_state);
                let value = xorshift64(random_state);
                set.insert_value(value);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_hashset::<HashSet<u64>>,
    bench_hashset::<HashBrownHashSet<u64>>,
    bench_hashset::<FxHashSet<u64>>,
    bench_hashset::<AHashSet<u64>>,
    bench_hashset::<Vec<u64>>,
    bench_hashset::<BTreeSet<u64>>,
);

criterion_main!(benches);
