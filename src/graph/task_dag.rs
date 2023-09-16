use petgraph::prelude::*;
use rand::distributions::uniform::SampleRange;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::HashMap;

use crate::graph::random_dag_np::random_dag_np;

pub fn random_tasks_graph<N, E, R, A>(
    rng: &mut R,
    num_components: usize,
    n_range: A,
    p: f64,
) -> DiGraph<N, E>
where
    N: Clone,
    E: Clone,
    Standard: Distribution<N> + Distribution<E>,
    R: Rng + ?Sized,
    A: SampleRange<usize> + Clone,
{
    let mut tasks_graph = DiGraph::new();
    for _ in 0..num_components {
        let n = rng.gen_range(n_range.clone());
        let component: DiGraph<N, E> = random_dag_np(rng, n, p);

        let mut matching: HashMap<NodeIndex, NodeIndex> = HashMap::with_capacity(n);

        let old_indices: Vec<_> = component.node_indices().collect();
        for node_idx in old_indices.iter() {
            let node = component[*node_idx].clone();
            let new_node_idx = tasks_graph.add_node(node);
            matching.insert(*node_idx, new_node_idx);
        }

        for edge in component.raw_edges() {
            let source = matching[&edge.source()];
            let target = matching[&edge.target()];
            tasks_graph.add_edge(source, target, edge.weight.clone());
        }
    }
    tasks_graph
}
