use std::collections::HashMap;
use primitive_types::U256;

pub struct _Estorage{
    data: HashMap<U256, U256>
}

impl _Estorage { 
    pub fn _new() ->Self {
        Self {data: HashMap::new()}
    }

    pub fn _load(&self, key: U256) -> U256 {
        *self.data.get(&key).unwrap_or(&U256::zero())
    }

    pub fn _store(&mut self, key:U256, val:U256) {
        if val.is_zero(){
            self.data.remove(&key);
        } self.data.insert(key, val );
    }

    pub fn _dump(&self) -> &HashMap<U256, U256> {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_store_and_load(){
        let mut storage = _Estorage::_new();
        let key = U256::from(0x1);
        let val = U256::from(0xabc);
        storage._store(key, val);
        assert_eq!(storage._load(key), val);
    }

    #[test]
    fn test_storage_zero_deletes() {
        let mut storage = _Estorage::_new();
        let key = U256::from(0x02);
        storage._store(key, U256::from(0x233));
        storage._store(key, U256::zero());
        assert_eq!(storage._load(key), U256::zero());
    }
}