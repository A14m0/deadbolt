mod processor;
mod compile;
mod translation;


fn main() {
    let prog = vec![0x6a010100,0x6f000000];
    let mut proc = processor::CPU::init(prog);
    proc.run();
}
