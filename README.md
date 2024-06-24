# Drone Boi Computer 0.2.0
A computer built to be made in DroneBoi with a simple architecture and instruction set

## Registers
### Private
Prog_CNT
### Public
- Zero
- ALU_A
- ALU_B
- ALU_O
- FLAGS
- BASE
- GPR1
- GPR2

## Instruction Set
| Done | Name | Explanation
|------|------|------------
|X|Jump|Moves program counter by some amount
|X|Store|Moves a value from register to RAM
|X|Load|Moves a value from RAM to register
|X|ALU|Operates on values in the A and B registers
|X|Move|Moves values between registers
|X|LOAD IMD| Moves a value imediatly into a reg
|X|Halt| Stops CPU from running

## Opcode Distribution
There is a 4 bit opcode so here is the distribution of instructions
|Range|Label|Instructions|
|-----|-----|------------|
|0|ALU|(Add, Sub, stl, str, or, and, not, xor) |
|1|Load| Load from RAM to reg
|2|Store| Store value from reg to RAM
|3|Move| Move value between registers
|4|Jump| Jumps progcounter by some value
|5|LOAD IMD| Loads a predefined 12 bit number into a reg
|6|HALT| 

# Flags
16 available flags
- 0 Constant 1 value
- 1 Greater than
- 2 Equal to
- 3 Less than
- 4 ?
- 5 ?
- 6 ?
- 7 ?
- 8 ?
- 9 ?
- 10 ?
- 11 ?
- 12 ?
- 13 ?
- 14 ?
- 15 ?
