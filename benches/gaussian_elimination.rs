use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use para_graph::algorithms::gaussian_elimination::{
    gaussian_elimination_par_cpu, gaussian_elimination_par_gpu, gaussian_elimination_serial,
};

const N: usize = 1_000;

fn get_a() -> Vec<Vec<f64>> {
    (0..N * N)
        .map(|x| {
            let i = x / N;
            let j = x % N;
            if i == j {
                x as f64 * 2. / 3.
            } else {
                x as f64
            }
        })
        .chunks(N)
        .into_iter()
        .map(|v| v.collect())
        .collect()
}

fn get_b() -> Vec<f64> {
    (0..N).map(|x| x as f64).collect()
}

fn bench_gaussian_elimination(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gaussian Elimination");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("gaussian elimination serial", |b| {
        b.iter(|| gaussian_elimination_serial(&get_a(), &get_b()))
    });
    group.bench_function("gaussian elimination par cpu", |b| {
        b.iter(|| gaussian_elimination_par_cpu(&get_a(), &get_b()))
    });
    group.bench_function("gaussian elimination par gpu", |b| {
        b.iter(|| gaussian_elimination_par_gpu(&get_a(), &get_b()))
    });
    group.finish();
}

criterion_group!(benches, bench_gaussian_elimination);
criterion_main!(benches);
