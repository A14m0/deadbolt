use std::collections::HashMap;
use crate::processor::Instruction;

pub fn build_translation_table() -> HashMap<u8, Instruction> {
    let mut map: HashMap<u8, Instruction> = HashMap::new();
    map.insert(0x11, Instruction::Add);
    map.insert(0x02, Instruction::Sub);
    map.insert(0x38, Instruction::Mul);

    map.insert(0x41, Instruction::And);
    map.insert(0x56, Instruction::Or);
    map.insert(0x6a, Instruction::Xor);
    map.insert(0x79, Instruction::Cmp);
    map.insert(0x8e, Instruction::MovDregSaddr);
    map.insert(0x8f, Instruction::MovDaddrSreg);

    map.insert(0xa4, Instruction::Sfg);
    map.insert(0xb1, Instruction::Ld);
    map.insert(0xc5, Instruction::Swp);
    map.insert(0xd6, Instruction::Push);
    map.insert(0xf1, Instruction::Pop);

    map.insert(0xff, Instruction::Nop);
    map.insert(0x6f, Instruction::Hlt);
    map.insert(0x81, Instruction::JmpAddr);
    map.insert(0x82, Instruction::JmpImm);

    
    map
}

pub fn convert_to_signed(a: u32) -> i32 {
    let ret: i16;
    if a & 0x80000000 != 0 {
        return (a & 0x7FFFFFFF) as i32 * -1;
    } else {
        return (a & 0x7FFFFFFF) as i32;
    }
}

