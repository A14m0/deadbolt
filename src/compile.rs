//use crate::processor::cpu::CPU;
use crate::translation::{
    build_compile_table,
    build_decode_table, 
    encode_instruction 
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use colored::*;

use crate::debug::debug;


/// enum for the status of the compiler print
#[derive(Clone, Copy, Debug)]
enum Status {
    Info,
    //Warning,
    Error
}


#[derive(Clone, Debug)]
struct Section {
    _name: String,
    lines: Vec<String>
}

impl Section {
    fn new(_name: String) -> Self{
        Section { _name, lines: Vec::new() }
    }
    fn push_line(&mut self, line: String) {
        self.lines.push(line);
    }
    fn get_lines(&self) -> Vec<String>{
        self.lines.clone()
    }
}



/// color prints with prefix and whatnot
fn status(s: Status, msg: String) {
    match s {
        Status::Info => {
            println!("{} - {}", "[INFO]".green(), msg)
        },
        /*Status::Warning => {
            println!("{} - {}", "[WARN]".yellow(), msg)
        },*/
        Status::Error =>  {
            println!("{} - {}", "[FAIL]".red(), msg)
        }
    }
}


/// compiles a program in `prog`
pub fn compile(prog: String, output: String) {
    status(Status::Info, format!("Compiling {}...", prog));

    // read all of the data into a vector
    let file = match File::open(prog.clone()) {
        Ok(a) => a,
        Err(e) => {
            status(Status::Error, format!("Failed to open file {}: {}", prog, e));
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    // initialize variables we need
    let compile_table = build_compile_table();
    let decode_table = build_decode_table();
    let mut sections: Vec<Section> = Vec::new();
    let mut output_bytes: Vec<u32> = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut prev_bytes: u32 = 0;
    


    // read file line by line
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut lineclone = line.clone();
        lineclone.retain(|x| !x.is_whitespace());

        debug(format!("Line {} - {} characters (prev bytes={})", index, line.len(), prev_bytes));
        if lineclone.len() == 0 {
            continue;
        }

        println!("{}", line);

        // remove comment lines
        if lineclone.chars().next().unwrap() != ';' {
            // see if the line contains a label or section declaration
            if line.contains('.') {
                let thing: Vec<&str> = line.split(';').collect();
                let mut command = thing[0].clone().to_string();
                command.retain(|x| !x.is_whitespace());
            
                // if its a section, save it
                if command.contains("section") {
                    let thing2: String = command.split("section").collect::<Vec<&str>>()[1].clone().to_string();
                    sections.push(Section::new(thing2));
                } else if line.chars().next().unwrap() == '.' {
                    // its a lable so push it to the labels HashMap
                    command.retain(|x| !x.is_whitespace());
                
                    labels.insert(command, prev_bytes); /*match ((index*4-prev_bytes)).try_into() {
                        Ok(a) => a,
                        Err(e) => {
                            status(Status::Error, format!("Failed to parse address: {}", e));
                            std::process::exit(1);
                        }
                    } */

                } else {
                    // its a line that contains a label, but doesnt define one
                    let section = match sections.last_mut() {
                        None => {
                            status(Status::Error, format!("No section declared before first instruction"));
                            std::process::exit(1);
                        }, 
                        Some(a) => a
                    };
                    section.push_line(line);
                    prev_bytes += 4;
                }
            } else {
                // its not a label or section declaration
                let section = match sections.last_mut() {
                    None => {
                        status(Status::Error, format!("No section declared before first instruction"));
                        std::process::exit(1);
                    }, 
                    Some(a) => a
                };
                section.push_line(line);
                prev_bytes += 4;
            }
        } 
    }

    // now that we have put the lines in their corresponding thing
    debug(format!("Sections currently parsed:"));
    for m in sections.iter() {
        debug(format!(""));
        debug(format!("{:?}", m));
    }
    debug(format!("Labels found: {:?}", labels));

    for m in sections.iter() {
        for l in m.get_lines() {
            let mut inst = match encode_instruction(l, &compile_table, &decode_table, &labels) {
                Ok(a) => a,
                Err(e) => {
                    status(Status::Error, format!("Failed to compile: {}", e));
                    std::process::exit(1);
                }
            };
            output_bytes.append(&mut inst);
        }
    }

    debug(format!("Output: "));
    debug(format!("{:?}", output_bytes));

    let mut fout: File;
    if output == "" {
        fout = File::create("a.out".to_string()).unwrap();
    } else {
        fout = File::create(output).unwrap();
    }

    for entry in output_bytes.iter() {
        fout.write_all(&entry.to_be_bytes()).unwrap();
    }

    status(Status::Info, format!("Successfully wrote bytes to file"));
    
}



/*
Things we need:
1. array of each line raw
2. array of .text data
3. array of .data data 
4. array of labels
5. output buffer
6. translation HashMap

Process:
1. Remove all comment lines
2. Find and store labels
    a. Go through each line
    b. When a label is found, remove its index from the list of lines and store the index in a HashMap 
3. replace all instances of a label with an address offset
4. Make sure both sections are available
5. Separate lines into each section's list of stuff
6. Translate .text section as normal





*/
