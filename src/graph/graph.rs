use crate::graph::ops::Op;

pub type NodeId = usize;

pub struct Node {
    pub op: Op,
    pub inputs: Vec<NodeId>,
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn add_node(&mut self, op: Op, inputs: Vec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node { op, inputs });
        id
    }
}
