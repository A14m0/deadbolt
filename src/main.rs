mod processor;
mod compile;
mod translation;
mod debug;

use std::io::Read;
use std::path::PathBuf;

use clap::{Command, arg, value_parser, ArgAction};
use compile::compile;

fn main() {
    // parse command line arguments
    let matches = Command::new("DeadBolt")
                        .about("Compiler and emulator for the DeadBolt instruction set")
                        .subcommand(
                            Command::new("compile")
                                    .about("Compiles a program from source assembly file")
                                    .arg(arg!(-f --file <VALUE> "File to compile").required(false).value_parser(value_parser!(PathBuf))
                                    .action(ArgAction::Set))
                                    .arg(arg!(-o --output <VALUE> "Path to save binary to").required(false).value_parser(value_parser!(PathBuf))
                                    .action(ArgAction::Set)))
                        .subcommand(
                                Command::new("run")
                                    .about("Runs a binary")
                                    .arg(arg!(-i --input <VALUE> "Path to the binary to run").required(true).value_parser(value_parser!(PathBuf))
                                    .action(ArgAction::Set))
                        ).get_matches();

    // determine which subcommand we will be using
    if let Some(m) = matches.subcommand_matches("compile") {
        // compile the thing
        let path = m.get_one::<PathBuf>("file").unwrap().clone();
        let output = match m.get_one::<PathBuf>("output") {
            Some(a) => a.clone(),
            None => PathBuf::from("")
        };
        compile(path, output);
    } else if let Some(m) = matches.subcommand_matches("run") {
        // run program
        let path = m.get_one::<PathBuf>("input").unwrap();
        let mut f = std::fs::File::open(path).unwrap();
        let mut prog: Vec<u8> = Vec::new();
        f.read_to_end(&mut prog).unwrap();
        let mut proc = processor::cpu::CPU::init(prog);
        match proc.run() {
            Ok(_) => (),
            Err(e) => {
                println!("[ERROR] Encountered fatal error: {}\n{}", e, proc);
            }
        };
    } else {
        println!("No command provided. Use --help to see commands");

    }

    
}
