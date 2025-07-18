mod decoder;
mod memory;
mod cpu;

use memory::Mmu;
use cpu::Cpu;

struct Emulator {
    cpu: Cpu,
    mmu: Mmu,
}

impl Emulator {
    fn new() -> Self {
        Emulator {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
        }
    }

    fn load(&self) {

    }

}

pub fn emulate() {
    
}