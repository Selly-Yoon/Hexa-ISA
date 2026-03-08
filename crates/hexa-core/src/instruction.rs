use crate::opcode::Opcode;
use crate::meta_field::MetaField;

#[derive(Debug, Clone)]
pub enum Operands {
    RdRs       { rd: u8, rs: u8 },
    RdRs1Rs2   { rd: u8, rs1: u8, rs2: u8 },
    RdPtrLen   { rd: u8, ptr: u64, len: u32 },
    R1R2       { r1: u8, r2: u8 },
    RdSt       { rd: u8, st: u8 },
    Imm(u64),
    None,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode:   Opcode,
    pub meta:     MetaField,
    pub operands: Operands,
}
