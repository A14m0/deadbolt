# The Deadbolt Instruction Set
The Deadbolt software processor implements a custom instruction set. This 
document explains what each does and the parameters it includes. 

One quick note about the format of this documentation: 
1. Registers will be in the format `r[i]` where `i` is some integer.
2. Addresses will be represented as `addr`. Deadbolt uses a 32-bit address space, however because of limitations of my implementation of the instruction set some instructions do not support full 32-bit addresses. Those exceptions will be noted in the description.
3. Immediates will be represented as `imm`. Like the address limitations above, the description will note the maximum length that an immediate can be for a given instruction.

## Registers
The processor has 4 32-bit general purpose registers (`r0, r1, r2, r3`), as well 
as a program counter, stack pointer, and a flag register.  

## CPU Flags
The CPU has a few flags that tell it how to operate
### ZERO Flag
This flag gets set when a compare operation returns that two values are the same
and is represented as the 1 bit of the flag register.

### CARRY Flag
Signals if a mathematical operation triggers an overflow and is represented as 
the 2 bit of the flag register.

### GREATER Flag
Signals if a compare operation returns a value that is greater than the other 
and is represented as the 4 bit of the flag register

### ECHO Flag
Signals that the `int_readcon` interrupt should echo the input back to the 
console. Hopefully this will be moved out of the flags register into a proper
spot in the architecture, but for the time being this is what we got.


## Instruction Table
| Inst  | Args | Desc |
|-------|------|------|
| add   | `r0, r1`  | Adds the value from register `r1` into `r0`. Stores in `r0` |
| addi  | `r0, imm` | Adds `imm` into `r0`. `imm <= 0xffff`, stores in `r0` |
| sub   | `r0, r1`  | Subtracts the value from register `r1` from `r0`. Stores in `r0`|
| subi  | `r0, imm` | Subtracts `imm` from `r0`. `imm <= 0xffff`, stores in `r0` |
| mul   | `r0, r1`  | Multiplies `r0` by the value in `r1`. Stores in `r0`|
| muli  | `r0, imm` | Multiplies `r0` by the immediate value. `imm <= 0xffff`, stores in `r0` |
| and   | `r0, r1`  | Bitwise AND between values in `r0` and `r1`. Stores in `r0` |
| andi  | `r0, imm` | Bitwise AND between value in `r0` and `imm`. `imm <= 0xffff`, stores in `r0` |
| or    | `r0, r1`  | Bitwise OR between values in `r0` and `r1`. Stores in `r0` |
| ori   | `r0, imm` | Bitwise OR between value in `r0` and `imm`. `imm <= 0xffff`, stores in `r0` |
| xor   | `r0, r1`  | Bitwise XOR between the values in `r0` and `r1`. Stores in `r0` |
| xori  | `r0, imm` | Bitwise XOR between the value in `r0` and `imm`. `imm <= 0xffff`, stores in `r0` |
| cmp   | `r0, r1`  | Compares the values in `r0` and `r1`, setting the CPU flags accordingly |
| cmpi  | `r0, imm` | Compares the value in `r0` and the `imm`, setting the CPU flags accordingly |
| mov   | `r0, r1`  | Moves the value of `r1` into `r0` |
| movi  | `r0, imm` | Moves the immediate value `imm` into `r0`. `imm <= 0xffff` |
| mova  | `r0, addr`| Moves the address into `r0`. `addr <= 0xffff` |
| movr  | `addr, r1`| Moves the value of `r1` to the address `addr`. `addr <= 0xffff` |
| ldi   | `r0, addr`| Loads the value stored at `addr` into `r0`. `addr <= 0xffff` |
| ldr   | `r0, r1`  | Loads the value stored at the address stored in `r1` into `r0` |
| swp   | `r0, r1`  | Swaps the values of `r1` and `r0` |
| pusha | `addr`    | Pushes the address `addr` to the stack. `addr <= 0xffffff` |
| push  | `r0`      | Pushes the value of `r0` to the stack |
| sfgr  | `r0`      | Sets the processor flags according to `r0` |
| sfgi  | `imm`     | Sets the processor flags according to `imm`. `imm <= 0xffffff` |
| pop   | `r0`      | Pops the value from the top of the stack into `r0` |
| nop   | None      | No-operation instruction |
| hlt   | None      | Halt the processor |
| jmpl  | `r0`      | Jumps to the value stored in `r0` |
| jmpi  | `imm`     | Jumps to the instruction pointer + `imm`, where `imm` is a signed 24-bit integer |
| jmp   | `addr`    | Jumps to the address `addr`. `addr <= 0xffffff` |
| jeq   | `addr`    | Jumps to the address `addr` if the ZERO processor flag is set. `addr <= 0xffffff` |
| jeqi  | `imm`     | Jumps to the instruction pointer + `imm` if the ZERO processor flag is set. `imm` is a signed 24-bit integer|
| int   | `imm`     | Run an interrupt specified by `imm`. `imm <= 0xffffff` |
| intr  | `r0`      | Run an interrupt specified by the value of `r0` |