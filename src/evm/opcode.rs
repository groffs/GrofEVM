use super::stack::_Stack;

pub fn _execute(opcode: u8, _stack:&mut _Stack) {
    match opcode {
        0x01 => {},
        _ => panic!("bad opcode")
    }
}