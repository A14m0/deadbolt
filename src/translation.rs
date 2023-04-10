use std::{collections::HashMap};
use regex::Regex;
use crate::processor::instructions::Instruction;
use crate::debug;

pub fn build_translation_table() -> HashMap<u8, Instruction> {
    let mut map: HashMap<u8, Instruction> = HashMap::new();
    map.insert(0x11, Instruction::AddReg);
    map.insert(0x12, Instruction::AddImm);
    map.insert(0x02, Instruction::SubReg);
    map.insert(0x03, Instruction::SubImm);
    map.insert(0x38, Instruction::MulReg);
    map.insert(0x39, Instruction::MulImm);
    map.insert(0x41, Instruction::AndReg);
    map.insert(0x42, Instruction::AndImm);
    map.insert(0x56, Instruction::OrReg);
    map.insert(0x57, Instruction::OrImm);
    map.insert(0x6a, Instruction::XorReg);
    map.insert(0x6b, Instruction::XorImm);
    map.insert(0x79, Instruction::CmpReg);
    map.insert(0x80, Instruction::CmpImm);
    map.insert(0x8d, Instruction::MovDregSreg);
    map.insert(0x8e, Instruction::MovDregSaddr);
    map.insert(0x8f, Instruction::MovDaddrSreg);
    map.insert(0x90, Instruction::MovDregSimm);
    map.insert(0xb1, Instruction::LdImm);
    map.insert(0xb2, Instruction::LdReg);
    map.insert(0xc5, Instruction::Swp);
    map.insert(0xd5, Instruction::PushAddr);
    map.insert(0xd6, Instruction::PushReg);
    map.insert(0xf0, Instruction::SfgReg);
    map.insert(0xf1, Instruction::SfgImm);
    map.insert(0xf2, Instruction::Pop);
    map.insert(0xff, Instruction::Nop);
    map.insert(0x6f, Instruction::Hlt);
    map.insert(0x81, Instruction::JmpAddr);
    map.insert(0x82, Instruction::JmpImm);
    map.insert(0x83, Instruction::JmpReg);
    map.insert(0x84, Instruction::JeqImm);
    map.insert(0x85, Instruction::JeqReg);
    map.insert(0xaa, Instruction::IntImm);
    map.insert(0xab, Instruction::IntReg);
    
    map
}

pub fn build_compile_table() -> HashMap<Instruction, u8> {
    let mut map: HashMap<Instruction, u8> = HashMap::new();
    map.insert(Instruction::AddReg, 0x11);
    map.insert(Instruction::AddImm, 0x12);
    map.insert(Instruction::SubReg, 0x02);
    map.insert(Instruction::SubImm, 0x03);
    map.insert(Instruction::MulReg, 0x38);
    map.insert(Instruction::MulImm, 0x39);
    map.insert(Instruction::AndReg, 0x41);
    map.insert(Instruction::AndImm, 0x42);
    map.insert(Instruction::OrReg, 0x56);
    map.insert(Instruction::OrImm, 0x57);
    map.insert(Instruction::XorReg, 0x6a);
    map.insert(Instruction::XorImm, 0x6b);
    map.insert(Instruction::CmpReg, 0x79);
    map.insert(Instruction::CmpImm, 0x80);
    map.insert(Instruction::MovDregSreg, 0x8d);
    map.insert(Instruction::MovDregSaddr, 0x8e);
    map.insert(Instruction::MovDaddrSreg, 0x8f);
    map.insert(Instruction::MovDregSimm, 0x90);
    map.insert(Instruction::LdImm, 0xb1);
    map.insert(Instruction::LdReg, 0xb2);
    map.insert(Instruction::Swp, 0xc5);
    map.insert(Instruction::PushAddr, 0xd5);
    map.insert(Instruction::PushReg, 0xd6);
    map.insert(Instruction::SfgReg, 0xf0);
    map.insert(Instruction::SfgImm, 0xf1);
    map.insert(Instruction::Pop, 0xf2);
    map.insert(Instruction::Nop, 0xff);
    map.insert(Instruction::Hlt, 0x6f);
    map.insert(Instruction::JmpAddr, 0x81);
    map.insert(Instruction::JmpImm, 0x82);
    map.insert(Instruction::JmpReg, 0x83);
    map.insert(Instruction::JeqImm, 0x84);
    map.insert(Instruction::JeqReg, 0x85);
    map.insert(Instruction::IntImm, 0xaa);
    map.insert(Instruction::IntReg, 0xab);
    
    
    map
}

