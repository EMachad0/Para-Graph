#![allow(dead_code)]

use itertools::Itertools;
use petgraph::prelude::*;
use rayon::prelude::*;

use crate::bridge::ffi;
use crate::graph::adj_matrix;
use crate::model::{Device, Transmission};

pub fn floyd_warshall(graph: &UnGraph<Device, Transmission>) -> Vec<Vec<f64>> {
    let n = graph.node_count();
    let graph = graph.map(|_, n| n, |_, e| 1. / (e.transmission_rate * 1_000_000_000.));
    let mat = adj_matrix::get_adj_matrix(&graph);
    floyd_warshall_gpu_par(n, &mat)
}

pub fn floyd_warshall_serial(n: usize, mat: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut mat = mat.to_vec().clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                mat[i][j] = mat[i][j].min(mat[i][k] + mat[k][j]);
            }
        }
    }
    mat
}

pub fn floyd_warshall_cpu_par(n: usize, mat: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut dist = mat.iter().flatten().cloned().collect_vec();
    for k in 0..n {
        dist.par_iter_mut().enumerate().for_each(|(idx, d)| {
            let i = idx / n;
            let j = idx % n;
            let new_dist = mat[i][k] + mat[k][j];
            *d = (*d).min(new_dist);
        });
    }
    dist.chunks_exact(n).map(|v| v.to_vec()).collect_vec()
}

pub fn floyd_warshall_gpu_par(n: usize, mat: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut mat = mat.iter().flatten().cloned().collect_vec();
    unsafe {
        ffi::floyd_warshall(n, &mut mat);
    }
    mat.chunks_exact(n).map(|v| v.to_vec()).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_matrix() -> Vec<Vec<f64>> {
        let mat = [[0., 1., 43.], [1., 0., 6.], [2., 1., 0.]];
        mat.iter().map(|v| v.to_vec()).collect_vec()
    }

    fn expected_matrix() -> Vec<Vec<f64>> {
        let mat = [[0., 1., 7.], [1., 0., 6.], [2., 1., 0.]];
        mat.iter().map(|v| v.to_vec()).collect_vec()
    }

    #[test]
    fn test_floyd_warshall_serial() {
        let res = floyd_warshall_serial(3, &test_matrix());
        assert_eq!(res, expected_matrix());
    }

    #[test]
    fn test_floyd_warshall_cpu_par() {
        let res = floyd_warshall_cpu_par(3, &test_matrix());
        assert_eq!(res, expected_matrix());
    }

    #[test]
    fn test_floyd_warshall_gpu_par() {
        let res = floyd_warshall_gpu_par(3, &test_matrix());
        assert_eq!(res, expected_matrix());
    }
}
