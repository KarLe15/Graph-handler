use serde::{Serialize, Deserialize};
use crate::structure::graph::Graph as DataGraph;

#[derive(Debug, Serialize, Deserialize)]
struct EdgeDescriptor {
    from: i64,
    to: i64,
    label: isize,
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeDescriptor {
    key: i64,
    color: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    edges: Vec<EdgeDescriptor>,
    nodes: Vec<NodeDescriptor>,
}

impl Graph {
    pub fn from_data(g: &DataGraph) -> Graph {
        let mut nodes = vec![];
        let mut edges = vec![];

        // nodes
        for (n, node) in g.nodes.iter() {
            nodes.push(NodeDescriptor {
                key: *n,
                color: "lightblue".to_string()
            });
            for (target, weight) in &node.neighbors {
                edges.push(EdgeDescriptor {
                    from: *n,
                    to: *target,
                    label: *weight,
                })
            }
        }
        Graph {
            nodes,
            edges,
        }
    }
}