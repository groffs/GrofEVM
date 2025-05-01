use super::stack::Stack;


pub enum EvmOps{
    OpAdd = 0x60,
}

pub fn execute(opcode : EvmOps, stack: &mut Stack){
    match opcode {
        EvmOps::OpAdd => {
            let dst = stack.pop();
            let src = stack.pop();
            stack.push(dst + src);
        },
        _ => panic!("bad opcode")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let eops = EvmOps::OpAdd;
        let mut stack = Stack::new();
        execute(eops, &mut stack);
    }
}