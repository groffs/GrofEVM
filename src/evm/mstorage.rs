use super::hash::hash;

#[derive(Debug, Clone)]
pub struct Emsnode {
    pub hash:Vec<u8>,
    pub left: Option<Box<Emsnode>>,
    pub right: Option<Box<Emsnode>>
}

impl Emsnode {
    pub fn new(data: &[u8]) -> Self {
        Self { hash: hash(data), left: None, right: None }
    }

    pub fn parent(left: Emsnode, right:Emsnode) -> Self {
        let combine = [left.hash.as_slice(), right.hash.as_slice()].concat();
        Self { hash: hash(&combine), left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }
}

pub struct Emstree {
    pub root: Emsnode,
    pub leaves: Vec<Emsnode>
}

impl Emstree {
    pub fn build(data:Vec<&[u8]>) -> Self {
        let leaves:Vec<Emsnode> = data.into_iter().map(Emsnode::new).collect();
        let mut nodes = leaves.clone();
        while nodes.len() > 1 {
            let mut next_lvl = Vec::new();
            for i in (0..nodes.len()).step_by(2) {
                let left = nodes[i].clone();
                let right = if i + 1 < nodes.len() { nodes[i+1].clone()} else {left.clone()};
                next_lvl.push(Emsnode::parent(left, right));
            }
            nodes = next_lvl;
        }
        let root = nodes[0].clone();
        Self { root: root, leaves: leaves }
    }

    pub fn root_hash(&self) -> Vec<u8> {
        self.root.hash.clone()
    }
}