pub enum InstructionT {
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MultInt,
    MultFloat,
    DivInt,
    DivFloat,
    Equal,
    Gthan,
    Lthan,
    GthanEq,
    LthanEq,
    NoEq,
}

pub struct Instruction {
    arguments: Vec<u8>, //pointers to memory in the machine
    t: InstructionT,    //the type of instruction
}

impl Instruction {
    pub fn new(arguments: Vec<u8>, t: InstructionT) -> Self {
        Self { arguments, t }
    }
}
