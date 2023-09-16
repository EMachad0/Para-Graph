use petgraph::prelude::*;
use rand::distributions::{Bernoulli, Standard};
use rand::prelude::*;

pub fn random_graph_np<N, E, R>(rng: &mut R, n: usize, p: f64) -> UnGraph<N, E>
where
    Standard: Distribution<N> + Distribution<E>,
    R: Rng + ?Sized,
{
    let mut graph = UnGraph::with_capacity(n, 0);
    let mut nodes = Vec::with_capacity(n);
    for _ in 0..n {
        let node_weight: N = rng.gen();
        nodes.push(graph.add_node(node_weight));
    }

    let distribution = Bernoulli::new(p).unwrap();
    for i in 0..n {
        for j in i + 1..n {
            if distribution.sample(rng) {
                let edge_weight_1: E = rng.gen();
                let edge_weight_2: E = rng.gen();
                graph.add_edge(nodes[i], nodes[j], edge_weight_1);
                graph.add_edge(nodes[j], nodes[i], edge_weight_2);
            }
        }
    }

    graph
}
