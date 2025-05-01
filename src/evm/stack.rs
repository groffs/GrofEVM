use primitive_types::U256;

pub struct Stack {
    pub data : Vec<U256>
}

impl Stack {
    pub fn new() -> Self {
        Self {data: Vec::with_capacity(1024)}
    }

    pub fn push(&mut self, val : U256) {
        if self.data.len() >= 1024 {
            panic!();
        } self.data.push(val);
    }

    pub fn pop(&mut self) -> U256 {
        self.data.pop().expect("stack underflow")
    }

    pub fn dump(&self) -> &Vec<U256> {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        let stack_val = U256::from(0x01);
        let stack_val_two = U256::from(0x02);
        stack.push(stack_val); stack.push(stack_val_two);
        stack.pop(); assert_eq!(stack.dump(), &[stack_val]);
    }
}