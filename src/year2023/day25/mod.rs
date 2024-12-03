use std::collections::HashMap;

use petgraph::{
    graph::{EdgeReference, UnGraph},
    stable_graph::NodeIndex,
    visit::EdgeRef,
};

pub mod task1 {
    use super::find_three_cut;

    pub fn ans() -> u128 {
        find_three_cut("resources/2023/day25/input")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        // Push the button:
        0
    }
}

fn find_three_cut(file: &str) -> u128 {
    // Finds a cut of 3 edges which bisect the graph,
    // and returns the product of the number of nodes in each subnet

    // Do this by picking a random node as start of a subset
    // Then, add the subnet's neighbour which does the most to reduce the number of
    // edges leaving the subnet. Repeat until the subnet has only 3 edges leaving it.

    let graph = parse_file(file);

    let mut subnet: Vec<NodeIndex> = vec![];

    let start_node = graph.node_indices().next().unwrap();
    subnet.push(start_node);

    loop {
        let outward_subnet_edges_count = subnet
            .iter()
            .flat_map(|node| {
                graph.edges(*node).filter(|edge| {
                    !subnet.contains(&edge.target()) || !subnet.contains(&edge.source())
                })
            })
            .count();

        if outward_subnet_edges_count == 3 {
            break;
        }

        let mut subnet_neighbours: Vec<NodeIndex> = graph
            .node_indices()
            .filter(|node| {
                subnet.iter().any(|subnet_node| {
                    graph.contains_edge(*subnet_node, *node)
                        || graph.contains_edge(*node, *subnet_node)
                })
            })
            .filter(|node| !subnet.contains(node))
            .collect();

        if subnet_neighbours.is_empty() {
            panic!("No neighbours found");
        }

        subnet_neighbours.sort_by_key(|node| {
            let node_edges: Vec<EdgeReference<()>> = graph.edges(*node).collect();
            let edges_count = node_edges.len();

            // partition node's edges into those which are into the subnet, and those which are not

            let subnet_edges = node_edges
                .into_iter()
                .filter(|edge| subnet.contains(&edge.source()) || subnet.contains(&edge.target()))
                .count();

            edges_count - subnet_edges
        });

        let next_node = subnet_neighbours.swap_remove(0);
        subnet.push(next_node);
    }

    // multiply the size of the subnets
    let graph_nodes = graph.node_count();
    let subnet_nodes = subnet.len();
    let other_nodes = graph_nodes - subnet_nodes;

    subnet_nodes as u128 * other_nodes as u128
}

type Network = UnGraph<String, ()>;
fn parse_file(file: &str) -> Network {
    let contents = std::fs::read_to_string(file).unwrap();

    let mut graph = UnGraph::<String, ()>::new_undirected();

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    contents.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let host = parts.next().unwrap().to_string();
        let connections = parts.next().unwrap().split(' ');

        let host_node = if let Some(node) = nodes.get(&host) {
            *node
        } else {
            let node = graph.add_node(host.clone());
            nodes.insert(host.clone(), node);
            node
        };

        connections.for_each(|connection| {
            let connection = connection.to_string();
            let connection_node = if let Some(node) = nodes.get(&connection) {
                *node
            } else {
                let node = graph.add_node(connection.clone());
                nodes.insert(connection.clone(), node);
                node
            };
            graph.add_edge(host_node, connection_node, ());
        });
    });

    graph
}

#[cfg(test)]
mod tests {
    // use petgraph::dot::{Config, Dot};

    use super::*;

    // #[test]
    // fn test_parse_file() {
    //     let graph = parse_file("resources/2023/day25/test_input");

    //     // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    //     // write graphviz to a file
    //     std::fs::write(
    //         "resources/2023/day25/graph.dot",
    //         format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])),
    //     )
    //     .unwrap();
    // }

    #[test]
    fn test_find_three_cut() {
        let result = find_three_cut("resources/2023/day25/test_input");
        assert_eq!(result, 54);
    }
}
