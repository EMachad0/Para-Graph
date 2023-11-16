mod algorithms;
mod graph;
mod model;
mod bridge;
mod scheduler;

use petgraph::prelude::*;

use crate::scheduler::heft::heft;
use crate::graph::dot;
use crate::model::{Dependency, Device, Task, Transmission};

fn make_topology_graph() -> UnGraph<Device, Transmission> {
    let mut topology: UnGraph<Device, Transmission> = UnGraph::new_undirected();
    let iot_device = topology.add_node(Device::new(1, 1.));
    let edge_devices = {
        [
            Device::new(16, 3.),
            Device::new(32, 2.),
            Device::new(16, 3.),
        ]
        .map(|d| topology.add_node(d))
    };
    let cloud_devices = {
        [
            Device::new(64, 3.5),
            Device::new(128, 2.5),
            Device::new(64, 3.5),
        ]
        .map(|d| topology.add_node(d))
    };
    for edge_device in edge_devices {
        topology.add_edge(iot_device, edge_device, Transmission::new(0.1));
        for cloud_device in cloud_devices {
            topology.add_edge(edge_device, cloud_device, Transmission::new(1.));
        }
    }
    topology
}

fn make_tasks_graph() -> DiGraph<Task, Dependency> {
    // Node of topology graph
    let iot_device = NodeIndex::new(0);
    let cloud_device = NodeIndex::new(5);

    let mut tasks: DiGraph<Task, Dependency> = DiGraph::new();
    {
        let mut nodes = Vec::new();
        nodes.push(tasks.add_node(Task::empty().with_pin(iot_device)));
        for i in 1..5 {
            let data_size = 1_000_000_000 / i;
            let node = tasks.add_node(Task {
                data_size,
                processing_density: 1. * (i + 1) as f64,
                parallel_fraction: 0.5,
                pin: None,
            });
            let dep = *nodes.last().unwrap();
            tasks.add_edge(dep, node, Dependency::new(data_size));
            nodes.push(node);
        }
        let end_node = tasks.add_node(Task::empty().with_pin(cloud_device));
        let dep = *nodes.last().unwrap();
        tasks.add_edge(
            dep,
            end_node,
            Dependency::new(1_000_000_000 / nodes.len() as u64),
        );
        nodes.push(end_node);
    }
    {
        let starting_node = tasks.add_node(Task::empty().with_pin(iot_device));
        let end_node = tasks.add_node(Task::empty().with_pin(iot_device));
        (0..3).for_each(|i| {
            let input_data = 1_000_000;
            let output_data = 50_000;
            let node = tasks.add_node(Task {
                data_size: input_data,
                processing_density: 1. + i as f64,
                parallel_fraction: 0.5,
                pin: None,
            });
            tasks.add_edge(starting_node, node, Dependency::new(input_data));
            tasks.add_edge(node, end_node, Dependency::new(output_data));
        });
    }

    tasks
}

fn main() {
    let topology: UnGraph<Device, Transmission> = make_topology_graph();
    let tasks: DiGraph<Task, Dependency> = make_tasks_graph();

    let matching = heft(&topology, &tasks);
    matching.iter().enumerate().for_each(|(i, m)| {
        println!("{:2}: {}", i, m);
    });

    dot::to_dot("topology", &topology);
    dot::to_dot("tasks", &tasks);
}
