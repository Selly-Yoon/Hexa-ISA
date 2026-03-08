use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    let threshold = inst.meta.confidence_threshold();

    match (&inst.opcode, &inst.operands) {
        (Opcode::EqId, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let eq = regs.general[*rs1 as usize] == regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if eq { 1 } else { 0 };
        }
        (Opcode::EqIso, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            // TODO: structural compare
            let eq = regs.general[*rs1 as usize] == regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if eq { 1 } else { 0 };
        }
        (Opcode::EqSim, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize] as f64;
            let b = regs.general[*rs2 as usize] as f64;
            let dist = (a - b).abs() / (a.max(b) + 1.0);
            let epsilon = 1.0 - threshold;
            regs.general[*rd as usize] = if dist < epsilon { 1 } else { 0 };
            regs.confidence_fail = dist >= epsilon;
        }
        (Opcode::NeqSim, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize] as f64;
            let b = regs.general[*rs2 as usize] as f64;
            let dist = (a - b).abs() / (a.max(b) + 1.0);
            let epsilon = 1.0 - threshold;
            regs.general[*rd as usize] = if dist >= epsilon { 1 } else { 0 };
        }
        _ => {}
    }
}
