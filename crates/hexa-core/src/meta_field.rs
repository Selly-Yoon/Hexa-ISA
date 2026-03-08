/// Meta-Field (Byte 2) - FIXED structure
/// [7:6] ApproxMode | [5:4] FallbackPolicy | [3:0] ConfidenceTarget

#[derive(Debug, Clone, PartialEq)]
pub enum ApproxMode {
    PreciseOnly   = 0b00,
    ApproxAllowed = 0b01,
    Hybrid        = 0b10,
    Reserved      = 0b11,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FallbackPolicy {
    Abort    = 0b00,
    Reduce   = 0b01,
    Retry    = 0b10,
    Escalate = 0b11,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaField {
    pub approx_mode: ApproxMode,
    pub fallback: FallbackPolicy,
    pub confidence: u8, // 4-bit: 0x0=DontCare 0x8=>50% 0xC=>90% 0xF=>99.9%
}

impl MetaField {
    pub fn from_byte(byte: u8) -> Self {
        let approx_mode = match (byte >> 6) & 0b11 {
            0b00 => ApproxMode::PreciseOnly,
            0b01 => ApproxMode::ApproxAllowed,
            0b10 => ApproxMode::Hybrid,
            _    => ApproxMode::Reserved,
        };
        let fallback = match (byte >> 4) & 0b11 {
            0b00 => FallbackPolicy::Abort,
            0b01 => FallbackPolicy::Reduce,
            0b10 => FallbackPolicy::Retry,
            _    => FallbackPolicy::Escalate,
        };
        let confidence = byte & 0x0F;
        MetaField { approx_mode, fallback, confidence }
    }

    /// confidence target을 0.0~1.0 f64로 변환
    pub fn confidence_threshold(&self) -> f64 {
        match self.confidence {
            0x0 => 0.0,
            0x8 => 0.5,
            0xC => 0.9,
            0xF => 0.999,
            v   => v as f64 / 15.0,
        }
    }
}
