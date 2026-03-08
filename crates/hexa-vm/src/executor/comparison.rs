use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

const DEFAULT_THRESHOLD: u64 = 100;

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    match (&inst.opcode, &inst.operands) {
        (Opcode::CmpMlt, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if b.saturating_sub(a) > DEFAULT_THRESHOLD { 1 } else { 0 };
        }
        (Opcode::CmpMgt, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            let a = regs.general[*rs1 as usize];
            let b = regs.general[*rs2 as usize];
            regs.general[*rd as usize] = if a.saturating_sub(b) > DEFAULT_THRESHOLD { 1 } else { 0 };
        }
        (Opcode::CmpInc, Operands::RdRs1Rs2 { rd, .. }) => {
            // TODO: type disjoint 체크
            regs.general[*rd as usize] = 0;
        }
        (Opcode::CmpFuz, Operands::RdRs1Rs2 { rd, rs1, rs2 }) => {
            // 확률 점수: 단순 비율 (MVP stub)
            let a = regs.general[*rs1 as usize] as f64;
            let b = regs.general[*rs2 as usize] as f64;
            let score = if b == 0.0 { 0.0 } else { (a / b).min(1.0) };
            regs.general[*rd as usize] = (score * 1000.0) as u64;
        }
        _ => {}
    }
}
