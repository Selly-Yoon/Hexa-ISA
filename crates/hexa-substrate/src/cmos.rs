use hexa_core::instruction::Instruction;
use crate::backend::{ExecutionResult, SubstrateBackend, TileType};

pub struct CMOSBackend;

impl SubstrateBackend for CMOSBackend {
    fn execute(&self, inst: &Instruction) -> ExecutionResult {
        // MVP: 결과는 hexa-vm executor에서 처리
        // 여기선 substrate 정보만 태깅
        ExecutionResult { value: 0, substrate: TileType::CMOS }
    }

    fn supports_approx(&self) -> bool { true }
    fn tile_type(&self) -> TileType   { TileType::CMOS }
}

// Phase 5+ stub
pub struct PhotonicBackend;
pub struct MemristorBackend;
pub struct SpintronicBackend;

impl SubstrateBackend for PhotonicBackend {
    fn execute(&self, _inst: &Instruction) -> ExecutionResult { unimplemented!() }
    fn supports_approx(&self) -> bool { true }
    fn tile_type(&self) -> TileType   { TileType::Photonic }
}

impl SubstrateBackend for MemristorBackend {
    fn execute(&self, _inst: &Instruction) -> ExecutionResult { unimplemented!() }
    fn supports_approx(&self) -> bool { true }
    fn tile_type(&self) -> TileType   { TileType::Memristor }
}

impl SubstrateBackend for SpintronicBackend {
    fn execute(&self, _inst: &Instruction) -> ExecutionResult { unimplemented!() }
    fn supports_approx(&self) -> bool { true }
    fn tile_type(&self) -> TileType   { TileType::Spintronic }
}