pub fn build_decode_table() -> HashMap<&'static str, Instruction> {
    let mut map: HashMap<&'static str, Instruction> = HashMap::new();
    map.insert("add", Instruction::AddReg);
    map.insert("addi", Instruction::AddImm);
    map.insert("sub", Instruction::SubReg);
    map.insert("subi", Instruction::SubImm);
    map.insert("mul", Instruction::MulReg);
    map.insert("muli", Instruction::MulImm);
    map.insert("and", Instruction::AndReg);
    map.insert("andi", Instruction::AndImm);
    map.insert("or", Instruction::OrReg);
    map.insert("ori", Instruction::OrImm);
    map.insert("xor", Instruction::XorReg);
    map.insert("xori", Instruction::XorImm);
    map.insert("cmp", Instruction::CmpReg);
    map.insert("cmpi", Instruction::CmpImm);
    map.insert("mov", Instruction::MovDregSreg);
    map.insert("movi", Instruction::MovDregSimm);
    map.insert("mova", Instruction::MovDregSaddr);
    map.insert("movr", Instruction::MovDaddrSreg);
    map.insert("ldi", Instruction::LdImm);
    map.insert("ldr", Instruction::LdReg);
    map.insert("swp", Instruction::Swp);
    map.insert("pusha", Instruction::PushAddr);
    map.insert("push", Instruction::PushReg);
    map.insert("sfgr", Instruction::SfgReg);
    map.insert("sfgi", Instruction::SfgImm);
    map.insert("pop", Instruction::Pop);
    map.insert("nop", Instruction::Nop);
    map.insert("hlt", Instruction::Hlt);
    map.insert("jmpl", Instruction::JmpAddr);
    map.insert("jmpi", Instruction::JmpImm);
    map.insert("jmp", Instruction::JmpReg);
    map.insert("jeq", Instruction::JeqReg);
    map.insert("jeqi", Instruction::JeqImm);
    map.insert("int", Instruction::IntImm);
    map.insert("intr", Instruction::IntReg);

    
    map
}


pub fn get_bytes_from_line(
    line: &String,
    dt: &HashMap<&'static str, Instruction>,
) -> u32 {
    let components: Vec<&str> = line.split(" ").collect();

    let decode_inst = match dt.get(components[0]) {
        Some(a) => a,
        None => {
            if components[0] == "bytes" {
                let line = &line[5..];
                let mut ret: Vec<u8> = Vec::new();
                let str_check = Regex::new("\"(.*?)\"").unwrap();
                let byte_check = Regex::new("0[xX][0-9a-fA-F]+").unwrap();

                // look for strings
                for str_match in str_check.captures_iter(line) {
                    let str_raw = &str_match[1];

                    for c in str_raw.as_bytes() {
                        ret.push(c.clone());
                    }
                }

                debug!("Bytes");
                // look for bytes
                for byte_match in byte_check.captures_iter(line) {
                    debug!("{:?}", &byte_match[0]);
                    let byte = &byte_match[0][1..byte_match[0].len()-1];
                    let mut ret_byte = match u32::from_str_radix(byte, 16) {
                        Ok(a) => a.to_be_bytes().to_vec(),
                        Err(_e) => {
                            vec![0u8]
                        }
                    };
                    ret.append(&mut ret_byte);
                }

                return ret.len() as u32;
            }
            return 0;
        }
    };

    match decode_inst {
        Instruction::AddReg | Instruction::SubReg | Instruction::MulReg | 
        Instruction::AndReg | Instruction::OrReg  | Instruction::XorReg | 
        Instruction::CmpReg | Instruction::Swp | Instruction::MovDregSreg|
        Instruction::LdReg => 2,
        Instruction::AddImm | Instruction::SubImm | Instruction::MulImm | 
        Instruction::AndImm | Instruction::OrImm  | Instruction::XorImm | 
        Instruction::CmpImm | Instruction::MovDregSaddr | Instruction::MovDregSimm | Instruction::LdImm | Instruction::SfgReg => 6,
        Instruction::PushReg | Instruction::Pop | 
        Instruction::JmpReg | Instruction::IntReg | Instruction::JeqReg => 2,
        Instruction::MovDaddrSreg => 6,
        Instruction::PushAddr | Instruction::JmpAddr | Instruction::JeqImm | Instruction::JmpImm | Instruction::IntImm => 5,
        Instruction::SfgImm => 3,
        Instruction::Hlt | Instruction::Nop => 1   
    }
}

