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
    AddReg,
    AddImm,            
    SubReg,
    SubImm,          
    MulReg,
    MulImm,            
    AndReg,
    AndImm,            
    OrReg,
    OrImm,             
    XorReg,
    XorImm,            
    CmpReg,
    CmpImm,    
    MovDregSreg,    
    MovDregSimm,    
    MovDregSaddr,   
    MovDaddrSreg,   
    Hlt,            
    JmpAddr,        
    JmpImm,
    JmpReg,
    JeqReg,
    JeqImm,
    IntReg,
    IntImm
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
    pub fn run(&mut self) -> Result<(), String>{
        loop {
            //println!("\n{}", self);
            self.decode_and_execute()?;
        }
    }

    /// decodes and executes instruction
    fn decode_and_execute(&mut self) -> Result<(), String> {
        if self.pc >= self.prog.len() {
            return Err("Executing illicit memory".to_string());
        }
        let inst = self.prog[self.pc];
        //println!("OPCODE 0x{:x}", inst);
        let inst_type = self.decode_table[&inst];
        
        match inst_type {
            Instruction::AddReg => self.add_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::AddImm => self.add_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::SubReg => self.sub_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::SubImm => self.sub_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::MulReg => self.mul_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),  
            Instruction::MulImm => self.mul_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),            
            Instruction::AndReg => self.and_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::AndImm => self.and_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::OrReg => self.or_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::OrImm => self.or_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::XorReg => self.xor_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::XorImm => self.xor_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::CmpReg => self.cmp_reg(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::CmpImm => self.cmp_imm(self.prog[self.pc + 1], self.get_u16(self.pc + 2).unwrap() as u32),
            Instruction::MovDregSreg => self.mov_dreg_sreg(self.prog[self.pc + 1], self.prog[self.pc + 3]),
            Instruction::MovDregSimm => self.mov_dreg_simm(self.prog[self.pc + 1], self.get_u16(self.pc+2).unwrap()),
            Instruction::MovDregSaddr => self.mov_dreg_saddr(self.prog[self.pc+1], self.get_u16((self.pc+2) as usize).unwrap()),
            Instruction::MovDaddrSreg => self.mov_daddr_sreg(self.get_u16((self.pc+1) as usize).unwrap(), self.prog[self.pc+2]),
            Instruction::JmpAddr => {
                self.jmp_addr(self.get_u24(self.pc+1).unwrap() as usize);
                return Ok(())
            },
            Instruction::JmpImm => {
                let a = self.get_u24(self.pc+1).unwrap();
                let b = convert_to_signed(a);
                self.jmp_imm(b);
                return Ok(())   // note we bail early here so we dont increment pc
            },
            Instruction::JmpReg => {
                self.jmp_reg(self.prog[self.pc+1]);
                return Ok(())
            },
            Instruction::JeqImm => {
                self.jeq_imm(self.get_u24(self.pc+1).unwrap());
                return Ok(())
            },
            Instruction::JeqReg => {
                self.jeq_reg(self.prog[self.pc+1]);
                return Ok(())
            },
            Instruction::Ld => self.ld(self.prog[self.pc + 1], self.get_u16(self.pc+2).unwrap() as usize),
            Instruction::Swp => self.swp(self.prog[self.pc + 1], self.prog[self.pc + 2]),
            Instruction::PushAddr => self.push_addr(self.get_u24(self.pc + 1).unwrap()),
            Instruction::PushReg => self.push_reg(self.prog[self.pc+1]),
            Instruction::Pop => self.pop(self.prog[self.pc + 1]),
            Instruction::IntImm => self.int_imm(self.get_u24(self.pc+1).unwrap()),
            Instruction::IntReg => self.int_reg(self.prog[self.pc+1]),
            Instruction::Nop => self.nop(),
            Instruction::Hlt => self.hlt(),

            //_ => panic!("Unknown opcode: 0x{:x}", &inst)
        }
        // increment program counter
        self.pc += 4;

        Ok(())

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
    fn add_reg(&mut self, dest: u8, src: u8) {
        println!("ADD r{},r{}", dest, src);
        let v = self.get_reg(src);
        
        match dest {
            0 => self.r0 += v,
            1 => self.r1 += v,
            2 => self.r2 += v,
            3 => self.r3 += v,
            _ => panic!("Illicit destination value {}", dest)            
        };
    }

    fn add_imm(&mut self, dest: u8, src: u32) {
        println!("ADDI r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 += src,
            1 => self.r1 += src,
            2 => self.r2 += src,
            3 => self.r3 += src,
            _ => panic!("Illicit destination value {}", dest)            
        };
    }

    /// performs logical AND operation, storing result in `dest`
    fn and_reg(&mut self, dest: u8, src: u8) {
        println!("AND r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 &= o,
            1 => self.r1 &= o,
            2 => self.r2 &= o,
            3 => self.r3 &= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical AND operation, storing result in `dest`
    fn and_imm(&mut self, dest: u8, src: u32) {
        println!("ANDI r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 &= src,
            1 => self.r1 &= src,
            2 => self.r2 &= src,
            3 => self.r3 &= src,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical OR operation, storing result in `dest`
    fn or_reg(&mut self, dest: u8, src: u8) {
        println!("OR r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 |= o,
            1 => self.r1 |= o,
            2 => self.r2 |= o,
            3 => self.r3 |= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical OR operation, storing result in `dest`
    fn or_imm(&mut self, dest: u8, src: u32) {
        println!("ORI r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 |= src,
            1 => self.r1 |= src,
            2 => self.r2 |= src,
            3 => self.r3 |= src,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// performs logical XOR operation, storing result in `dest`
    fn xor_reg(&mut self, dest: u8, src: u8) {
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

    /// performs logical XOR operation, storing result in `dest`
    fn xor_imm(&mut self, dest: u8, src: u32) {
        println!("XORI r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 ^= src,
            1 => self.r1 ^= src,
            2 => self.r2 ^= src,
            3 => self.r3 ^= src,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// compares two register values, storing success in
    fn cmp_reg(&mut self, test: u8, src: u8) {
        println!("CMP r{},r{}", test, src);
        
        let o = self.get_reg(src) as i64;
        let t = self.get_reg(test) as i64;
        let v = match t-o {
            0 => 1,
            _ => 0
        };
        self.fl |= v;
    }

    /// compares two register values, storing success in
    fn cmp_imm(&mut self, test: u8, src: u32) {
        println!("CMPI r{},0x{:x}", test, src);
        
        let t = self.get_reg(test) as i64;
        let src = src as i64;
        let v = match t-src {
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
        println!("LOAD r{}, 0x{:x}", dest, addr);
        match dest {
            0 => self.r0 = self.get_u32(addr).unwrap(),
            1 => self.r1 = self.get_u32(addr).unwrap(),
            2 => self.r2 = self.get_u32(addr).unwrap(),
            3 => self.r3 = self.get_u32(addr).unwrap(),
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// multiplies `dest` with `src`, storing in `dest`
    fn mul_reg(&mut self, dest: u8, src: u8) {
        println!("MUL r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 *= o,
            1 => self.r1 *= o,
            2 => self.r2 *= o,
            3 => self.r3 *= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// multiplies `dest` with `src`, storing in `dest`
    fn mul_imm(&mut self, dest: u8, src: u32) {
        println!("MULI r{},r{}", dest, src);
        match dest {
            0 => self.r0 *= src,
            1 => self.r1 *= src,
            2 => self.r2 *= src,
            3 => self.r3 *= src,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// subtracts `dest` with `src`, storing in `dest`
    fn sub_reg(&mut self, dest: u8, src: u8) {
        println!("SUB r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 -= o,
            1 => self.r1 -= o,
            2 => self.r2 -= o,
            3 => self.r3 -= o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// subtracts `dest` with `src`, storing in `dest`
    fn sub_imm(&mut self, dest: u8, src: u32) {
        println!("SUB r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 -= src,
            1 => self.r1 -= src,
            2 => self.r2 -= src,
            3 => self.r3 -= src,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (register) into `dest` (register)
    fn mov_dreg_sreg(&mut self, dest: u8, src: u8) {
        println!("MOV r{},r{}", dest, src);
        let o = self.get_reg(src);
        match dest {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (register) into `dest` (register)
    fn mov_dreg_simm(&mut self, dest: u8, src: u16) {
        println!("MOVI r{},0x{:x}", dest, src);
        match dest {
            0 => self.r0 = src as u32,
            1 => self.r1 = src as u32,
            2 => self.r2 = src as u32,
            3 => self.r3 = src as u32,
            _ => panic!("Illicit destination value {}", dest)            
        }
    }

    /// moves value from `src` (address) into `dest` (register)
    fn mov_dreg_saddr(&mut self, dest: u8, src: u16) {
        println!("MOVA r{}, 0x{:x}", dest, src);
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
        println!("MOVR 0x{:x},r{}", dest, src);
        let o = self.get_reg(src);
        self.write_u32(dest as usize, o).unwrap();
    }

    /// swaps `r1` and `r2`
    fn swp(&mut self, r1: u8, r2: u8) {
        println!("SWP r{},r{}", r1, r2);
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
        println!("PUSHA 0x{:x}", val);
        self.stack.push(val);
    }
    
    /// pushes `val` to the stack
    fn push_reg(&mut self, reg: u8) {
        println!("PUSH r{}", reg);
        self.stack.push(self.get_reg(reg));
    }

    /// pops the top value from the stack into `dest`
    fn pop(&mut self, dest: u8) {
        println!("POP r{}", dest);
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
        println!("NOP");
        self.swp(0, 0);
    }

    /// performs a long jump 
    fn jmp_addr(&mut self, addr: usize) {
        println!("JMPL 0x{}", addr);
        self.pc = addr;
    }
    
    /// performs a short jump to offset 
    fn jmp_imm(&mut self, short: i32) {
        println!("JMPI 0x{:x}", short);
        if short < 0 {
            self.pc -= short.abs() as usize;
        } else {
            self.pc += short.abs() as usize;
        }
    }

    /// performs a jump to an offset stored in a register
    fn jmp_reg(&mut self, reg: u8) {
        println!("JMP r{}", reg);
        self.pc = self.get_reg(reg) as usize;
    }
    
    /// performs a jump to an offset stored in a register
    fn jeq_imm(&mut self, reg: u32) {
        println!("JEQI 0x{:x}", reg);
        if self.fl & 0x1 == 1 {
            self.fl &= 0xfe;
            self.pc = reg as usize;
        } else {
            self.pc += 4;
        }
    }

    /// performs a jump to an offset stored in a register
    fn jeq_reg(&mut self, reg: u8) {
        println!("JEQ r{}", reg);
        let o = self.get_reg(reg);

        if self.fl & 0x1 == 1 {
            self.fl &= 0xfe;
            self.pc = o as usize;
        } else {
            self.pc += 4;
        }
    }

    /// handles an immediate interrupt
    fn int_imm(&mut self, code: u32) {
        self.handle_interrupt(code);
    }

    /// handles an interrupt in a register
    fn int_reg(&mut self, reg: u8) {
        let code = self.get_reg(reg);

        self.handle_interrupt(code);
    }

    /// handles interrupt codes
    fn handle_interrupt(&mut self, code: u32) {
        match code {
            _ => panic!("Unknown interrupt code {:x}", code)
        }
    }

    /// halt the program
    fn hlt(&mut self) {
        println!("HLT");
        std::process::exit(1);
    } 


    fn get_u32(&self, offset: usize) -> Result<u32, String> {
        if offset as usize > self.prog.len() -3 {
            Ok(0)
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
            // extend the space available 
            let mut m: Vec<u8> = Vec::with_capacity(offset-self.prog.len() + 4);
            for _ in 0..m.capacity() {
                m.push(0);
            }
            self.prog.extend(m);
        } 
        
        self.prog[offset] = ((data >> 24) & 0xFF).try_into().unwrap();
        self.prog[offset + 1] = ((data >> 16) & 0xFF).try_into().unwrap();
        self.prog[offset + 2] = ((data >> 8) & 0xFF).try_into().unwrap();
        self.prog[offset + 3] = (data & 0xFF).try_into().unwrap();
        Ok(())
    }
}




