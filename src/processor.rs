use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use crate::translation::{
    build_translation_table,
    convert_to_signed
};


/// implements the cpu's functionality
pub struct CPU {
    // general purpose registers
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,

    // specialized registers
    pc: usize, // program counter
    sp: u32, // stack pointer
    fl: u8, // flag register
    /*
    Flags: 
        Zero: 0x1
        Carry: 0x2
        Greater: 0x4
    */

    // program information
    stack: Vec<u32>,
    prog: Vec<u32>, 
    decode_table: HashMap<u8, Instruction>
}

/// defines instructions
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Sfg,            
    Ld,             
    Swp,            
    Push,            
    Pop,            
    Nop,            
    
    Add,            
    Sub,            
    Mul,            
    
    And,            
    Or,             
    Xor,            
    
    Cmp,            
    MovDregSaddr,   
    MovDaddrSreg,   
    Hlt,            
    JmpAddr,        
    JmpImm          
}

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "r0={}, r1={}, r2={}, r3={}\npc={}, sp={}, fl={}",
            self.r0, self.r1, self.r2, self.r3, self.pc, self.sp, self.fl)
    }
}


impl CPU {
    /// initializes the CPU and loads the program into memory
    pub fn init(prog: Vec<u32>) -> Self{
        // initialize...
        CPU {
            // ... GP registers ...
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,

            // ... special registers ...
            pc: 0,
            sp: 0,
            fl: 0,

            // ... program related stuff
            stack: Vec::new(),
            prog,
            decode_table: build_translation_table()
        }
    }

    /// run the processor
    pub fn run(&mut self) {
        loop {
            self.decode_and_execute();
        }
    }

