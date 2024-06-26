//! The DroneBoi computer project
//! By Larmbs

/// RAM Storage
/// -----------
/// 16 addresses with 16 bits each equalling 16X16 bits
/// When creating RAM DroneBoi does not allow for setting
/// individual function values in editor
/// Therefore this cannot store instructions
struct RAM {
    mem: [u16; 16],
}
impl RAM {
    fn new() -> Self {
        RAM { mem: [0u16; 16] }
    }
    fn read(&self, addr: usize) -> u16 {
        assert!(addr < self.mem.len(), "Address provided was out of range");
        self.mem[addr]
    }
    fn write(&mut self, addr: usize, val: u16) {
        assert!(addr < self.mem.len(), "Address provided was out of range");
        self.mem[addr] = val;
    }
    /// Returns a string with data about itself
    fn debug(&self) -> String {
        let header = format!("|{:^3}|{:^7}|\n", "I", "Val");
        let bar = "-".repeat(header.len()) + "\n";
        let rows = self.mem.clone()
            .iter()
            .enumerate()
            .map(|(index, val)| format!("|{:3}|{:^7}|", index, val))
            .collect::<Vec<String>>()
            .join("\n");
        header + &bar + &rows
    }
}

/// Program ROM Storage
/// -------------------
/// This storage is solely meant for program memory and will store
/// the entirety of the program
/// This follows the Harvard Architecture https://en.wikipedia.org/wiki/Harvard_architecture
/// As we`ve seen before ROM can be extremely 1 block for 3X16 bits, therefore we will
/// give a big ROM size that is some multiple of 3
/// For a prog length of 64 lines ish we will need 21 function blocks
struct ProgROM {
    mem: [u16; 64],
}
impl ProgROM {
    fn from(mem: [u16; 64]) -> Self {
        ProgROM { mem }
    }
    fn read(&self, addr: usize) -> u16 {
        assert!(addr < self.mem.len(), "Address provided was out of range");
        self.mem[addr]
    }
}

/// ALU Module
/// ----------
/// This module will be kept as simple as possible to minimize the projects complexity
/// there will be no integer overflow protection or anything else.
/// All this will have is binary operators and simple binary addition and subtraction.
/// This means it will have only 8 operations.
struct ALU;
impl ALU {
    /// WARNING
    /// This will return unexpected values when overflow occurs
    fn exec(mode: u8, a: u16, b: u16) -> u16 {
        assert!(
            mode < 11,
            "Mode provided was invalid, modes go from 0 to {} exclusive",
            11
        );

        match mode {
            0 => a << b, // Left Shift
            1 => a >> b, // Right Shift
            2 => a & b,  // AND
            3 => !a,     // NOT
            4 => a ^ b,  // XOR
            5 => a | b,  // OR
            6 => a + b,  // Addition
            7 => a - b,  // Subtraction
            8 => ((a > b) as u16  ) << 3 | ((a < b) as u16) << 2 | ((a == b) as u16) << 1 | 1,
            9 => a + 1,
            10 => a - 1,
            _ => will_never_happen(),
        }
    }
}

/// We will run the CPU here, CPU is just a control unit and is sort of messy
pub fn run(prog: [u16; 64]) -> String {
    // Initializing modules
    let mut ram = RAM::new();
    let rom = ProgROM::from(prog); // Replace this with program values

    // Registers
    /* Private */
    let mut prog_cnt: u16 = 0; // Program counter, tells computer what instruction to read
    /* Public */
    let alu_a: u16 = 0; // The A value to be passed into the ALU
    let alu_b: u16 = 0; // The B value to be passed into the ALU
    let alu_o: u16 = 0; // The output register of ALU
    let flags: u16 = 0; // Register with a multitude of flags
    let base: u16 = 0;  // Base addr when using RAM
    let gpr1: u16 = 0;  // General purpose register no unique use
    let gpr2: u16 = 0;  // General purpose register no unique use

    // Putting all regs in one array to make things simpler to visualize
    let mut regs = [0, alu_a, alu_b, alu_o, flags, base, gpr1, gpr2];

    // Looping over and over
    loop {
        // Getting instruction
        let instr = rom.read(prog_cnt as usize);

        // 16 different possible operations, bit mask and shift to isolate 4 first bits.
        let opcode: u8 = ((instr & 0b1111_0000_0000_0000) >> 12) as u8;

        match opcode {
            0 => {
                // ALU operation
                let mode = instr & 0b1111_1111_1111;
                regs[3] = ALU::exec(mode as u8, regs[1], regs[2]);
            },
            1 => {
                // LOAD operation

                // Bit-masking out args
                let addr = (instr & 0b1111_1111) as usize; // First 8 bits is addr
                let reg = ((instr & 0b1111_0000_0000) >> 8) as usize; // Middle 4 bits is reg addr

                regs[reg] = ram.read(addr);
            },
            2 => {
                // STORE operation

                // Bit-masking out args
                let addr = (instr & 0b1111_1111) as usize; // First 8 bits is addr
                let reg = ((instr & 0b1111_0000_0000) >> 8) as usize; // Middle 4 bits is reg addr

                ram.write(addr, regs[reg]);
            },
            3 => {
                // MOVE instruction

                // Bit-masking out args
                let reg_copy = ((instr & 0b1111_0000) >> 8) as usize;
                let reg_recv = ((instr & 0b1111) >> 4) as usize;

                regs[reg_recv] = regs[reg_copy];
            },
            4 => {
                // JUMP instruction
            
                // Bit-masking jump value
                let jump_size = instr & 0b0001_1111_1111;
                let jump_sign = (instr & 0b0010_0000_0000) >> 9;
                let jump_type = (instr & 0b1100_0000_0000) >> 10;
                
                let res = regs[4] & (1 << jump_type);

                // Jumps if flag is true
                if res == 1 {
                    // Jumping in different direction depending on jump sign
                    match jump_sign {
                        0 => prog_cnt += jump_size,
                        1 => prog_cnt -= jump_size,
                        _ => will_never_happen()
                    }
                }
            },
            5 => {
                // LOAD IMD instruction
                // 8 bit constant value
                let value = instr & 0b1111_1111;
                let reg = ((instr >> 8) & 0b1111) as usize;

                regs[reg] = value;
                
            },
            6 => {
                // HALT Instruction
                break;
            },
            _ => will_never_happen(),
        }
        
        // Incrementing program counter
        prog_cnt += 1;
    }

    format!("DroneBoiComputer VM Debugger\n\
    Created By: Larmbs\n\
    \n\
    RAM Values:\n\
    {}\n\
    \n\
    REG Values:\n\
    {}
    ", ram.debug(), "Not Done Yet")
}

#[inline]
#[allow(unused)]
fn will_never_happen() -> ! {
    panic!("Congratulation in getting this function to run!")
}
