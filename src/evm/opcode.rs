use primitive_types::U256;

use super::stack::Stack;

#[derive(Debug)]
pub enum EvmError {
    OutOfGas,
    StackUnderflow
}

pub type EvmResult<T> = Result<T, EvmError>;


pub fn execute(opcode : u8, stack: &mut Stack, gas: U256) -> EvmResult<()>{
    match opcode {
        0x01 => {
            /* handles the addition operation for the evm */
            /* handle the two error cases that could happen for this call */
            /* it's either insufficient gas or stack underflow */
            if gas < U256::from(3)  { return Err(EvmError::OutOfGas);}
            if stack.data.len() < 2 { return Err(EvmError::StackUnderflow); }
            let stack_top = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_bot = stack.pop().ok_or(EvmError::StackUnderflow);
            stack.push(stack_top.unwrap() + stack_bot.unwrap());
            Ok(())
        },
        0x02 => {
            /* handles the multiplication operation for the evm */
            /* the amount of gas needed for each operation is different */
            if gas < U256::from(5) { return Err(EvmError::OutOfGas);}
            if stack.data.len() < 2 { return Err(EvmError::StackUnderflow);}
            let stack_top = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_bot = stack.pop().ok_or(EvmError::StackUnderflow);
            stack.push(stack_top.unwrap() +  stack_bot.unwrap());
            Ok(())
        },
        0x0A => {
            if gas < U256::from(10) { return Err(EvmError::OutOfGas);}
            if stack.data.len() < 2 { return Err(EvmError::StackUnderflow);}
            let stack_exp = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_val = stack.pop().ok_or(EvmError::StackUnderflow);
            let operation = stack_val.unwrap() ^ stack_exp.unwrap();
            stack.push(operation);
            Ok(())
        },
        0x0B => {
            /* sign extension which is used for immediate register calls */
            /* and specifically used for negative value conversions based */
            /* on two-s complement */
           
            Ok(())
        }
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
        execute(eops, &mut stack, gas).unwrap();
    }
}