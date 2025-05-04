use primitive_types::U256;

use super::stack::Stack;

#[derive(Debug)]
pub enum EvmError {
    OutOfGas,
    StackUnderflow
}

pub type EvmResult<T> = Result<T, EvmError>;


pub fn execute(opcode : u8, stack: &mut Stack, gas: U256) -> EvmResult<()>{
    
    if gas < U256::from(3) { 
        return Err(EvmError::OutOfGas);
    }

    match opcode {
        0x01 => {
            let stack_top = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_bot = stack.pop().ok_or(EvmError::StackUnderflow);
            stack.push(stack_top.unwrap() + stack_bot.unwrap());
            Ok(())
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
        let gas = U256::from(4);
        let mut stack = Stack::new();
        execute(eops, &mut stack, gas);
    }
}