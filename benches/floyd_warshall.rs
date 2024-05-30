use criterion::{criterion_group, criterion_main, Criterion};
use para_graph::algorithms::floyd_warshall::{
    floyd_warshall_par_cpu, floyd_warshall_par_gpu, floyd_warshall_serial,
};

const N: usize = 1000;

fn full_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut mat = vec![vec![f64::INFINITY; n]; n];
    for (i, row) in mat.iter_mut().enumerate() {
        row[i] = 0.;
    }
    mat
}

fn bench_floyd_warshall(c: &mut Criterion) {
    let mut group = c.benchmark_group("Floyd Warshall");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("floyd warshall serial", |b| {
        b.iter(|| floyd_warshall_serial(N, &full_matrix(N)))
    });
    group.bench_function("floyd warshall cpu par", |b| {
        b.iter(|| floyd_warshall_par_cpu(N, &full_matrix(N)))
    });
    group.bench_function("floyd warshall gpu par", |b| {
        b.iter(|| floyd_warshall_par_gpu(N, &full_matrix(N)))
    });
    group.finish();
}

criterion_group!(benches, bench_floyd_warshall);
criterion_main!(benches);
