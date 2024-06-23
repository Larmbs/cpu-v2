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
- GPR


## Instruction Set
| Done | Name | Explanation
|------|------|------------
|X|Jump|Moves program counter by some amount
|X|Store|Moves a value from register to RAM
|X|Load|Moves a value from RAM to register
|X|ALU|Operates on values in the A and B registers
|X|Move|Moves values between registers
|X|LOAD IMD| Moves a value imediatly into a reg

## Opcode Distribution
There is a 4 bit opcode so here is the distribution of instructions
|Range|Label|Instructions|
|-----|-----|------------|
|0..9|ALU|(Add, Sub, stl, str, or, and, not, xor) |
|10|Load| Load from RAM to reg
|11|Store| Store value from reg to RAM
|12|Move| Move value between registers
|13|Jump| Jumps progcounter by some value
|14|LOAD IMD| Loads a predefined 12 bit number into a reg

