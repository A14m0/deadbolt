use std::collections::HashMap;
use std::io::Write;

use super::CPU;
use crate::debug::debug;


pub trait Interrupt {
    fn run(r0: u32, r1: u32, r2: u32, r3: u32) -> Result<u32, String>;
}

pub type IntFn = fn(&mut CPU) -> Result<u32,String>;

pub fn build_interrupt_table() -> HashMap<u32, IntFn>{
    let mut map = HashMap::new();

    let int_writecon: IntFn = int_writeconsole;
    map.insert(0x80u32, int_writecon);

    map
}


//// INTERRUPT 0x80: WRITE BYTE TO STDOUT ////
/// format for this is as follows:
/// 
/// R0      ->  Address of byte to write to console 
/// R1-R3   ->  
pub fn int_writeconsole(cpu: &mut CPU) -> Result<u32, String> {
    let o = cpu.memory[cpu.get_reg(0) as usize] as char;
    debug(format!("INTERRUPTS: writing {}...", o));
    print!("{}", o);
    std::io::stdout().flush().unwrap();
    Ok(0)
}
