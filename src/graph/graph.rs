use crate::graph::ops::Op;

pub type NodeId = usize;

pub struct Node {
    pub op: Op,
    pub inputs: Vec<NodeId>
}

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn add_node(&mut self, op: Op) {
        let id = self.nodes.len();
        self.nodes.push(Node { op, inputs } );
    }
}