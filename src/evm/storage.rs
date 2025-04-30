use std::collections::HashMap;
use primitive_types::U256;

pub struct _Ememory{
    data: HashMap<U256, U256>
}

impl _Ememory { 
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