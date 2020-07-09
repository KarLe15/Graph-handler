use crate::structure::node::Node;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i64, Node>,
}

impl Graph {
// TODO :: debug this constructor
    pub fn new(nodes: Vec<i64>, edges: Vec<(i64, i64)>) -> Graph {
        let mut nodes_map = HashMap::new();
        for n in nodes {
            let node = Node::new(n);
            nodes_map.insert(n,node);
        }
        for (source,target) in &edges {
            if *source == *target {
               let node = nodes_map.get_mut(source)
                   .expect(format!("source node : {} not found !!!!", source).as_str());
                node.connect_to_node(node.payload, 1);
            } else {
                // this ugly fix make destroy the reference of
                {
                    let node_target = nodes_map.get(target)
                        .expect(format!("target node : {} not found !!!!", target).as_str());
                }
                let node_source = nodes_map.get_mut(source)
                    .expect(format!("source node : {} not found !!!!", source).as_str());

                node_source.connect_to_node(*target, 1);
            }
        }
        Graph {
            nodes: nodes_map
        }
    }
}
