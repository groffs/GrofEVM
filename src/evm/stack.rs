use primitive_types::U256;

pub struct _Stack {
    data:Vec<U256>
}

impl _Stack {
    pub fn _new() -> Self {
        Self {data: Vec::with_capacity(1024)}
    }

    pub fn _push(&mut self, val: U256) {
        if self.data.len() >= 1024 {
            panic!("stack overflow");
        }
        self.data.push(val);
    }

    pub fn _pop(&mut self) -> U256 {
        self.data.pop().expect("stack underflow")
    }

    pub fn _dump(&self) -> &Vec<U256>{
        &self.data
    }   
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_stack_addition_load() {
        let mut stack = _Stack::_new();
        let val = U256::from(0x02);
        stack._push(val);
        assert_eq!(stack._dump(), &vec![val]);
    }
}