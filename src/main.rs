mod processor;
mod compile;
mod translation;
mod debug;

use std::io::Read;

use clap::{App, Arg, SubCommand, crate_version, crate_authors};
use compile::compile;

fn main() {
    // parse command line arguments
    let matches = App::new("DeadBolt")
                        .version(crate_version!())
                        .author(crate_authors!())
                        .about("Compiler and emulator for the DeadBolt instruction set")
                        .subcommands(
                            vec![
                                SubCommand::with_name("compile")
                                    .about("Compiles a program from source assembly file")
                                    .arg(Arg::with_name("file")
                                        .long("file")
                                        .short("f")
                                        .takes_value(true)
                                        .required(true)
                                        .help("File to compile")
                                    )
                                    .arg(Arg::with_name("output")
                                        .long("output")
                                        .short("o")
                                        .takes_value(true)
                                        .required(false)
                                        .help("Path to save binary to")
                                    ),
                                SubCommand::with_name("run")
                                    .about("Runs a binary")
                                    .arg(Arg::with_name("input")
                                        .long("input")
                                        .short("i")
                                        .takes_value(true)
                                        .required(true)
                                        .help("Path to the binary to run"))
                            ]
                        ).get_matches();

    // determine which subcommand we will be using
    if let Some(m) = matches.subcommand_matches("compile") {
        // compile the thing
        let path = m.value_of("file").unwrap().to_string();
        let output = match m.value_of("output") {
            Some(a) => a.to_string(),
            None => "".to_string()
        };
        compile(path, output);
    } else if let Some(m) = matches.subcommand_matches("run") {
        // run program
        let path = m.value_of("input").unwrap().to_string();
        let mut f = std::fs::File::open(path).unwrap();
        let mut prog: Vec<u8> = Vec::new();
        f.read_to_end(&mut prog).unwrap();
        let mut proc = processor::cpu::CPU::init(prog);
        match proc.run() {
            Ok(_) => (),
            Err(e) => {
                println!("[ERROR] Encountered fatal error: {}", e);
            }
        };
    } else {
        println!("No command provided. Use --help to see commands");

    }

    
}
