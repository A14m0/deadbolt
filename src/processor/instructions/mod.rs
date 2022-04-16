mod math_inst;
mod logical;
mod instruction_enum;

pub use instruction_enum::Instruction;

pub trait Instr {
    fn init() -> Self;
}