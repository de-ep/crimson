mod decoder;
mod memory;
mod cpu;
mod loader;
mod exceptions;

use std::path::Path;
use memory::Mmu;
use cpu::Cpu;
use exceptions::Exceptions;

#[derive(thiserror::Error, Debug)]
pub enum EmulatorErr {
    #[error("Loader error: {0}")]
    ErrLoader(#[from] loader::LoaderErr),

    #[error("CPU error: {0}")]
    ErrCpu(#[from] cpu::CpuErr),

    #[error("MMU error: {0}")]
    ErrMmu(#[from] memory::MmmuErr),

    #[error("Exception Handler error: {0}")]
    ErrExceptionHandler(#[from] exceptions::ExceptionHandlerErr),
}

#[derive(Clone)]
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

    pub fn take_snapshot(&self) -> Emulator{
        self.clone()

    }

    fn load<P: AsRef<Path>>(&mut self, file: &P) -> Result<loader::File, EmulatorErr>{
        let file = loader::load_file_to_dram(&mut self.mmu, &file)?;

        let pc_val = file.entry_point;

        self.cpu.set_pc(pc_val);

        Ok(file)
    }

    fn fetch_rinst(&self) -> Result<u32, EmulatorErr>{
        //do not throw any exception for fetch coz of perms according to: 1.4. Memory

        let mut inst = 0;
        let pc = self.cpu.get_pc();
        
        let inst_l = self.mmu.dram_read(pc as usize, cpu::RAW_INST_SIZE as usize)?;
    
        //coz little endian
        for (i, val) in inst_l.iter().enumerate() {
            inst = inst | (*val as u32) << 8 * i; 
        }

        Ok(inst)
    }

    fn exec(&mut self) -> Result<(), EmulatorErr> {
        
        let pc = self.cpu.get_pc();
        let perms = self.mmu.perm_get(pc as usize, cpu::RAW_INST_SIZE as usize)?;

        //checking if PC points to executable memory
        if perms.iter().all(|perm| perm & memory::PERM_X == 0) {
            todo!("exception perm");
        }    

        //checking alignment
        if pc % 4 != 0 {
            self.handle_exception(Exceptions::ExceptionInstructionAddressMisaligned(pc as usize))?;
        }
  
        //FDE
        let rinst = self.fetch_rinst()?;
        let inst = decoder::decode(rinst);
        cpu::exec(self, inst)?;

        
        Ok(())
    }

    fn handle_exception(&self, exception: Exceptions) -> Result<(), EmulatorErr> {
        let continue_execution = exceptions::handle_expection(exception)?;

        if !continue_execution {
            println!();
        }

        Ok(())
    }

}

pub fn emulate() {
    let mut emu = Emulator::new();
    //println!("{:?}", decoder::decode(0x7369));
    if let Ok(file) = emu.load(&"/home/Deep/Desktop/emu/temp") {
        if let Ok(rinst) = emu.fetch_rinst() {
            println!("{0:x?}", rinst);
            println!("{:?}", decoder::decode(rinst));
            let inst = decoder::decode(rinst);
            
            println!("hii");
            if let Err(err) = emu.exec() {
                println!("nohi{:?}", err);
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