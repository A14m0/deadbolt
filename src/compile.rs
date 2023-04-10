//use crate::processor::cpu::CPU;
use crate::translation::{
    build_compile_table,
    build_decode_table, 
    encode_instruction,
    get_bytes_from_line
};

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Write};

use crate::{debug, info, error};





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



/// compiles a program in `prog`
pub fn compile(prog: PathBuf, output: PathBuf) {
    info!("Compiling {}...", prog.display());

    // read all of the data into a vector
    let file = match File::open(prog.clone()) {
        Ok(a) => a,
        Err(e) => {
            error!("Failed to open file {}: {}", prog.display(), e);
        }
    };
    let reader = BufReader::new(file);

    // initialize variables we need
    let compile_table = build_compile_table();
    let decode_table = build_decode_table();
    let mut sections: Vec<Section> = Vec::new();
    let mut output_bytes: Vec<u8> = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut prev_bytes: u32 = 0;
    


    // read file line by line
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut lineclone = line.clone();
        lineclone.retain(|x| !x.is_whitespace());

        debug!("Line {} - {} characters (prev bytes={})", index, line.len(), prev_bytes);
        if lineclone.len() == 0 {
            continue;
        }

        debug!("{}", line);

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
                            error!("No section declared before first instruction");
                        }, 
                        Some(a) => a
                    };
                    prev_bytes += get_bytes_from_line(&line, &decode_table);
                    section.push_line(line);
                }
            } else {
                // its not a label or section declaration
                let section = match sections.last_mut() {
                    None => {
                        error!("No section declared before first instruction");
                    }, 
                    Some(a) => a
                };

                prev_bytes += get_bytes_from_line(&line, &decode_table);
                section.push_line(line);
            }
        } 
    }

    // now that we have put the lines in their corresponding thing
    debug!("Sections currently parsed:");
    for m in sections.iter() {
        debug!("");
        debug!("{:?}", m);
    }
    debug!("Labels found: {:?}", labels);

    for m in sections.iter() {
        for l in m.get_lines() {
            let mut inst = match encode_instruction(l, &compile_table, &decode_table, &labels) {
                Ok(a) => a,
                Err(e) => {
                    error!("Failed to compile: {}", e);
                }
            };
            output_bytes.append(&mut inst);
            debug!("Length of output: {}", output_bytes.len());
        }
    }

    debug!("Output: ");
    debug!("{:?}", output_bytes);

    let mut fout: File;
    if output == PathBuf::from("") {
        fout = File::create("a.out".to_string()).unwrap();
    } else {
        fout = File::create(output).unwrap();
    }

    for entry in output_bytes.iter() {
        fout.write_all(&entry.to_be_bytes()).unwrap();
    }

    info!("Successfully wrote bytes to file");
    
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
