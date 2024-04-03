use criterion::{criterion_group, criterion_main, Criterion};
use framework_rs::algorithms::floyd_warshall::{
    floyd_warshall_cpu_par, floyd_warshall_gpu_par, floyd_warshall_serial,
};

const N: usize = 1000;

fn full_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut mat = vec![vec![f64::INFINITY; n]; n];
    for i in 0..n {
        mat[i][i] = 0.;
    }
    mat
}

fn bench_floyd_warshall(c: &mut Criterion) {
    let mut group = c.benchmark_group("Floyd Warshall");
    group.bench_function("floyd warshall serial", |b| {
        b.iter(|| floyd_warshall_serial(N, &full_matrix(N)))
    });
    group.bench_function("floyd warshall cpu par", |b| {
        b.iter(|| floyd_warshall_cpu_par(N, &full_matrix(N)))
    });
    group.bench_function("floyd warshall gpu par", |b| {
        b.iter(|| floyd_warshall_gpu_par(N, &full_matrix(N)))
    });
    group.finish();
}

criterion_group!(benches, bench_floyd_warshall);
criterion_main!(benches);
