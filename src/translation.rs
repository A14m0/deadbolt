use std::{collections::HashMap};
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
    map.insert(0x8d, Instruction::Mov);
    map.insert(0x8e, Instruction::MovDregSaddr);
    map.insert(0x8f, Instruction::MovDaddrSreg);
    map.insert(0xb1, Instruction::Ld);
    map.insert(0xc5, Instruction::Swp);
    map.insert(0xd5, Instruction::PushAddr);
    map.insert(0xd6, Instruction::PushReg);
    map.insert(0xf1, Instruction::Pop);
    map.insert(0xff, Instruction::Nop);
    map.insert(0x6f, Instruction::Hlt);
    map.insert(0x81, Instruction::JmpAddr);
    map.insert(0x82, Instruction::JmpImm);
    map.insert(0xaa, Instruction::Int);
    
    map
}

pub fn build_compile_table() -> HashMap<Instruction, u8> {
    let mut map: HashMap<Instruction, u8> = HashMap::new();
    map.insert(Instruction::Add, 0x11);
    map.insert(Instruction::Sub, 0x02);
    map.insert(Instruction::Mul, 0x38);
    map.insert(Instruction::And, 0x41);
    map.insert(Instruction::Or, 0x56);
    map.insert(Instruction::Xor, 0x6a);
    map.insert(Instruction::Cmp, 0x79);
    map.insert(Instruction::Mov, 0x8d);
    map.insert(Instruction::MovDregSaddr, 0x8e);
    map.insert(Instruction::MovDaddrSreg, 0x8f);
    map.insert(Instruction::Ld, 0xb1);
    map.insert(Instruction::Swp, 0xc5);
    map.insert(Instruction::PushAddr, 0xd5);
    map.insert(Instruction::PushReg, 0xd6);
    map.insert(Instruction::Pop, 0xf1);
    map.insert(Instruction::Nop, 0xff);
    map.insert(Instruction::Hlt, 0x6f);
    map.insert(Instruction::JmpAddr, 0x81);
    map.insert(Instruction::JmpImm, 0x82);
    map.insert(Instruction::Int, 0xaa);

    
    map
}

pub fn build_decode_table() -> HashMap<&'static str, Instruction> {
    let mut map: HashMap<&'static str, Instruction> = HashMap::new();
    map.insert("add", Instruction::Add);
    map.insert("sub", Instruction::Sub);
    map.insert("mul", Instruction::Mul);
    map.insert("and", Instruction::And);
    map.insert("or", Instruction::Or);
    map.insert("xor", Instruction::Xor);
    map.insert("cmp", Instruction::Cmp);
    map.insert("mov", Instruction::Mov);
    map.insert("movr", Instruction::MovDregSaddr);
    map.insert("mova", Instruction::MovDaddrSreg);
    map.insert("ld", Instruction::Ld);
    map.insert("swp", Instruction::Swp);
    map.insert("pusha", Instruction::PushAddr);
    map.insert("pushr", Instruction::PushReg);
    map.insert("pop", Instruction::Pop);
    map.insert("nop", Instruction::Nop);
    map.insert("hlt", Instruction::Hlt);
    map.insert("jmpl", Instruction::JmpAddr);
    map.insert("jmp", Instruction::JmpImm);
    map.insert("int", Instruction::Int);

    
    map
}


