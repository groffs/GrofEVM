# GrofEVM
This is a custom implementation of the ethereum virtual machine. The ethereum virtual machine is the core component that defines the runtime needed for smart contracts execution in the ethereum blockchain, it is a stack based machine that does not store data in registers but rather on the stack, it consists of different components as listed below.

# Components of the EVM
- EVM execution environment
The execution environment consists of different components which includes the program counter (PC) that tracks the current instruction in execution, it also consists of the stack which basically implements a LIFO structure and has a maximium depth of 1024 and basically stores information, furthermore it consists of the memory which is a temporary byte-addressable data that is cleared after program execution and is used for intermediate computations, it also consists of the storage which is a persistent key-value store that is specific to one account and does not clear after program execution.

- Account
The account is a key component of the EVM and it consists of the Nonce which basically keep track of the number of transactions output, it also consists of the balance, which is the total number of ethers an account has, it also consists of the storageRoot which is the root hash of the account gotten from storage, and the roothash which is the hash of the bytecode generated.

- Transaction Execution
This consists of different processes which includes the deployment of a contract, calling of a contract and transfers of ethers from one point to another. in the transaction execution there's a concept of gas price and gas fees which help to prevent indefinite loops as every computation step is paid for using gas.

- Gas Mechanism
Every computation performed on the ethereum blockchain is paid for using gas, the gas prevents an event where there's indefinite loop, it also prevents hackers from running arbitary programs on the blockchain and moreso it is used to maintain validity in the blockchain.

- EVM instruction set/opcode
The opcode is the language the virtual machine understands, apparently in the implementation of any virtual machine, it is more like a simulation of an existing CPU's hardware and corresponding instruction set, the EVM opcode as specified in the ethereum yellow paper specifies the instructions that the EVM understands for different operations, for example : add, calldatacopy etc.

- Execution state
The execution state is divided into the world state and the contract state, the world state can also be called the system state which basically comprises of the machine state, the execution environment and gas. the machine state consists of user accounts etc..