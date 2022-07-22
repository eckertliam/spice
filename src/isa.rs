use std::collections::HashMap;

// machine instruction types in human readable form
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum InstructionT {
    End,
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MultInt,
    MultFloat,
    DivInt,
    DivFloat,
    EqualInt,
    EqualFloat,
    GthanInt,
    GthanFloat,
    LthanInt,
    LthanFloat,
    GthanEqInt,
    GthanEqFloat,
    LthanEqInt,
    LthanEqFloat,
    NoEqInt,
    NoEqFloat,
    EqStr,
    EqChar,
    NoEqStr,
    NoEqChar,
    Error,
}

const MAP_SIZE: usize = 26;

// instead of writing out huge match statements each instructions opcode is its index
const OPCODE_MAP: [InstructionT; MAP_SIZE] = [
    InstructionT::End,
    InstructionT::AddInt,
    InstructionT::AddFloat,
    InstructionT::SubInt,
    InstructionT::SubFloat,
    InstructionT::MultInt,
    InstructionT::MultFloat,
    InstructionT::DivInt,
    InstructionT::DivFloat,
    InstructionT::EqualInt,
    InstructionT::EqualFloat,
    InstructionT::GthanInt,
    InstructionT::GthanFloat,
    InstructionT::LthanInt,
    InstructionT::LthanFloat,
    InstructionT::GthanEqInt,
    InstructionT::GthanEqFloat,
    InstructionT::LthanEqInt,
    InstructionT::LthanEqFloat,
    InstructionT::NoEqInt,
    InstructionT::NoEqFloat,
    InstructionT::EqStr,
    InstructionT::EqChar,
    InstructionT::NoEqStr,
    InstructionT::NoEqChar,
    InstructionT::Error,
];

impl InstructionT {
    // if the array contains the op
    pub fn from_opcode(op: u8) -> Self {
        if op < MAP_SIZE as u8 {
            return OPCODE_MAP[op as usize];
        } else {
            return Self::Error;
        }
    }

    pub fn to_opcode(&self) -> u8 {
        let op = OPCODE_MAP.iter().position(|&t| &t == self);
        match op {
            Some(r) => r as u8,
            None => 255, //error code
        }
    }
}

// Instruction type and the arguments attached
pub struct Instruction {
    arguments: Vec<u8>, //pointers to memory in the machine
    t: InstructionT,    //the type of instruction
}

impl Instruction {
    pub fn new(arguments: Vec<u8>, t: InstructionT) -> Self {
        Self { arguments, t }
    }
}
