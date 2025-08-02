mod decoder;
mod memory;
mod cpu;
mod loader;

use std::path::Path;
use memory::Mmu;
use cpu::Cpu;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum EmulatorErr {
    #[error("Loader error: {0}")]
    ErrLoader(#[from] loader::LoaderErr),

    #[error("CPU error: {0}")]
    ErrCpu(#[from] cpu::CpuErr),

    #[error("MMU error: {0}")]
    ErrMmu(#[from] memory::MmmuErr),
}

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

    fn load<P: AsRef<Path>>(&mut self, file: &P) -> Result<(), EmulatorErr>{
        let pc_val = loader::load_file_to_dram(&mut self.mmu, &file)? as u64;

        //this cant return error in any case
        self.cpu.set_reg(cpu::RegType::ProgCounter, 0, pc_val)?;

        Ok(())
    }

    fn fetch_rinst(&self) -> Result<u32, EmulatorErr>{
        let mut inst = 0;
        let pc = self.cpu.get_reg(cpu::RegType::ProgCounter, 0)?;
        
        let inst_l = self.mmu.dram_read(999999999999 as usize, 4)?;
    
        //coz little endian
        for (i, val) in inst_l.iter().enumerate() {
            inst = inst | (*val as u32) << 8 * i; 
        }

        Ok(inst)
    }

}

pub fn emulate() {
    let mut emu = Emulator::new();
    //println!("{:?}", decoder::decode(0x7369));
    if let Ok(()) = emu.load(&"/home/Deep/Desktop/temp") {
        if let Ok(inst) = emu.fetch_rinst() {
            println!("{0:x?}", inst);
            println!("{:?}", decoder::decode(inst));
        }
        else if let Err(err) = emu.fetch_rinst() {
            println!("{:?}", err);
        }
    }
    else {
        println!("cooked");
    }
    println!("{:?}",decoder::decode(0x00000073));
}