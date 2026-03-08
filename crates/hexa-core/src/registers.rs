/// Hexa-ISA Register File
/// chip8의 레지스터 배열 구조를 기반으로 확장

pub struct Registers {
    /// 범용 레지스터 32개 (u64)
    pub general: [u64; 32],
    /// 확률 벡터 레지스터 (Quantum ops용)
    pub prob: [f64; 32],
    /// Program Counter
    pub pc: u64,
    /// Flags
    pub zero: bool,
    pub overflow: bool,
    pub confidence_fail: bool,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            general:          [0u64; 32],
            prob:             [0.0f64; 32],
            pc:               0,
            zero:             false,
            overflow:         false,
            confidence_fail:  false,
        }
    }
}
