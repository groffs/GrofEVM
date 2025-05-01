use super::hash::hash;

#[derive(Debug,Clone)]
pub struct EmerkNode {
    pub hash: Vec<u8>,
    pub left: Option<Box<EmerkNode>>,
    pub right: Option<Box<EmerkNode>>
}

impl EmerkNode {
    pub fn new(data: &[u8]) -> Self {
        Self{hash:hash(data), left:None, right: None}
    }

    pub fn parent(left:EmerkNode, right: EmerkNode) -> Self {
        let combine = [left.hash.as_slice(), right.hash.as_slice()].concat();
        Self { hash: hash(&combine), left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }
}

pub struct EmerkTree {
    pub root: EmerkNode,
    pub leaves: Vec<EmerkNode>
}

impl EmerkTree {
    pub fn build(data: Vec<&[u8]>) -> Self{
        let leaves : Vec<EmerkNode> = data.into_iter().map(EmerkNode::new).collect();
        let mut node = leaves.clone();
        while node.len() > 1 {
            let mut next = Vec::new();
            for i in (0..node.len()).step_by(2) {
                let  left = node[i].clone();
                let  right = if i + 1 < node.len() {
                    node[i+1].clone()
                } else { left.clone()};
                next.push(EmerkNode::parent(left, right));
            }
            node = next;
        }
        let root = node[0].clone();
        EmerkTree { root, leaves }
    }

    pub fn root_hash(&self) -> Vec<u8>{
        self.root.hash.clone()
    }
}