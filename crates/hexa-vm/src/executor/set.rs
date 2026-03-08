use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    match (&inst.opcode, &inst.operands) {
        (Opcode::SetSub, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if (a & b) == a { 1 } else { 0 };
        }
        (Opcode::SetSup, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if (a & b) == b { 1 } else { 0 };
        }
        (Opcode::SetInt, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            regs.general[*rd as usize] =
                regs.general[*rs1 as usize] & regs.general[*rs2 as usize];
        }
        (Opcode::SetUni, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            regs.general[*rd as usize] =
                regs.general[*rs1 as usize] | regs.general[*rs2 as usize];
        }
        (Opcode::SetDif, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = a & (!b);
        }
        (Opcode::SetSym, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            regs.general[*rd as usize] =
                regs.general[*rs1 as usize] ^ regs.general[*rs2 as usize];
        }
        (Opcode::SetMem, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let el  = regs.general[*rs1 as usize];
            let set = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if (set >> el) & 1 == 1 { 1 } else { 0 };
        }
        _ => {}
    }
}
