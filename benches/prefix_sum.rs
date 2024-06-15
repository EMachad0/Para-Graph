use criterion::{criterion_group, criterion_main, Criterion};
use para_graph::algorithms::prefix_sum::{pref_sum_par_cpu, pref_sum_par_gpu, pref_sum_serial};

const N: usize = 1_000_000;

fn get_arr() -> Vec<f64> {
    (0..N).map(|x| x as f64).collect()
}

fn bench_prefix_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Prefix Sum");
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));

    let input = get_arr();

    group.bench_with_input("Serial", &input, |b, arr| {
        let mut arr = arr.clone();
        b.iter(|| pref_sum_serial(&mut arr))
    });
    group.bench_with_input("CPU", &input, |b, arr| {
        let mut arr = arr.clone();
        b.iter(|| pref_sum_par_cpu(&mut arr))
    });
    group.bench_with_input("GPU", &input, |b, arr| {
        let mut arr = arr.clone();
        b.iter(|| pref_sum_par_gpu(&mut arr))
    });

    group.finish();
}

criterion_group!(benches, bench_prefix_sum);
criterion_main!(benches);
