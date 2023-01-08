use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::translation::{
    build_translation_table,
    convert_to_signed
};

use crate::processor::cpu::mmu::MMU;
use crate::processor::cpu::interrupts::{IntFn, build_interrupt_table};
use crate::processor::instructions::Instruction;

use crate::debug::debug;





/// implements the cpu's functionality
pub struct CPU {
    // general purpose registers
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,

    // specialized registers
    pc: usize, // program counter
    sp: usize, // stack pointer
    fl: u8, // flag register
    /*
    Flags: 
        Zero: 1
        Carry: 2
        Greater: 3
        Echo: 4
    */

    // program information
    pub memory: MMU,
    interrupt_table: HashMap<u32, IntFn>,
    prog: Vec<u8>, 

    decode_table: HashMap<u8, Instruction>
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
            memory: MMU::new(),
            prog,
            interrupt_table: build_interrupt_table(),
            decode_table: build_translation_table()
        }
    }

    /// run the processor
    pub fn run(&mut self) -> Result<(), String>{

        // load the program into memory
        for i in 0..self.prog.len() {
            self.memory[i] = self.prog[i];
            assert_eq!(self.memory[i], self.prog[i]);
        }

        loop {
            debug(format!("\n{}", self));
            self.decode_and_execute()?;
        }
    }

    /// decodes and executes instruction
    fn decode_and_execute(&mut self) -> Result<usize, String> {
        let inst = self.memory[self.pc];
        //debug(format!("OPCODE 0x{:x}", inst);
        let inst_type = match self.decode_table.get(&inst) {
            Some(a) => a,
            None => return Err(format!("Illegal instruction 0x{:x}", inst))
        };
        
        // match the instruction, saving how much we need to increment the program couter
        let incr = match inst_type {
            Instruction::AddReg => self.add_reg(),
            Instruction::AddImm => self.add_imm(),
            Instruction::SubReg => self.sub_reg(),
            Instruction::SubImm => self.sub_imm(),
            Instruction::MulReg => self.mul_reg(),  
            Instruction::MulImm => self.mul_imm(),            
            Instruction::AndReg => self.and_reg(),
            Instruction::AndImm => self.and_imm(),
            Instruction::OrReg => self.or_reg(),
            Instruction::OrImm => self.or_imm(),
            Instruction::XorReg => self.xor_reg(),
            Instruction::XorImm => self.xor_imm(),
            Instruction::CmpReg => self.cmp_reg(),
            Instruction::CmpImm => self.cmp_imm(),
            Instruction::MovDregSreg => self.mov_dreg_sreg(),
            Instruction::MovDregSimm => self.mov_dreg_simm(),
            Instruction::MovDregSaddr => self.mov_dreg_saddr(),
            Instruction::MovDaddrSreg => self.mov_daddr_sreg(),
            Instruction::JmpAddr => {
                return self.jmp_addr();
            },
            Instruction::JmpImm => {
                return self.jmp_imm();
            },
            Instruction::JmpReg => {
                return self.jmp_reg();
            },
            Instruction::JeqImm => {
                return self.jeq_imm();
            },
            Instruction::JeqReg => {
                return self.jeq_reg();
            },
            Instruction::LdImm => self.ld_imm(),
            Instruction::LdReg => self.ld_reg(),
            Instruction::SfgImm => self.sfg_imm(),
            Instruction::SfgReg => self.sfg_reg(),
            Instruction::Swp => self.swp(),
            Instruction::PushAddr => self.push_addr(),
            Instruction::PushReg => self.push_reg(),
            Instruction::Pop => self.pop(),
            Instruction::IntImm => self.int_imm(),
            Instruction::IntReg => self.int_reg(),
            Instruction::Nop => self.nop(),
            Instruction::Hlt => self.hlt(),

            //_ => panic!("Unknown opcode: 0x{:x}", &inst)
        }?;
        // increment program counter
        self.pc += incr;

        Ok(0)

    }

    /// gets the value of a register
    pub fn get_reg(&self, r: u8) -> Result<u32, String> {
        match r {
            0 => Ok(self.r0),
            1 => Ok(self.r1),
            2 => Ok(self.r2),
            3 => Ok(self.r3),
            _ => Err(format!("Illicit value {}", r))
        }
    }

    /// sets the value of a register
    pub fn set_reg(&mut self, r: u8, v: u32) -> Result<(), String> {
        match r {
            0 => self.r0 = v,
            1 => self.r1 = v,
            2 => self.r2 = v,
            3 => self.r3 = v,
            _ => return Err(format!("Illicit value {}", r))
        };

        Ok(())
    }

    /// checks if a certain flag is set
    pub fn is_flag_set(&self, flag: u8) -> bool {
        let v = flag & self.fl;
        match v {
            0 => false,
            _ => true
        }
    }

    /// adds value in `src` into `dest`
    fn add_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;

        debug(format!("ADD r{},r{}", dest, src));
        let v = self.get_reg(src)?;
        
        match dest {
            0 => self.r0 += v,
            1 => self.r1 += v,
            2 => self.r2 += v,
            3 => self.r3 += v,
            _ => return Err(format!("Illicit destination value {}", dest))        
        };
        Ok(2)
    }

    fn add_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1];
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("ADDI r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 += src,
            1 => self.r1 += src,
            2 => self.r2 += src,
            3 => self.r3 += src,
            _ => return Err(format!("Illicit destination value {}", dest))            
        };

        Ok(6)
    }

    /// performs logical AND operation, storing result in `dest`
    fn and_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;

        debug(format!("AND r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 &= o,
            1 => self.r1 &= o,
            2 => self.r2 &= o,
            3 => self.r3 &= o,
            _ => return Err(format!("Illicit destination value {}", dest))         
        }
        Ok(2)
    }

    /// performs logical AND operation, storing result in `dest`
    fn and_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1];
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("ANDI r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 &= src,
            1 => self.r1 &= src,
            2 => self.r2 &= src,
            3 => self.r3 &= src,
            _ => return Err(format!("Illicit destination value {}", dest))           
        }

        Ok(6)
    }

    /// performs logical OR operation, storing result in `dest`
    fn or_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;


        debug(format!("OR r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 |= o,
            1 => self.r1 |= o,
            2 => self.r2 |= o,
            3 => self.r3 |= o,
            _ => return Err(format!("Illicit destination value {}", dest))         
        }

        Ok(2)
    }

    /// performs logical OR operation, storing result in `dest`
    fn or_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1];
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("ORI r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 |= src,
            1 => self.r1 |= src,
            2 => self.r2 |= src,
            3 => self.r3 |= src,
            _ => return Err(format!("Illicit destination value {}", dest))           
        }

        Ok(6)
    }

    /// performs logical XOR operation, storing result in `dest`
    fn xor_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;


        debug(format!("XOR r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 ^= o,
            1 => self.r1 ^= o,
            2 => self.r2 ^= o,
            3 => self.r3 ^= o,
            _ => return Err(format!("Illicit destination value {}", dest))            
        }

        Ok(2)
    }

    /// performs logical XOR operation, storing result in `dest`
    fn xor_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1];
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("XORI r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 ^= src,
            1 => self.r1 ^= src,
            2 => self.r2 ^= src,
            3 => self.r3 ^= src,
            _ => return Err(format!("Illicit destination value {}", dest))         
        }

        Ok(6)
    }

    /// compares two register values, storing success in
    fn cmp_reg(&mut self) -> Result<usize, String> {
        let test = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;


        debug(format!("CMP r{},r{}", test, src));
        
        let o = self.get_reg(src)? as i64;
        let t = self.get_reg(test)? as i64;
        let v = match t-o {
            0 => 1,
            _ => 0
        };
        self.fl |= v;

        Ok(2)
    }

    /// compares two register values, storing success in
    fn cmp_imm(&mut self) -> Result<usize, String> {
        let test = self.memory[self.pc + 1];
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("CMPI r{},0x{:x}", test, src));
        
        let t = self.get_reg(test)? as i64;
        let src = src as i64;
        debug(format!("\t{}-{}\n\n\n\n", t, src));
        let v = match t-src {
            0 => 1,
            _ => 0
        };
        self.fl |= v;
        Ok(6)
    }

    /// sets the flag value according to `val` and `flags`
    fn sfg_imm(&mut self) -> Result<usize, String> {
        let flag = self.memory[self.pc + 1];
        let val = self.memory[self.pc + 2];
        
        debug(format!("SFGI 0x{:x}, 0x{:x}", flag, val));
        self.fl ^= val << flag;
        Ok(3)
    }

    fn sfg_reg(&mut self) -> Result<usize, String> {
        let r = self.memory[self.pc + 1];
        let val = self.memory[self.pc + 2];
        let flag = self.get_reg(r)? as u8;

        debug(format!("SFGR 0x{:x}, 0x{:x}", flag, val));
        self.fl ^= val << flag;
        Ok(3)
    }

    /// loads a 32-bit value from `addr` into `dest`
    fn ld_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1]; 
        let addr = self.memory.get_u32(self.pc+2)? as usize;

        debug(format!("LOAD r{}, 0x{:x}", dest, addr));
        match dest {
            0 => self.r0 = self.memory.get_u32(addr).unwrap(),
            1 => self.r1 = self.memory.get_u32(addr).unwrap(),
            2 => self.r2 = self.memory.get_u32(addr).unwrap(),
            3 => self.r3 = self.memory.get_u32(addr).unwrap(),
            _ => return Err(format!("Illicit destination value {}", dest))          
        }

        Ok(6)
    }

    /// loads a 32-bit value from `src` into `dest`
    fn ld_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;

        let addr = self.get_reg(src)? as usize;

        debug(format!("LOAD r{}, r{}", dest, src));
        match dest {
            0 => self.r0 = self.memory.get_u32(addr).unwrap(),
            1 => self.r1 = self.memory.get_u32(addr).unwrap(),
            2 => self.r2 = self.memory.get_u32(addr).unwrap(),
            3 => self.r3 = self.memory.get_u32(addr).unwrap(),
            _ => return Err(format!("Illicit destination value {}", dest))          
        }

        Ok(2)
    }

    /// multiplies `dest` with `src`, storing in `dest`
    fn mul_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;


        debug(format!("MUL r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 *= o,
            1 => self.r1 *= o,
            2 => self.r2 *= o,
            3 => self.r3 *= o,
            _ => return Err(format!("Illicit destination value {}", dest))        
        }
        Ok(2)
    }

    /// multiplies `dest` with `src`, storing in `dest`
    fn mul_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1]; 
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("MULI r{},r{}", dest, src));
        match dest {
            0 => self.r0 *= src,
            1 => self.r1 *= src,
            2 => self.r2 *= src,
            3 => self.r3 *= src,
            _ => return Err(format!("Illicit destination value {}", dest))           
        }

        Ok(6)
    }

    /// subtracts `dest` with `src`, storing in `dest`
    fn sub_reg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;

        debug(format!("SUB r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 -= o,
            1 => self.r1 -= o,
            2 => self.r2 -= o,
            3 => self.r3 -= o,
            _ => return Err(format!("Illicit destination value {}", dest))          
        }

        Ok(2)
    }

    /// subtracts `dest` with `src`, storing in `dest`
    fn sub_imm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1]; 
        let src = self.memory.get_u32(self.pc + 2)?;

        debug(format!("SUB r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 -= src,
            1 => self.r1 -= src,
            2 => self.r2 -= src,
            3 => self.r3 -= src,
            _ => return Err(format!("Illicit destination value {}", dest))          
        }

        Ok(6)
    }

    /// moves value from `src` (register) into `dest` (register)
    fn mov_dreg_sreg(&mut self) -> Result<usize, String> {
        let dest = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let src = self.memory[self.pc + 1] & 0x0f;

        debug(format!("MOV r{},r{}", dest, src));
        let o = self.get_reg(src)?;
        match dest {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => return Err(format!("Illicit destination value {}", dest))           
        }

        Ok(2)
    }

    /// moves value from `src` (immediate) into `dest` (register)
    fn mov_dreg_simm(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1]; 
        let src = self.memory.get_u32(self.pc+2)?;

        debug(format!("MOVI r{},0x{:x}", dest, src));
        match dest {
            0 => self.r0 = src,
            1 => self.r1 = src,
            2 => self.r2 = src,
            3 => self.r3 = src,
            _ => return Err(format!("Illicit destination value {}", dest))         
        }

        Ok(6)
    }

    /// moves value from `src` (address) into `dest` (register)
    fn mov_dreg_saddr(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc+1]; 
        let src = self.memory.get_u32((self.pc+2) as usize)?;

        debug(format!("MOVA r{}, 0x{:x}", dest, src));
        match dest {
            0 => self.r0 = self.memory.get_u32(src as usize)?,
            1 => self.r1 = self.memory.get_u32(src as usize)?,
            2 => self.r2 = self.memory.get_u32(src as usize)?,
            3 => self.r3 = self.memory.get_u32(src as usize)?,
            _ => return Err(format!("Illicit destination value {}", dest))           
        }

        Ok(6)
    }

    /// moves value from `src` (register) into `dest` (address)
    fn mov_daddr_sreg(&mut self) -> Result<usize, String> {
        let dest = self.memory.get_u32((self.pc+1) as usize)?; 
        let src = self.memory[self.pc+5];

        debug(format!("MOVR 0x{:x},r{}", dest, src));
        let o = self.get_reg(src)?;
        self.memory.write_u32(dest as usize, o)?;
        Ok(6)
    }

    /// swaps `r1` and `r2`
    fn swp(&mut self) -> Result<usize, String> {
        let r1 = (self.memory[self.pc + 1] & 0xf0) >> 4; 
        let r2 = self.memory[self.pc + 1] & 0x0f;


        debug(format!("SWP r{},r{}", r1, r2));
        let o = self.get_reg(r2)?;
        let t = self.get_reg(r1)?;

        match r2 {
            0 => self.r0 = t,
            1 => self.r1 = t,
            2 => self.r2 = t,
            3 => self.r3 = t,
            _ => return Err(format!("Illicit destination value {}", r2))       
        }
        match r1 {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => return Err(format!("Illicit destination value {}", r1))           
        }

        Ok(2)
    }

    /// pushes `val` to the stack
    fn push_addr(&mut self) -> Result<usize, String> {
        let val = self.memory.get_u32(self.pc + 1)?;
        debug(format!("PUSHA 0x{:x}", val));

        self.sp += 4;
        self.memory.write_u32(self.sp, val)?;
        Ok(5)
    }
    
    /// pushes `val` to the stack
    fn push_reg(&mut self) -> Result<usize, String> {
        let reg = self.memory[self.pc+1];
        debug(format!("PUSH r{}", reg));
        
        let val = self.get_reg(reg)?;
        self.sp += 4;
        self.memory.write_u32(self.sp, val)?;
        Ok(2)
    }

    /// pops the top value from the stack into `dest`
    fn pop(&mut self) -> Result<usize, String> {
        let dest = self.memory[self.pc + 1];

        debug(format!("POP r{}", dest));

        let o = self.memory.get_u32(self.sp)?;
        self.sp -= 4;

        match dest {
            0 => self.r0 = o,
            1 => self.r1 = o,
            2 => self.r2 = o,
            3 => self.r3 = o,
            _ => return Err(format!("Illicit destination value {}", dest))      
        }

        Ok(2)
    }

    /// does nothing
    fn nop(&mut self) -> Result<usize, String> {
        debug(format!("NOP"));
        Ok(1)
    }

    /// performs a long jump 
    fn jmp_addr(&mut self) -> Result<usize, String> {
        let addr = self.memory.get_u32(self.pc+1)? as usize;
        debug(format!("JMPL 0x{}", addr));
        self.pc = addr;

        Ok(5)
    }
    
    /// performs a short jump to offset 
    fn jmp_imm(&mut self) -> Result<usize, String> {
        let a = self.memory.get_u32(self.pc+1)?;
        let short = convert_to_signed(a);
                

        debug(format!("JMPI 0x{:x}", short));
        if short < 0 {
            self.pc -= short.abs() as usize;
        } else {
            self.pc += short.abs() as usize;
        }

        Ok(5)
    }

    /// performs a jump to an offset stored in a register
    fn jmp_reg(&mut self) -> Result<usize, String> {
        let reg = self.memory[self.pc+1];

        debug(format!("JMP r{}", reg));
        self.pc = self.get_reg(reg)? as usize;

        Ok(1)
    }
    
    /// performs a jump to an offset stored in a register
    fn jeq_imm(&mut self) -> Result<usize, String> {
        let imm = self.memory.get_u32(self.pc+1)?;

        debug(format!("JEQI 0x{:x}", imm));
        if self.fl & 0x1 == 1 {
            self.fl &= 0xfe;
            self.pc = imm as usize;
        } else {
            self.pc += 5;
        }
        Ok(5)
    }

    /// performs a jump to an offset stored in a register
    fn jeq_reg(&mut self) -> Result<usize, String> {
        let reg = self.memory[self.pc+1];

        debug(format!("JEQ r{}", reg));
        let o = self.get_reg(reg)?;

        if self.fl & 0x1 == 1 {
            self.fl &= 0xfe;
            self.pc = o as usize;
        } else {
            self.pc += 2;
        }

        Ok(2)
    }

    /// handles an immediate interrupt
    fn int_imm(&mut self) -> Result<usize, String> {
        let code = self.memory.get_u32(self.pc+1)?;
        debug(format!("INTI 0x{:x}", code));
        self.handle_interrupt(code)
    }

    /// handles an interrupt in a register
    fn int_reg(&mut self) -> Result<usize, String> {
        let code = self.get_reg(self.memory[self.pc+1])?;
        debug(format!("INTR 0x{:x}", code));
        self.handle_interrupt(code)
    }

    /// handles interrupt codes
    fn handle_interrupt(&mut self, code: u32) -> Result<usize, String> {
        let int_func = match self.interrupt_table.get(&code) {
            Some(a) => a,
            None => return Err(format!("Unknown interrupt code {:x}", code))
        };

        int_func(self)
    }

    /// halt the program
    fn hlt(&mut self) -> ! {
        debug(format!("HLT"));
        std::process::exit(1);
    } 

}




