use petgraph::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;

pub fn random_dag_np<N, E, R>(rng: &mut R, n: usize, p: f64) -> DiGraph<N, E>
where
    Standard: Distribution<N>,
    Standard: Distribution<E>,
    R: Rng + ?Sized,
{
    let mut graph = DiGraph::new();
    let mut indices = Vec::new();

    for _ in 0..n {
        let node_weight: N = rng.gen();
        indices.push(graph.add_node(node_weight));
    }

    for i in 1..n {
        let parent = rng.gen_range(0..i);
        let edge_weight: E = rng.gen();
        graph.add_edge(indices[parent], indices[i], edge_weight);
        for j in 0..i {
            if j == parent {
                continue;
            }
            if rng.gen_bool(p) {
                let edge_weight: E = rng.gen();
                graph.add_edge(indices[j], indices[i], edge_weight);
            }
        }
    }

    graph
}
