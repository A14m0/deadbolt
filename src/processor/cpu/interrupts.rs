use std::collections::HashMap;
use std::io::Write;
use getch::Getch;

use super::CPU;
use crate::debug::debug;


pub trait Interrupt {
    fn run(r0: u32, r1: u32, r2: u32, r3: u32) -> Result<u32, String>;
}

pub type IntFn = fn(&mut CPU) -> Result<u32,String>;

pub fn build_interrupt_table() -> HashMap<u32, IntFn>{
    let mut map = HashMap::new();

    let int_writecon: IntFn = int_writeconsole;
    let int_readcon: IntFn = int_readconsole;
    map.insert(0x80u32, int_writecon);
    map.insert(0xa0u32, int_readcon);

    map
}


//// INTERRUPT 0x80: WRITE BYTE TO STDOUT ////
/// format for this is as follows:
/// 
/// R0      ->  Address of byte to write to console 
/// R1-R3   ->  Not used 
pub fn int_writeconsole(cpu: &mut CPU) -> Result<u32, String> {
    let o = cpu.memory[cpu.get_reg(0)? as usize] as char;
    debug(format!("INTERRUPTS: writing {}...", o));
    print!("{}", o);
    std::io::stdout().flush().unwrap();
    Ok(0)
}

//// INTERRUPT 0xA0: READ BYTE FROM STDIN ////
/// format for this is as follows:
/// 
/// R0      ->  Address where byte will be written 
/// R1      ->  Copy of byte read saved
/// R2-R3   ->  Not used
pub fn int_readconsole(cpu: &mut CPU) -> Result<u32, String> {
    //let o = cpu.memory[cpu.get_reg(0) as usize] as char;
    debug(format!("INTERRUPTS: Waiting for read..."));
    
    let g = Getch::new();
    let u = match g.getch() {
        Ok(a) => a,
        Err(e) => panic!("Interrupt Failed: {}", e)
    };

    if cpu.is_flag_set(1 << 4) {
        print!("{}", u as char);
        std::io::stdout().flush().unwrap();
        debug(format!("Flag IS set"));
    } else {
        debug(format!("Flag NOT set"));
    }
    
    debug(format!("INTERRUPTS: Read {:x}", u));
    let addr = cpu.get_reg(0)? as usize;
    cpu.memory[addr] = u;
    cpu.set_reg(1, u as u32)?;
    Ok(0)
}

