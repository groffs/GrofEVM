use primitive_types::U256;
use sha2::digest::consts::U25;

use super::stack::Stack;

pub enum Opcode {
    Add,
    Mul,
    Exp,
    SignExtend,
    CallDataCopy
}

#[derive(Debug)]
pub enum EvmError {
    OutOfGas,
    StackUnderflow
}

pub type EvmResult<T> = Result<T, EvmError>;


pub fn execute(evmop : Opcode, stack: &mut Stack, gas: U256) -> EvmResult<()>{
    match evmop {
        Opcode::Add => {
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
        Opcode::Mul => {
            /* handles the multiplication operation for the evm */
            /* the amount of gas needed for each operation is different */
            if gas < U256::from(5) { return Err(EvmError::OutOfGas);}
            if stack.data.len() < 2 { return Err(EvmError::StackUnderflow);}
            let stack_top = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_bot = stack.pop().ok_or(EvmError::StackUnderflow);
            stack.push(stack_top.unwrap() *  stack_bot.unwrap());
            Ok(())
        },
        Opcode::Exp => {
            if gas < U256::from(10) { return Err(EvmError::OutOfGas);}
            if stack.data.len() < 2 { return Err(EvmError::StackUnderflow);}
            let stack_exp = stack.pop().ok_or(EvmError::StackUnderflow);
            let stack_val = stack.pop().ok_or(EvmError::StackUnderflow);
            let operation = stack_val.unwrap() ^ stack_exp.unwrap();
            stack.push(operation);
            Ok(())
        },
        Opcode::CallDataCopy => {
            /* handles the calldatacopy operation for the evm */
            /* the calldatacopy call has the gas formula below */
            /* Gverylow + Gcopy × dµs[2] ÷ 32e */
            let very_low = stack.pop().ok_or(EvmError::StackUnderflow)?;
            let copy_gas = stack.pop().ok_or(EvmError::StackUnderflow)?;
            let cdsize = stack.pop().ok_or(EvmError::StackUnderflow)?;
            let mod_stack = cdsize % 32;

            /* compute the gas cost for the call data copy and */
            /* determine if the caller has enough gas for the op */
            if U256::from(mod_stack) > gas /* could also mean gas < mod_stack */ {
                /* let's see the amount of gas passed at this point */
                dbg!(gas); dbg!(cdsize);
                return Err(EvmError::OutOfGas);
            }
            let stack_ops = very_low + (copy_gas * mod_stack );
            stack.push(stack_ops);
            Ok(())
        },
        Opcode::SignExtend => {
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