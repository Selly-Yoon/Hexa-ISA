use hexa_core::{
    instruction::Instruction,
    meta_field::{ApproxMode, FallbackPolicy},
    opcode::Opcode,
    registers::Registers,
};
use crate::executor;
use crate::fallback::handle_fallback;

pub fn dispatch(inst: &Instruction, regs: &mut Registers) {
    // TODO: confidence 사전 평가 후 fallback 분기
    // 현재는 직접 실행 (MVP 초기 단계)
    use Opcode::*;
    match &inst.opcode {
        // Modal
        Nec | Pos | Taut | Cont | ModAnd | ModOr | ModImp
            => executor::modal::execute(inst, regs),
        // Quantifier
        All | Exist | Nexist | ExistU | RelMap
            => executor::quantifier::execute(inst, regs),
        // Comparison
        CmpMlt | CmpMgt | CmpInc | CmpFuz
            => executor::comparison::execute(inst, regs),
        // Set
        SetSub | SetSup | SetInt | SetUni | SetDif | SetSym | SetMem
            => executor::set::execute(inst, regs),
        // Equivalence
        EqId | EqIso | EqSim | NeqSim
            => executor::equiv::execute(inst, regs),
        // Quantum
        QPrepare | QSuperpose | QEntangle | QMeasure
            => executor::quantum::execute(inst, regs),
        // Legacy pass-through
        Legacy(_) => {},
        // Extensions: TODO
        _ => {},
    }
}
