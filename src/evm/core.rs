use super::{opcode::execute, stack::Stack};


pub fn run(bytes:&[u8]) {
    let mut pc = 0;
    let mut stack = Stack::new();
    while pc < bytes.len() {
        execute(bytes[pc], &mut stack);
        pc += 1;
    }
}