use criterion::{criterion_group, criterion_main, Criterion};
use para_graph::algorithms::radix_sort::{radix_sort_par, radix_sort_serial};

const MAX: usize = 10_000_000;

fn test_vector() -> Vec<usize> {
    (0..MAX).rev().collect()
}

fn bench_radix_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radix Sort");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("radix sort serial", |b| {
        b.iter(|| radix_sort_serial(&mut test_vector()))
    });
    group.bench_function("radix sort par", |b| {
        b.iter(|| radix_sort_par(&mut test_vector()))
    });
    group.finish();
}

criterion_group!(benches, bench_radix_sort);
criterion_main!(benches);
