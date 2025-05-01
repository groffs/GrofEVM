use super::stack::Stack;

pub fn execute(opcode : u8, stack: &mut Stack){
    match opcode {
        0x60 => {
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
        let eops = 0x01;
        let mut stack = Stack::new();
        execute(eops, &mut stack);
    }
}