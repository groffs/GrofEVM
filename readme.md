# GrofEVM
This is a custom implementation of the ethereum virtual machine. The ethereum virtual machine is the core component that defines the runtime needed for smart contracts execution in the ethereum blockchain, it is a stack based machine that does not store data in registers but rather on the stack, it consists of different components as listed below.

# Components of the EVM
- EVM execution environment
The execution environment consists of different components which includes the program counter (PC) that tracks the current instruction in execution, it also consists of the stack which basically implements a LIFO structure and has a maximium depth of 1024 and basically stores information, furthermore it consists of the memory which is a temporary byte-addressable data that is cleared after program execution and is used for intermediate computations, it also consists of the storage which is a persistent key-value store that is specific to one account and does not clear after program execution.

- Account
The account is a key component of the EVM and it consists of the Nonce which basically keep track of the number of transactions output, it also consists of the balance, which is the total number of ethers an account has, it also consists of the storageRoot which is the root hash of the account gotten from storage, and the roothash which is the hash of the bytecode generated.

- Transaction Execution
This consists of different processes which includes the deployment of a contract, calling of a contract and transfers of ethers from one point to another.