    /// decodes and executes instruction
    fn decode_and_execute(&mut self){
        let inst = self.prog[self.pc];
        let op_code = (inst>>24) as u8;
        let inst_type = self.decode_table[&op_code];
        
        match inst_type {
            Instruction::Add => self.add((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Sub => self.sub((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Mul => self.mul((inst>>16) as u8, (inst>>8) as u8),            
            Instruction::And => self.and((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Or => self.or((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Xor => self.xor((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Cmp => self.cmp((inst>>16) as u8, (inst>>8) as u8),
            Instruction::MovDregSaddr => self.mov_dreg_saddr((inst>>16) as u8, (inst & 0xFFFF) as u16),
            Instruction::MovDaddrSreg => self.mov_daddr_sreg((inst & 0xFFFF00) as u16, (inst>>8) as u8),
            Instruction::JmpAddr => self.jmp_addr((inst & 0xFFFFFF) as usize),
            Instruction::JmpImm => {
                let a = inst & 0xFFFFFF;
                let b = convert_to_signed(a);
                self.jmp_imm(b)
            },
            Instruction::Sfg => self.sfg((inst>>16) as u8, (inst & 0xFFFF) as u8),
            Instruction::Ld => self.ld((inst>>16) as u8, (inst & 0xFFFF) as usize),
            Instruction::Swp => self.swp((inst>>16) as u8, (inst>>8) as u8),
            Instruction::Push => self.push((inst & 0xFFFFFF) as u32),
            Instruction::Pop => self.pop((inst>>16) as u8),
            Instruction::Nop => self.nop(),
            Instruction::Hlt => self.hlt(),

            _ => panic!("Unknown opcode: 0x{:x}", op_code)
        }
        // increment program counter
        self.pc += 1;

    }

    /// gets the value of a register
    fn get_reg(&self, r: u8) -> u32 {
        match r {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            _ => panic!("Illicit value {}", r)
        }
    }

    /// adds value in `src` into `dest`
    fn add(&mut self, dest: u8, src: u8) {
        let v = self.get_reg(src);
        
        match dest {
            0 => self.r0 + v,
            1 => self.r1 + v,
            2 => self.r2 + v,
            3 => self.r3 + v,
            _ => panic!("Illicit destination value {}", dest)            
        };
    }

    /// performs logical AND operation, storing result in `dest`
    fn and(&mut self, dest: u8, src: u8) {
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 &= o,
            1 => self.r1 &= o,
            2 => self.r2 &= o,
            3 => self.r3 &= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical OR operation, storing result in `dest`
    fn or(&mut self, dest: u8, src: u8) {
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 |= o,
            1 => self.r1 |= o,
            2 => self.r2 |= o,
            3 => self.r3 |= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical XOR operation, storing result in `dest`
    fn xor(&mut self, dest: u8, src: u8) {
        println!("XOR r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 ^= o,
            1 => self.r1 ^= o,
            2 => self.r2 ^= o,
            3 => self.r3 ^= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// compares two register values, storing success in
    fn cmp(&mut self, test: u8, src: u8) {
        let o = self.get_reg(src);
        let t = self.get_reg(test);
        let v = match (t-o) {
            0 => 1,
            _ => 0
        };
        self.fl |= v;
    }

    /// sets the flag value according to `val` and `flags`
    fn sfg(&mut self, flag: u8, val: u8) {
        match flag {
            0 => {}, // zero flag
            1 => {}, // carry flag
            2 => {}, // greater flag
            _ => {panic!("illegal flag {}", flag)}
        }
    }

    /// loads a 32-bit value from `addr` into `dest`
    fn ld(&mut self, dest: u8, addr: usize) {
        match dest {
            0 => self.r0 = self.prog[addr],
            1 => self.r1 = self.prog[addr],
            2 => self.r2 = self.prog[addr],
            3 => self.r3 = self.prog[addr],
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// multiplies `dest` with `src`, storing in `dest`
    fn mul(&mut self, dest: u8, src: u8) {
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 *= o,
            1 => self.r1 *= o,
            2 => self.r2 *= o,
            3 => self.r3 *= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// subtracts `dest` with `src`, storing in `dest`
    fn sub(&mut self, dest: u8, src: u8) {
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 -= o,
            1 => self.r1 -= o,
            2 => self.r2 -= o,
            3 => self.r3 -= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (address) into `dest` (register)
    fn mov_dreg_saddr(&mut self, dest: u8, src: u16) {
        match dest {
            0 => self.r0 = self.prog[src as usize],
            1 => self.r1 = self.prog[src as usize],
            2 => self.r2 = self.prog[src as usize],
            3 => self.r3 = self.prog[src as usize],
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (register) into `dest` (address)
    fn mov_daddr_sreg(&mut self, dest: u16, src: u8) {
        let o = self.get_reg(src);
        self.prog[dest as usize] = o;
    }

    /// swaps `r1` and `r2`
    fn swp(&mut self, r1: u8, r2: u8) {
        let o = self.get_reg(r2);
        let t = self.get_reg(r1);

        match r2 {
            0 => self.r0 = t,
            1 => self.r1 = t,
            2 => self.r2 = t,
            3 => self.r3 = t,
            _ => panic!("Illicit destination value {}", r2)            
        }
        match r1 {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => panic!("Illicit destination value {}", r1)            
        }
    }

    /// pushes `val` to the stack
    fn push(&mut self, val: u32) {
        self.stack.push(val);
    }

    /// pops the top value from the stack into `dest`
    fn pop(&mut self, dest: u8) {
        match dest {
            0 => self.r0 = self.stack.pop().unwrap(),
            1 => self.r1 = self.stack.pop().unwrap(),
            2 => self.r2 = self.stack.pop().unwrap(),
            3 => self.r3 = self.stack.pop().unwrap(),
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// does nothing
    fn nop(&mut self) {
        self.swp(0, 0);
    }

    /// performs a long jump 
    fn jmp_addr(&mut self, addr: usize) {
        self.pc = addr;
    }
    
    /// performs a short jump to offset stored in register
    fn jmp_imm(&mut self, short: i32) {
        if short < 0 {
            self.pc -= short.abs() as usize;
        } else {
            self.pc += short.abs() as usize;
        }
    }

    /// halt the program
    fn hlt(&mut self) {
        self.pc -= 1;
    } 
}




