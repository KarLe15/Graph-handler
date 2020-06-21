use crate::structure::node::Node;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i64, Node>,

}

impl Graph {
// TODO :: debug this constructor
    pub fn new(nodes: Vec<i64>, edges: Vec<(i64, i64)>) -> Graph {
        let nodes_map = RefCell::new(HashMap::new());
        for n in nodes {
            let node = Node::new(n);
            nodes_map.borrow_mut().insert(n,node);
        }
    // DEBUG :: in particular this for loop
        for (source,target) in &edges {
            nodes_map.borrow_mut().get_mut(source)
                .expect(format!("source node : {} not found !!!!", source).as_str())
                .connect_to_node(
                nodes_map.borrow_mut().get(target)
                    .expect(format!("target node : {} not found !!!!", target).as_str()),
                1
                );
        }
        Graph {
            nodes: nodes_map.into_inner()
        }
    }
}
