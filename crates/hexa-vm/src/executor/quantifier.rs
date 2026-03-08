use hexa_core::{instruction::{Instruction, Operands}, opcode::Opcode, registers::Registers};

pub fn execute(inst: &Instruction, regs: &mut Registers) {
    // TODO: ptr 기반 메모리 접근 구현
    // MVP 단계: 레지스터 기반 stub
    match (&inst.opcode, &inst.operands) {
        (Opcode::All, _)    => { /* TODO */ }
        (Opcode::Exist, _)  => { /* TODO */ }
        (Opcode::Nexist, _) => { /* TODO */ }
        (Opcode::ExistU, _) => { /* TODO */ }
        (Opcode::RelMap, _) => { /* TODO */ }
        _ => {}
    }
}
