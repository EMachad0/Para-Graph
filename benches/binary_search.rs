use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use num::BigUint;
use para_graph::algorithms::binary_search::{binary_search_par_cpu, binary_search_serial};

const MAX: u32 = 1_000;

fn factorial(n: u32) -> BigUint {
    (1..=n).product::<BigUint>()
}

fn bench_binary_search(c: &mut Criterion) {
    let queries = (1..=MAX).map(BigUint::from).collect_vec();
    // find the smallest mid such that factorial(mid) >= query^mid
    let eval = |mid: &u32, query: &BigUint| factorial(*mid).cmp(&query.pow(*mid));

    let mut group = c.benchmark_group("Binary Search");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("binary search serial", |b| {
        b.iter(|| binary_search_serial(eval, 1..MAX, &queries))
    });
    group.bench_function("binary search par", |b| {
        b.iter(|| binary_search_par_cpu(eval, 1..MAX, &queries))
    });
    group.finish();
}

criterion_group!(benches, bench_binary_search);
criterion_main!(benches);
