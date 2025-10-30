mod decoder;
mod memory;
mod cpu;
mod loader;
mod exceptions;

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

        self.cpu.set_reg(cpu::RegType::ProgCounter, 0, pc_val)?;

        Ok(())
    }

    fn fetch_rinst(&self) -> Result<u32, EmulatorErr>{
        //do not throw any exception for fetch coz of perms according to: 1.4. Memory

        let mut inst = 0;
        let pc = self.cpu.get_reg(cpu::RegType::ProgCounter, 0)?;
        
        let inst_l = self.mmu.dram_read(pc as usize, 4)?;
    
        //coz little endian
        for (i, val) in inst_l.iter().enumerate() {
            inst = inst | (*val as u32) << 8 * i; 
        }

        Ok(inst)
    }

    fn exec(&mut self) -> Result<(), EmulatorErr> {
        
        let pc = self.cpu.get_reg(cpu::RegType::ProgCounter, 0)?;
        let perms = self.mmu.perm_get(pc as usize, cpu::RAW_INST_SIZE as usize)?;

        //checking if PC points to executable memory
        if perms.iter().all(|perm| perm & memory::PERM_X == 0) {
            todo!("exception perm");
        }    

        //checking alignment
        if pc % 4 != 0 {
            todo!("exception align ");
        }
  
        
        //FDE
        let rinst = self.fetch_rinst()?;
        let inst_type = decoder::fetch_inst_type(rinst);
        let inst = decoder::decode(rinst);

        cpu::exec(self, inst, inst_type)?;


        Ok(())
    }

}

pub fn emulate() {
    let mut emu = Emulator::new();
    //println!("{:?}", decoder::decode(0x7369));
    if let Ok(()) = emu.load(&"/home/Deep/Desktop/emu/temp") {
        if let Ok(rinst) = emu.fetch_rinst() {
            println!("{0:x?}", rinst);
            println!("{:?}", decoder::decode(rinst));
            let inst = decoder::decode(rinst);
            
            if let Err(err) = emu.exec() {
                println!("{:?}", err);
            }
            
        }
        else if let Err(err) = emu.fetch_rinst() {
            println!("{:?}", err);
        }
    }
    else {
        println!("cookedok");
    }
    println!("{:?}",decoder::decode(0x00000073));
}