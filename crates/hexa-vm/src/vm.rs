use hexa_core::registers::Registers;
use hexa_decoder::decode;
use crate::dispatcher::dispatch;

pub struct VM {
    pub memory: Vec<u8>,
    pub regs:   Registers,
}

impl VM {
    pub fn new(program: Vec<u8>) -> Self {
        VM { memory: program, regs: Registers::default() }
    }

    /// fetch → decode → dispatch 메인 루프
    pub fn run(&mut self) {
        loop {
            let pc = self.regs.pc as usize;
            if pc >= self.memory.len() { break; }

            let (inst, consumed) = match decode(&self.memory[pc..]) {
                Ok(v)  => v,
                Err(e) => { eprintln!("DecodeError: {:?}", e); break; }
            };

            self.regs.pc += consumed as u64;
            dispatch(&inst, &mut self.regs);
        }
    }
}
