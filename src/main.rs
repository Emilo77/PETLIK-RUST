use std::io;
use std::io::Read;

type VarT = char;
type AddrT = u32;

struct InstrContainer {
    instructions : Vec<InstructionType>
}

impl InstrContainer {
    fn new() -> InstrContainer {
        InstrContainer {
            instructions : vec![]
        }
    }

    fn get_data() {
        let mut input : Vec<u8> = vec![];
        io::stdin().read_to_end(&mut input).expect("Could not read data");
        println!("{}", String::from(input));
    }

    fn add(&mut self, instr : InstructionType) -> usize {
        self.instructions.push(instr);
        self.instructions.len() - 1
    }
}

enum InstructionType {
    INC (VarT),
    ADD (VarT, VarT),
    CLR (VarT),
    JMP (AddrT),
    DJZ (VarT, AddrT),
    PRT (VarT),
    HLT
}




fn main() {
    let container = InstrContainer::new();


}
