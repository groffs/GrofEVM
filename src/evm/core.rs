use primitive_types::U256;

use super::{opcode::execute, stack::Stack};


pub fn run(bytes:&[u8]) {
    let mut pc = 0;

    /* for simulation purposes i'm passing the gas */
    /* here for now. we will get it via user accts */
    let gas = U256::from(5);
    let mut stack = Stack::new();
    while pc < bytes.len() {
        let res = execute(bytes[pc], &mut stack, gas);
        res.unwrap();
        pc += 1;
    }
}