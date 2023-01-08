# Deadbolt
Software processor and assembler written in Rust

This was a project I wrote at the end of my operating systems class mostly 
because I had nothing better to waste my time with :)

There are a few things to keep in mind:
1. This uses a custom assembly language, which I have not well documented (yet!). The instructions and their basic operations are shown in [ASM.md](/ASM.md). Some example files are provided in the repository (see [hello_world.dba](/hello_world.dba) and [example.dba](/example.dba))
2. This was written without the intention of it being actually useful for anything, so it is a little cumbersome :)

## Prerequisites
In order to run the software, you need a Rust toolchain and Cargo installed. All
other dependencies should be taken care of by Cargo :)

## Compiling Assembly Files
As stated above, this processor uses a custom assembly language at its core. To 
see the proper format for assembly files, see 
[the Assembly Format section](# Assembly Format). To compile assembly files, 
simply run the following command.

```sh
cargo run --release -- compile -f input_file.dba -o output_executable.bin
```

Once it successfully builds, it will save the executable in a format that is 
readable by the processor. Errors will *not* be as nice to read as Cargo's so be
aware that it will probably just dump some really obscure thing on you when it 
doesn't understand what you are doing :)

## Running Binaries
To run compiled programs, run the following.

```sh
cargo run --release -- run -i output_executable.bin
```

The software will then execute the program!



# Assembly Format
The format for writing assembly for the program is similar to that of other x86 
assemblers, particularly sharing some stylings of Intel syntax x86. Sections are 
denoted by the `section` keyword, variables and named references start with a 
`.`, comments are started with a `;`. The assembly language has different 
instructions for the same operation but different parameter sets. For example, 
you can add a register to another using the `add` instruction, but if you wanted 
to add the immediate value `0x1`, you would use the `addi` instruction.

The architecture also implements an interrupt system. As of now, the only 
supported interrupt numbers are `int_readcon` and `int_writecon`, to read and
write data to the console respectively. 

All assembly files must have a `.text` section, where instructions will be 
stored. Other sections can be included if so inclined.