# Drone Boi Computer 0.2.0
## Registers
Prog_CNT
ALU_A
ALU_B
ALU_O

## Instruction Set
| Done | Name | Explanation
|------|------|------------
||Jump|Moves program counter by some amount
||Store|Moves a value from register to RAM
||Load|Moves a value from RAM to register
|X|ALU|Operates on values in the A and B registers
||Move|Moves values between registers

## Opcode Distribution
There is a 4 bit opcode so here is the distribution of instructions
|Range|Label|Instructions|
|-----|-----|------------|
| 0..7|ALU|(Add, Sub, stl, str, or, and, not, xor) |
|8|Load| Load from RAM to reg
|9|Store| Store value from reg to RAM
|10|Move| Move value between registers

