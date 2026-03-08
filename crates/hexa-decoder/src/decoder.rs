use hexa_core::{
    instruction::{Instruction, Operands},
    meta_field::MetaField,
    opcode::Opcode,
};

#[derive(Debug)]
pub enum DecodeError {
    UnexpectedEof,
    UnknownOpcode(u8),
    OperandMismatch,
}

/// 바이트 슬라이스 → Instruction
/// 인코딩: [u8: opcode][u8: meta-field][...operands]
pub fn decode(bytes: &[u8]) -> Result<(Instruction, usize), DecodeError> {
    if bytes.len() < 2 {
        return Err(DecodeError::UnexpectedEof);
    }

    let opcode_byte = bytes[0];
    let meta_byte   = bytes[1];
    let meta        = MetaField::from_byte(meta_byte);

    let opcode = Opcode::try_from(opcode_byte)
        .map_err(|_| DecodeError::UnknownOpcode(opcode_byte))?;

    // operand 파싱 (opcode family별)
    let (operands, consumed) = parse_operands(&opcode, &bytes[2..])?;

    Ok((
        Instruction { opcode, meta, operands },
        2 + consumed,
    ))
}

fn parse_operands(
    opcode: &Opcode,
    rest: &[u8],
) -> Result<(Operands, usize), DecodeError> {
    use Opcode::*;
    match opcode {
        // rd, rs (2바이트)
        Nec | Pos => {
            if rest.len() < 2 { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::RdRs { rd: rest[0], rs: rest[1] }, 2))
        }
        // rd only (1바이트)
        Taut | Cont | QSuperpose => {
            if rest.is_empty() { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::RdRs { rd: rest[0], rs: 0 }, 1))
        }
        // rd, rs1, rs2 (3바이트)
        ModAnd | ModOr | ModImp
        | CmpMlt | CmpMgt | CmpInc | CmpFuz
        | SetSub | SetSup | SetInt | SetUni | SetDif | SetSym | SetMem
        | EqId | EqIso | EqSim | NeqSim => {
            if rest.len() < 3 { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::RdRs1Rs2 {
                rd: rest[0], rs1: rest[1], rs2: rest[2],
            }, 3))
        }
        // r1, r2 (2바이트)
        QEntangle | RelMap => {
            if rest.len() < 2 { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::R1R2 { r1: rest[0], r2: rest[1] }, 2))
        }
        // rd, st (2바이트)
        QPrepare | QMeasure => {
            if rest.len() < 2 { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::RdSt { rd: rest[0], st: rest[1] }, 2))
        }
        // rd, ptr(8바이트), len(4바이트)
        All | Exist | Nexist | ExistU => {
            if rest.len() < 13 { return Err(DecodeError::UnexpectedEof); }
            let rd  = rest[0];
            let ptr = u64::from_le_bytes(rest[1..9].try_into().unwrap());
            let len = u32::from_le_bytes(rest[9..13].try_into().unwrap());
            Ok((Operands::RdPtrLen { rd, ptr, len }, 13))
        }
        // imm(1바이트)
        ExtPfx | EscVnd | EscLng => {
            if rest.is_empty() { return Err(DecodeError::UnexpectedEof); }
            Ok((Operands::Imm(rest[0] as u64), 1))
        }
        // Legacy: pass-through
        Legacy(_) => Ok((Operands::None, 0)),
    }
}
