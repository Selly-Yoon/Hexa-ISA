/// Quantum ops: MVP stub (확률 벡터 시뮬레이션)
use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    match (&inst.opcode, &inst.operands) {
        (Opcode::QPrepare, Operands::RdSt { rd, st }) => {
            regs.prob[*rd as usize] = if *st == 0 { 0.0 } else { 1.0 };
        }
        (Opcode::QSuperpose, Operands::RdRs { rd, .. }) => {
            regs.prob[*rd as usize] = 0.5; // Hadamard
        }
        (Opcode::QEntangle, Operands::R1R2 { r1, r2 }) => {
            // CNOT stub: r2 확률을 r1에 바인딩
            let p = regs.prob[*r1 as usize];
            regs.prob[*r2 as usize] = p;
        }
        (Opcode::QMeasure, Operands::RdSt { rd, rs1: _ }) => {
            // 베르누이 샘플링 stub
            let p = regs.prob[*rd as usize];
            regs.general[*rd as usize] = if p >= 0.5 { 1 } else { 0 };
        }
        _ => {}
    }
}
