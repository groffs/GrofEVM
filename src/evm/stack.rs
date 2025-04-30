

pub struct _Stack {
    data:Vec<u128>
}

impl _Stack {
    pub fn _new() -> Self {
        Self {data: Vec::with_capacity(1024)}
    }

    pub fn _push(&mut self, val: u128) {
        if self.data.len() >= 1024 {
            panic!("stack overflow");
        }
        self.data.push(val);
    }

    pub fn _pop(&mut self) {
        self.data.pop().expect("stack underflow");
    }
}