pub fn encode_instruction(
    line: String, 
    ct: &HashMap<Instruction, u8>, 
    dt: &HashMap<&'static str, Instruction>,
    labels: &HashMap<String, u32>
) -> Result<Vec<u8>, String> {
    
    let components: Vec<&str> = line.split(" ").collect();

    let decoded_inst = match dt.get(components[0]) {
        Some(a) => a,
        None => {
            if components[0] == "bytes" {
                let line = &line[5..];
                let mut ret: Vec<u8> = Vec::new();
                let str_check = Regex::new("\"(.*?)\"").unwrap();
                let byte_check = Regex::new("0[xX][0-9a-fA-F]+").unwrap();

                // look for strings
                for str_match in str_check.captures_iter(line) {
                    let str_raw = &str_match[1];

                    for c in str_raw.as_bytes() {
                        ret.push(c.clone());
                    }
                }

                debug!("Bytes");
                // look for bytes
                for byte_match in byte_check.captures_iter(line) {
                    debug!("{:?}", &byte_match[0]);
                    let byte = &byte_match[0][1..byte_match[0].len()-1];
                    let mut ret_byte = match u32::from_str_radix(byte, 16) {
                        Ok(a) => a.to_be_bytes().to_vec(),
                        Err(_e) => {
                            vec![0u8]
                        }
                    };
                    ret.append(&mut ret_byte);
                }

                return Ok(ret);
            }
            
            return Err(format!("Unknown instruction {}", components[0]));
        }
    };
    let oc = *(ct.get(decoded_inst).unwrap());
            
    debug!("{:?}", decoded_inst);
    
    let rt = match decoded_inst {
        Instruction::AddReg | Instruction::SubReg | Instruction::MulReg | 
        Instruction::AndReg | Instruction::OrReg  | Instruction::XorReg | 
        Instruction::CmpReg | Instruction::Swp | Instruction::MovDregSreg|
        Instruction::LdReg => {
            // format: inst REG, REG
            let mut dest = components[1].to_string();
            dest.retain(|x| x != ',');
            let dest_byte = match reg_to_byte(&dest){
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            let src_byte = match reg_to_byte(components[2]){
                Ok(a) => a,
                Err(e) => return Err(e)
            };

            let regs = (dest_byte << 4) + src_byte;
            
            let mut ret = Vec::new();
            ret.push(oc);
            ret.push(regs);
            
            ret
            
        },
        Instruction::AddImm | Instruction::SubImm | Instruction::MulImm | 
        Instruction::AndImm | Instruction::OrImm  | Instruction::XorImm | 
        Instruction::CmpImm | Instruction::MovDregSaddr | Instruction::MovDregSimm | Instruction::LdImm | Instruction::SfgReg => {
            // format: inst REG, IMM 
            let mut dest = components[1].to_string();
            dest.retain(|x| x != ',');
            let dest_byte = match reg_to_byte(&dest){
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            let mut src_byte = match labels.get(components[2]) {
                Some(a) => a.to_be_bytes().to_vec(),
                None => {
                    debug!("{}", &components[2][2..]);
                    match u32::from_str_radix(&components[2][2..], 16){
                        Ok(a) => a.to_be_bytes().to_vec(),
                        Err(e) => return Err(format!("Failed to convert address {}: {}", &components[2][2..],  e))
                    }
                }
            };

            // put it together
            let mut ret = Vec::new();
            ret.push(oc);
            ret.push(dest_byte);
            ret.append(&mut src_byte);
            ret
        },
        
        Instruction::PushReg | Instruction::Pop | 
        Instruction::JmpReg | Instruction::IntReg | Instruction::JeqReg => {
            // format: inst REG
            let dest_byte = match reg_to_byte(components[1]){
                Ok(a) => a,
                Err(e) => return Err(e)
            };

            let mut ret = Vec::new();
            ret.push(oc);
            ret.push(dest_byte);
            ret
        },
        Instruction::MovDaddrSreg => {
            // format: inst ADDR, REG
            let mut dest = match labels.get(components[1]) {
                Some(a) => a.to_be_bytes().to_vec(),
                None => {
                    match u32::from_str_radix(&components[1][2..components[1][2..].len()], 16){
                        Ok(a) => a.to_be_bytes().to_vec(),
                        Err(e) => return Err(format!("Failed to convert address {}: {}", &components[1][2..],  e))
                    }
                }
            };
            let src = match reg_to_byte(components[2]){
                Ok(a) => a,
                Err(e) => return Err(e)
            };

            let mut ret = Vec::new();
            ret.push(oc);
            ret.append(&mut dest);
            ret.push(src);
            ret
        }
        /*Instruction::MovDregSaddr | Instruction::MovDregSimm => {
            // format: inst REG, ADDR
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

            (oc << 24) + (dest << 16) + (src & 0xFFFF)
        } */
        Instruction::PushAddr | Instruction::JmpAddr | Instruction::JeqImm | Instruction::JmpImm | Instruction::IntImm => {
            // format: inst ADDR
            let mut dest = match labels.get(components[1]) {
                Some(a) => a.to_be_bytes().to_vec(),
                None => {
                    match u32::from_str_radix(&components[1][2..], 16){
                        Ok(a) => a.to_be_bytes().to_vec(),
                        Err(e) => return Err(format!("Failed to convert address: {}", e))
                    }
                }
            };

            
            let mut ret = Vec::new();
            ret.push(oc);
            ret.append(&mut dest);
            ret
        },
        /*Instruction::LdImm => {
            // format: inst REG, ADDR
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
        },*/
        /*Instruction::JmpImm => {
            // format: inst ADDR
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
        },*/
        /*Instruction::IntImm => {
            // format: inst ADDR
            let dest_byte = match u32::from_str_radix(&components[1][2..], 16){
                Ok(a) => a,
                Err(e) => return Err(format!("Failed to convert address: {}", e))
            };

            (oc << 24) + (dest_byte << 16)
        },*/
        Instruction::SfgImm => {
            // format: inst BYTE ADDR
            let dest: u8 = match labels.get(components[1]) {
                Some(a) => (*a).try_into().unwrap(),
                None => {
                    match u32::from_str_radix(&components[1][2..components[1].len()-1], 16){
                        Ok(a) => a.try_into().unwrap(),
                        Err(e) => return Err(format!("Failed to convert address: {}", e))
                    }
                }
            };

            let src: u8 = match labels.get(components[2]) {
                Some(a) => (*a).try_into().unwrap(),
                None => {
                    match u32::from_str_radix(&components[2][2..], 16){
                        Ok(a) => a.try_into().unwrap(),
                        Err(e) => return Err(format!("Failed to convert address: {}", e))
                    }
                }
            };
            let mut ret = Vec::new();
            ret.push(oc);
            ret.push(dest);
            ret.push(src);
            ret
        },
        Instruction::Hlt | Instruction::Nop => {
            // format: inst
            let mut ret = Vec::new();
            ret.push(oc);
            ret
        }
        
    };

    Ok(rt)
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