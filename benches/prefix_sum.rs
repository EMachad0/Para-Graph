use criterion::{criterion_group, criterion_main, Criterion};
use para_graph::algorithms::prefix_sum::{pref_sum_par_cpu, pref_sum_par_gpu, pref_sum_serial};

const N: usize = 100_000_000;

fn get_arr() -> Vec<f64> {
    (0..N).map(|x| x as f64).collect()
}

fn bench_prefix_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Prefix Sum");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("prefix sum serial", |b| {
        b.iter(|| pref_sum_serial(&mut get_arr()))
    });
    group.bench_function("prefix sum par cpu", |b| {
        b.iter(|| pref_sum_par_cpu(&mut get_arr()))
    });
    group.bench_function("prefix sum par gpu", |b| {
        b.iter(|| pref_sum_par_gpu(&mut get_arr()))
    });
    group.finish();
}

criterion_group!(benches, bench_prefix_sum);
criterion_main!(benches);
