use petgraph::graph::UnGraph;
use petgraph::prelude::EdgeRef;
use itertools::Itertools;

pub fn get_adj_matrix<N>(graph: &UnGraph<N, f64>) -> Vec<Vec<f64>> {
    let n = graph.node_count();
    let mut mat: Vec<f64> = vec![f64::INFINITY; n * n];
    for i in 0..n {
        mat[i * n + i] = 0.;
    }
    for e in graph.edge_references() {
        let u = e.source().index();
        let v = e.target().index();
        let w = *e.weight();
        mat[u * n + v] = w;
        mat[v * n + u] = w;
    }
    mat.chunks_exact(n).map(|v| v.to_vec()).collect_vec()
}
