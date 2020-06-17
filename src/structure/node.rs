type Edge = (i64, isize);
#[derive(Debug)]
pub struct Node {
    pub payload: i64,
    pub neighbors:  Vec<Edge>,
}
impl Node {
    pub fn new(payload: i64) -> Node {
        Node {
            payload,
            neighbors: vec![],
        }
    }

    pub fn connect_to_node(&mut self, n: &Node, distance: isize) {
        let edge = (n.payload, distance);
        self.neighbors.push(edge);
    }

    pub fn has_neighbor(&self, other: &Node) -> bool {
       let elem = self.neighbors.iter().find(|(n,_)| *n == other.payload );
       match elem {
           None => false,
           Some(_) => true,
       }
    }

    pub fn is_neighbor_of(&self, other: &Node) -> bool {
        let elem = other.neighbors.iter().find(|(n,_)| *n == self.payload);
        match elem {
            None => false,
            Some(_) => true,
        }
    }


}
