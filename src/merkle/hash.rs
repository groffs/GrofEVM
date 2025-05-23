use sha2::{Digest, Sha256};

pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn encode(data: &[u8]) -> String {
    hex::encode(hash(data))
}