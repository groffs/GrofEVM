use super::stack::_Stack;

pub fn _execute(opcode: u8, stack:&mut _Stack) {
    match opcode {
        0x01 => {
            let f = stack._pop();
            let s = stack._pop();
            stack._push(f + s);
        },
        0x02 => {
            let sa = stack._pop();
            let sb = stack._pop();
            stack._push(sa * sb);
        },
        0x03 => {
            let sa = stack._pop();
            let sb =stack._pop();
            stack._push(sa - sb);
        },
        _ => panic!("bad opcode")
    }
}