pub fn encode_instruction(
    line: String, 
    ct: &HashMap<Instruction, u8>, 
    dt: &HashMap<&'static str, Instruction>,
    labels: &HashMap<String, u32>) -> Result<Vec<u32>, String> {
    
    let components: Vec<&str> = line.split(" ").collect();

    let decoded_inst = match dt.get(components[0]) {
        Some(a) => a,
        None => {
            if components[0] == "bytes" {
                let line = line[5..].to_string();
                let mut ret: Vec<u32> = Vec::new();
                let mut is_in_string: bool = false;
                
                for idx in (0..line.len()).step_by(4) {

                    // FIXME 
                    let b0:u8 = line[idx];
                    let b1:u8 = line[idx];
                    let b2:u8 = line[idx];
                    let b3:u8 = line[idx];

                    ret.push(
                        (b0 << 24) as u32 +
                        (b1 << 16) as u32 + 
                        (b2 << 8) as u32 + 
                        b3 as u32
                    );
                }
                return Ok(ret);
            }
            
            return Err(format!("Unknown instruction {}", components[0]));
        }
    };
    let oc = *(ct.get(decoded_inst).unwrap()) as u32;
            
    println!("{:?}", decoded_inst);
    
    let rt = match decoded_inst {
        Instruction::Add | Instruction::Sub | Instruction::Mul | 
        Instruction::And | Instruction::Or  | Instruction::Xor | 
        Instruction::Cmp | Instruction::Swp | Instruction::Mov => {
            let mut dest = components[1].to_string();
            dest.retain(|x| x != ',');
            let dest_byte = match reg_to_byte(&dest){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };
            let src_byte = match reg_to_byte(components[2]){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };

            // put it together
            (oc << 24) + (dest_byte << 16) + (src_byte << 8)
            
        },
        Instruction::PushReg | Instruction::Pop => {
            let dest_byte = match reg_to_byte(components[1]){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };
            
            // put it together
            (oc << 24) + (dest_byte << 16)
        },
        Instruction::MovDaddrSreg => {
            let dest: u32 = match labels.get(components[1]) {
                Some(a) => *a,
                None => {
                    match u32::from_str_radix(&components[1][2..components[1][2..].len()], 16){
                        Ok(a) => a,
                        Err(e) => return Err(format!("Failed to convert address {}: {}", &components[1][2..],  e))
                    }
                }
            };
            let src = match reg_to_byte(components[2]){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };

            println!("{:x} {:x} {:x}", oc << 24, dest << 16, src);
            (oc << 24) + ((dest << 16) & 0xFFFF00) + src
        }
        Instruction::MovDregSaddr => {
            let dest = match reg_to_byte(components[1]){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };
            let src: u32 = match labels.get(components[2]) {
                Some(a) => *a,
                None => {
                    match u32::from_str_radix(&components[2][2..], 16){
                        Ok(a) => a,
                        Err(e) => return Err(format!("Failed to convert address {}: {}", &components[2][2..], e))
                    }
                }
            };

            (oc << 24) + (dest << 16) + src
        } 
        Instruction::PushAddr | Instruction::JmpAddr => {
            let dest: u32 = match labels.get(components[1]) {
                Some(a) => *a,
                None => {
                    match u32::from_str_radix(&components[1][2..], 16){
                        Ok(a) => a,
                        Err(e) => return Err(format!("Failed to convert address: {}", e))
                    }
                }
            };

            // put it together
            (oc << 24) + ((dest << 16) & 0xFFFF00)
        },
        Instruction::Ld => {
            // format: inst reg, addr
            let mut dest = components[1].to_string();
            dest.retain(|x| x != ',');
            let dest_byte = match reg_to_byte(&dest){
                Ok(a) => a as u32,
                Err(e) => return Err(e)
            };
            let src: u32 = match labels.get(components[2]) {
                Some(a) => *a,
                None => match u32::from_str_radix(&components[2][2..], 16){
                    Ok(a) => a,
                    Err(e) => return Err(format!("Failed to convert address: {}", e))
                }
            };

            // put it together
            (oc << 24) + (dest_byte << 16) + src
        },
        Instruction::JmpImm => {
            let dest_byte = match labels.get(components[1]) {
                Some(a) => *a,
                None => {
                    match reg_to_byte(components[1]){
                        Ok(a) => a as u32,
                        Err(e) => return Err(e)
                    }
                }
            }; 
            
            
            
            (oc << 24) + (dest_byte << 16)
        },
        Instruction::Int => {
            let dest_byte = match u32::from_str_radix(&components[1][2..], 16){
                Ok(a) => a,
                Err(e) => return Err(format!("Failed to convert address: {}", e))
            };

            (oc << 24) + (dest_byte << 16)
        },
        Instruction::Hlt | Instruction::Nop => {
            oc << 24
        }
        
    };

    Ok(vec![rt])
}




/// converts an encoded u32 to a signed i32
pub fn convert_to_signed(a: u32) -> i32 {
    if a & 0x80000000 != 0 {
        return (a & 0x7FFFFFFF) as i32 * -1;
    } else {
        return (a & 0x7FFFFFFF) as i32;
    }
}

fn reg_to_byte(register: &str) -> Result<u8, String> {
    match &register[..2] {
        "r0" => Ok(0),
        "r1" => Ok(1),
        "r2" => Ok(2),
        "r3" => Ok(3),
        _ => Err(format!("Invalid register {}", register))
    }
}