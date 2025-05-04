use primitive_types::U256;

use super::{opcode::{execute, Opcode}, stack::Stack};


pub fn run(bytes:&[u8], ecallgas: U256) {
    let mut pc = 0;

    /* for simulation purposes i'm passing the gas */
    /* here for now. we will get it via user accts */
    let mut stack = Stack::new();
    while pc < bytes.len() {
        let ops = bytes[pc];
        if let Some(op) = Opcode::from_u8(ops) {
            if let Err(e) = execute(op, &mut stack, ecallgas) {
                eprintln!("{:?}",e); break;
            }
        } else {
            eprintln!("unknown opcode 0x{:x} at pc {}", ops, pc);
            break;
        }
        pc += 1;
    }
}