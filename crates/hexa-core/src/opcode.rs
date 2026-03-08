/// Hexa-ISA Opcode definitions (8-bit)
/// 0x00-0x7F: Legacy Binary (x86-64/ARM mapped)
/// 0x80-0xDF: Core Semantic (FIXED v1.0)
/// 0xE0-0xFF: Extensions (FREE)
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // 0x00-0x7F: Legacy
    Legacy(u8),

    // 0x80-0x8F: Modal
    Nec    = 0x80,
    Pos    = 0x81,
    Taut   = 0x82,
    Cont   = 0x83,
    ModAnd = 0x84,
    ModOr  = 0x85,
    ModImp = 0x86,

    // 0x90-0x9F: Quantifier
    All    = 0x90,
    Exist  = 0x91,
    Nexist = 0x92,
    ExistU = 0x93,
    RelMap = 0x94,

    // 0xA0-0xAF: Comparison
    CmpMlt = 0xA0,
    CmpMgt = 0xA1,
    CmpInc = 0xA2,
    CmpFuz = 0xA3,

    // 0xB0-0xBF: Set-Theoretic
    SetSub = 0xB0,
    SetSup = 0xB1,
    SetInt = 0xB2,
    SetUni = 0xB3,
    SetDif = 0xB4,
    SetSym = 0xB5,
    SetMem = 0xB6,

    // 0xC0-0xCF: Equivalence
    EqId   = 0xC0,
    EqIso  = 0xC1,
    EqSim  = 0xC2,
    NeqSim = 0xC3,

    // 0xD0-0xDF: Quantum Control
    QPrepare  = 0xD0,
    QSuperpose = 0xD1,
    QEntangle = 0xD2,
    QMeasure  = 0xD3,

    // 0xE0-0xFF: Extensions
    ExtPfx = 0xE0,
    EscVnd = 0xF0,
    EscLng = 0xFF,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00..=0x7F => Ok(Opcode::Legacy(byte)),
            0x80 => Ok(Opcode::Nec),
            0x81 => Ok(Opcode::Pos),
            0x82 => Ok(Opcode::Taut),
            0x83 => Ok(Opcode::Cont),
            0x84 => Ok(Opcode::ModAnd),
            0x85 => Ok(Opcode::ModOr),
            0x86 => Ok(Opcode::ModImp),
            0x90 => Ok(Opcode::All),
            0x91 => Ok(Opcode::Exist),
            0x92 => Ok(Opcode::Nexist),
            0x93 => Ok(Opcode::ExistU),
            0x94 => Ok(Opcode::RelMap),
            0xA0 => Ok(Opcode::CmpMlt),
            0xA1 => Ok(Opcode::CmpMgt),
            0xA2 => Ok(Opcode::CmpInc),
            0xA3 => Ok(Opcode::CmpFuz),
            0xB0 => Ok(Opcode::SetSub),
            0xB1 => Ok(Opcode::SetSup),
            0xB2 => Ok(Opcode::SetInt),
            0xB3 => Ok(Opcode::SetUni),
            0xB4 => Ok(Opcode::SetDif),
            0xB5 => Ok(Opcode::SetSym),
            0xB6 => Ok(Opcode::SetMem),
            0xC0 => Ok(Opcode::EqId),
            0xC1 => Ok(Opcode::EqIso),
            0xC2 => Ok(Opcode::EqSim),
            0xC3 => Ok(Opcode::NeqSim),
            0xD0 => Ok(Opcode::QPrepare),
            0xD1 => Ok(Opcode::QSuperpose),
            0xD2 => Ok(Opcode::QEntangle),
            0xD3 => Ok(Opcode::QMeasure),
            0xE0 => Ok(Opcode::ExtPfx),
            0xF0 => Ok(Opcode::EscVnd),
            0xFF => Ok(Opcode::EscLng),
            _ => Err(()),
        }
    }
}
