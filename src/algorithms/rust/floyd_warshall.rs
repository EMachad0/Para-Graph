use std::sync::atomic::{AtomicPtr, Ordering};
use itertools::Itertools;
use petgraph::prelude::*;
use rayon::prelude::*;

// use crate::bridge::ffi;
use crate::model::{Device, Transmission};

// pub fn floyd_warshall(topology: &UnGraph<Device, Transmission>) -> Vec<Vec<f64>> {
//     let n = topology.node_count();
//     let mut mat: Vec<f64> = vec![f64::INFINITY; n * n];
//     for i in 0..n {
//         mat[i * n + i] = 0.;
//     }
//     for e in topology.edge_references() {
//         let u = e.source().index();
//         let v = e.target().index();
//         let w = 1. / (e.weight().transmission_rate * 1_000_000_000.);
//         mat[u * n + v] = w;
//         mat[v * n + u] = w;
//     }
//     let res = ffi::floyd_warshall(mat);
//     let mat = res.chunks_exact(n).map(|v| v.to_vec()).collect_vec();
//     mat
// }

pub fn floyd_warshall(topology: &UnGraph<Device, Transmission>) -> Vec<Vec<f64>> {
    let n = topology.node_count();
    let mut mat: Vec<f64> = vec![f64::INFINITY; n * n];
    for i in 0..n {
        mat[i * n + i] = 0.;
    }
    for e in topology.edge_references() {
        let u = e.source().index();
        let v = e.target().index();
        let w = 1. / (e.weight().transmission_rate * 1_000_000_000.);
        mat[u * n + v] = w;
        mat[v * n + u] = w;
    }
    let res_ptr = AtomicPtr::new(mat.as_mut_ptr());
    for k in 0..n {
        let pairs: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .collect();

        pairs.into_par_iter().for_each(|(i, j)| {
            if j == k {
                return;
            }
            let ij = i * n + j;
            let ik = i * n + k;
            let kj = k * n + j;
            unsafe {
                let res = res_ptr.load(Ordering::Relaxed);
                *res.add(ij) = (*res.add(ij)).min(*res.add(ik) + *res.add(kj))
            }
        });
    }
    let mat = mat.chunks_exact(n).map(|v| v.to_vec()).collect_vec();
    mat
}
