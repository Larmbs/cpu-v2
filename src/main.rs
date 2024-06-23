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
}

/// Program ROM Storage
/// -----------
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
            mode < 2u8.pow(3),
            "Mode provided was invalid, modes go from 0 to {}",
            2u8.pow(3)
        );

        match mode {
            0b000 => a << b, // Left Shift
            0b001 => a >> b, // Right Shift
            0b010 => a & b,  // AND
            0b011 => !a,     // NOT
            0b100 => a ^ b,  // XOR
            0b101 => a | b,  // OR
            0b110 => a - b,  // Addition
            0b111 => a - b,  // Subtraction
            _ => will_never_happen(),
        }
    }
}

/// We will run the CPU here, CPU is just a control unit and is sort of messy
fn main() {
    // Initializing modules
    let mut ram = RAM::new();
    let rom = ProgROM::from([016; 64]); // Replace this with program values

    // Registers
    let mut prog_cnt: u16 = 0; // Program counter, tells computer what instruction to read
    let mut alu_a: u16 = 0; // The A value to be passed into the ALU
    let mut alu_b: u16 = 0; // The B value to be passed into the ALU
    let mut alu_o: u16 = 0; // The output register of ALU

    // Looping over and over
    loop {
        // Getting instruction
        let instr = rom.read(prog_cnt as usize);

        // 16 different possible operations, bit mask and shift to isolate 4 first bits.
        let opcode: u8 = ((instr & 0b1111_0000_0000_0000) >> 12) as u8;

        match opcode {
            0..=7 => {
                // ALU operations
                alu_o = ALU::exec(opcode as u8, alu_a, alu_b);
            },
            _ => will_never_happen(),
        }
        
        // Incrementing program counter
        prog_cnt += 1;
    }
}

#[inline]
#[allow(unused)]
fn will_never_happen() -> ! {
    panic!("Congratulation in getting this function to run!")
}
