use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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
    prog: Vec<u8>, 
    decode_table: HashMap<u8, Instruction>
}

/// defines instructions
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    Ld,             
    Swp,            
    PushAddr,
    PushReg,            
    Pop,            
    Nop,            
    Add,            
    Sub,            
    Mul,            
    And,            
    Or,             
    Xor,            
    Cmp,    
    Mov,        
    MovDregSaddr,   
    MovDaddrSreg,   
    Hlt,            
    JmpAddr,        
    JmpImm,
    Int,
}

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "r0={}, r1={}, r2={}, r3={}\npc={}, sp={}, fl={}",
            self.r0, self.r1, self.r2, self.r3, self.pc, self.sp, self.fl)
    }
}


impl CPU {
    /// initializes the CPU and loads the program into memory
    pub fn init(prog: Vec<u8>) -> Self{
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
        println!("OPCODE 0x{:x}", inst);
        let inst_type = self.decode_table[&inst];
        
        match inst_type {
            Instruction::Add => self.add(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Sub => self.sub(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Mul => self.mul(self.prog[self.pc + 2], self.prog[self.pc + 3]),            
            Instruction::And => self.and(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Or => self.or(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Xor => self.xor(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Cmp => self.cmp(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::Mov => self.mov(self.prog[self.pc + 2], self.prog[self.pc + 3]),
            Instruction::MovDregSaddr => self.mov_dreg_saddr(self.prog[self.pc+1], self.get_u16((self.pc+2) as usize).unwrap()),
            Instruction::MovDaddrSreg => self.mov_daddr_sreg(self.get_u16((self.pc+1) as usize).unwrap(), self.prog[self.pc+2]),
            Instruction::JmpAddr => self.jmp_addr(self.get_u24((inst+1) as usize).unwrap() as usize),
            Instruction::JmpImm => {
                let a = self.get_u32(inst as usize).unwrap() & 0xFFFFFF;
                let b = convert_to_signed(a);
                self.jmp_imm(b)
            },
            Instruction::Ld => self.ld(self.prog[self.pc + 2] as u8, self.get_u16(self.pc+2).unwrap() as usize),
            Instruction::Swp => self.swp(self.prog[self.pc + 2] as u8, self.prog[self.pc + 3]),
            Instruction::PushAddr => self.push_addr(self.get_u24(self.pc + 1).unwrap()),
            Instruction::PushReg => self.push_reg(self.prog[self.pc+1]),
            Instruction::Pop => self.pop(self.prog[self.pc + 2] as u8),
            Instruction::Int => self.int(self.get_u24(self.pc+1).unwrap()),
            Instruction::Nop => self.nop(),
            Instruction::Hlt => self.hlt(),

            _ => panic!("Unknown opcode: 0x{:x}", &inst)
        }
        // increment program counter
        self.pc += 4;

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
            0 => self.r0 = self.get_u32(addr).unwrap(),
            1 => self.r1 = self.get_u32(addr).unwrap(),
            2 => self.r2 = self.get_u32(addr).unwrap(),
            3 => self.r3 = self.get_u32(addr).unwrap(),
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

    /// moves value from `src` (register) into `dest` (register)
    fn mov(&mut self, dest: u8, src: u8) {
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (address) into `dest` (register)
    fn mov_dreg_saddr(&mut self, dest: u8, src: u16) {
        match dest {
            0 => self.r0 = self.get_u32(src as usize).unwrap(),
            1 => self.r1 = self.get_u32(src as usize).unwrap(),
            2 => self.r2 = self.get_u32(src as usize).unwrap(),
            3 => self.r3 = self.get_u32(src as usize).unwrap(),
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (register) into `dest` (address)
    fn mov_daddr_sreg(&mut self, dest: u16, src: u8) {
        let o = self.get_reg(src);
        self.write_u32(dest as usize, o).unwrap();
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
    fn push_addr(&mut self, val: u32) {
        self.stack.push(val);
    }
    
    /// pushes `val` to the stack
    fn push_reg(&mut self, reg: u8) {
        self.stack.push(self.get_reg(reg));
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

    fn int(&mut self, code: u32) {
        match code {
            _ => panic!("Unknown interrupt code {:x}", code)
        }
    }

    /// halt the program
    fn hlt(&mut self) {
        self.pc -= 1;
    } 


    fn get_u32(&self, offset: usize) -> Result<u32, String> {
        if offset as usize > self.prog.len() {

            Err(format!("Offset reference is greater than program size ({} > {})", offset, self.prog.len()))
        } else {
            let mut ret: u32 = 0;
            ret += (self.prog[offset] as u32) << 24;
            ret += (self.prog[offset + 1] as u32) << 16;
            ret += (self.prog[offset + 2] as u32) << 8;
            ret += self.prog[offset + 3] as u32;
            Ok(ret)
        }
    }

    fn get_u24(&self, offset: usize) -> Result<u32, String> {
        if offset as usize > self.prog.len() {
            Err(format!("Offset reference is greater than program size"))
        } else {
            let mut ret: u32 = 0;
            ret += (self.prog[offset] as u32) << 16;
            ret += (self.prog[offset + 1] as u32) << 8;
            ret += self.prog[offset + 2] as u32;
            Ok(ret)
        }
    }

    fn get_u16(&self, offset: usize) -> Result<u16, String> {
        if offset as usize > self.prog.len() {
            Err(format!("Offset reference is greater than program size"))
        } else {
            let mut ret: u16 = 0;
            ret += (self.prog[offset] as u16) << 8;
            ret += self.prog[offset + 1] as u16;
            Ok(ret)
        }
    }

    fn write_u32(&mut self, offset: usize, data: u32) -> Result<(), String> {
        if offset as usize > self.prog.len() {
            Err(format!("Offset reference is greater than program size"))
        } else {
            self.prog[offset] = ((data >> 24) & 0xFF).try_into().unwrap();
            self.prog[offset + 1] = ((data >> 16) & 0xFF).try_into().unwrap();
            self.prog[offset + 2] = ((data >> 8) & 0xFF).try_into().unwrap();
            self.prog[offset + 3] = (data & 0xFF).try_into().unwrap();
            Ok(())
        }
    }
}




