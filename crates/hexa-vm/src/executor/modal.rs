use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    match (&inst.opcode, &inst.operands) {
        (Opcode::Nec, Operands::RdRs { rd, rs }) => {
            regs.general[*rd as usize] = if regs.general[*rs as usize] != 0 { 1 } else { 0 };
        }
        (Opcode::Pos, Operands::RdRs { rd, rs }) => {
            regs.general[*rd as usize] = if regs.general[*rs as usize] != 0 { 1 } else { 0 };
        }
        (Opcode::Taut, Operands::RdRs { rd, .. }) => {
            regs.general[*rd as usize] = 1;
        }
        (Opcode::Cont, Operands::RdRs { rd, .. }) => {
            regs.general[*rd as usize] = 0;
        }
        (Opcode::ModAnd, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            regs.general[*rd as usize] =
                regs.general[*rs1 as usize] & regs.general[*rs2 as usize];
        }
        (Opcode::ModOr, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            regs.general[*rd as usize] =
                regs.general[*rs1 as usize] | regs.general[*rs2 as usize];
        }
        (Opcode::ModImp, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if a == 0 || b != 0 { 1 } else { 0 };
        }
        _ => {}
    }
}
