use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::EdgeType;
use std::fmt::Display;
use std::process::Command;

pub fn to_dot<N, E, Ty>(name: &str, graph: &Graph<N, E, Ty>)
where
    N: Display,
    E: Display,
    Ty: EdgeType,
{
    let path = format!("out/{}.dot", name);
    let dot_graph = Dot::with_attr_getters(
        graph,
        &[Config::EdgeNoLabel, Config::NodeNoLabel],
        &|_g, e| format!("label = \"{}\"", e.weight()),
        &|_g, (i, u)| format!("label = \"{}: {}\"", i.index(), u),
    );
    std::fs::write(&path, format!("{}", dot_graph)).expect("Unable to write file");
    Command::new("dot")
        .args(["-Tpng", &path, "-O"])
        .output()
        .unwrap();
}
