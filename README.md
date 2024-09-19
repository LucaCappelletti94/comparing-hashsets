# Comparing HashSets
A small rust project comparing the perfomance of different set implementations

The benchmark consists of inserting a large amount of u64 values sampled using a splitmix64 + xorshift64 random number generator, and measuring the time it takes to insert all the values.

## Running the benchmarks
The benchmark is a criterion benchmark, so you can run it using cargo. To run the benchmark, use the following command:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## Current results
The benchmark results, sorted by decreasing performance, are as follows:

| Collection                   | Time (ms)         |
|------------------------------|-------------------|
| `rustc_hash::FxHashSet`      | 1.3063 - 1.3076   |
| `ahash::AHashSet`            | 1.6001 - 1.6054   |
| `hashbrown::HashSet`         | 1.9176 - 1.9227   |
| `std::collections::HashSet`  | 4.3500 - 4.3569   |
| `std::collections::BTreeSet` | 8.1370 - 8.1452   |
| `Vec`                        | 1,109.2 - 1,109.5 |

## Would you like to add another HashSet implementation

By all means, we would love to see more implementations. Open up an issue to discuss it, and then submit a PR with your implementation. We will be happy to review it.