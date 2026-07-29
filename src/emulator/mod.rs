mod decoder;
mod memory;
mod cpu;
mod loader;
mod exceptions;

use std::path::Path;
use memory::Mmu;
use cpu::{Cpu, CpuErr};
use exceptions::{handle_expection, Exceptions};

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

    pub fn take_snapshot(&self) -> Self{
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

    fn exec(&mut self) -> Result<bool, EmulatorErr> {
        
        let pc = self.cpu.get_pc() as usize;
        let perms = self.mmu.perm_get(pc, cpu::RAW_INST_SIZE as usize)?;

        //checking if PC points to executable memory
        if perms.iter().any(|perm| perm & memory::PERM_X == 0) {
            handle_expection(Exceptions::ExceptionAccessFault(pc))?;
        }    

        //checking alignment
        if pc % 4 != 0 {
            self.handle_exception(Exceptions::ExceptionInstructionAddressMisaligned(pc as usize))?;
        }
  
        //FDE
        let rinst = self.fetch_rinst()?;
        let inst = decoder::decode(rinst);println!("{:?}",inst);

        if let Err(err) = cpu::exec(self, inst) {
            return match err {// inc pc incase we decide that we can continue running after exception
                CpuErr::ReadAccessFault(offset, _) => self.handle_exception(Exceptions::ExceptionAccessFault(offset)),
                CpuErr::WriteAccessFault(offset,_) => self.handle_exception(Exceptions::ExceptionAccessFault(offset)),

                _=> Err(err.into())
            }
        } 
        
        Ok(false)
    }

    fn handle_exception(&self, exception: Exceptions) -> Result<bool, EmulatorErr> {
        let continue_execution = exceptions::handle_expection(exception)?;

        if !continue_execution {
            println!();
        }

        Ok(continue_execution)
    }

}

/*
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

     */



pub fn emulate<P: AsRef<Path>>(file: &P) -> Result<(), EmulatorErr>{
    let mut emu = Emulator::new();
    let mut exit_cond = false;

    emu.load(file)?;

    while !exit_cond {
        
        exit_cond = emu.exec()?;    
    }
    
    for i in 0..cpu::MAX_REGS{
        println!("x{}: {}", i, emu.cpu.get_reg(i)?);
    }println!("pc: {}", emu.cpu.get_pc());


    Ok(())
}