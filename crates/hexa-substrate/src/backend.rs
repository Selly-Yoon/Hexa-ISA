use hexa_core::instruction::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    CMOS,
    Photonic,    // Phase 5+
    Memristor,   // Phase 5+
    Spintronic,  // Phase 5+
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub value:    u64,
    pub substrate: TileType,
}

pub trait SubstrateBackend {
    fn execute(&self, inst: &Instruction) -> ExecutionResult;
    fn supports_approx(&self) -> bool;
    fn tile_type(&self) -> TileType;
}
