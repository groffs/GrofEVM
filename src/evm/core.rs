use super::{opcode::_execute, stack::_Stack};

pub fn _run(bytecode: &[u8]) {
    let mut pc = 0;
    let mut stack = _Stack::_new();
    while pc < bytecode.len(){
        _execute(bytecode[pc], &mut stack);
        pc += 1
    }
}