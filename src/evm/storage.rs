use std::collections::HashMap;

use primitive_types::U256;


pub struct Estore {
    sto: HashMap<U256, U256>
}

impl Estore {
    pub fn new() -> Self {
        Self { sto: HashMap::new()}
    }

    pub fn store(&mut self, key : U256, val: U256){
        if val.is_zero() { 
            self.sto.remove(&key);
        } self.sto.insert(key, val);
    }

    pub fn fetch(&self, key: U256) -> U256 {
        *self.sto.get(&key).unwrap_or(&U256::zero())
    }

    pub fn dump(&self) -> &HashMap<U256, U256> {
        &self.sto
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evm0_storage() {
        let mut sto = Estore::new();
        let key = U256::from(0x01);
        let val = U256::from("0x0b");
        sto.store(key, val);
        assert_eq!(sto.fetch(key), val);
    }
}