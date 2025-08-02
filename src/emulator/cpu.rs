use super::{decoder::{Inst, InstType}, Emulator};

const MAX_REGS: usize = 32;


#[derive(thiserror::Error, Debug)]
pub enum CpuErr {
    #[error("Invalid register: {0}")]
    InvalidRegister(usize),
}

pub enum RegType {
    GenPurpose,
    ProgCounter,
}

pub struct Cpu {
    r: [u64; MAX_REGS],
    pc: u64,
}

impl Cpu {
    pub fn new() -> Self{
        Cpu {
            r: [0; MAX_REGS],
            pc: 0,
        }
    }

    pub fn get_reg(&self, rtype: RegType, reg_index: usize) -> Result<u64, CpuErr> {
        match rtype {
            RegType::GenPurpose => {
                if reg_index >= MAX_REGS {
                    return Err(CpuErr::InvalidRegister(reg_index));
                }

                Ok(self.r[reg_index])
            }
            RegType::ProgCounter => {
                Ok(self.pc)
            }
        }
        
    }

    pub fn set_reg(&mut self, rtype: RegType, reg_index: usize, value: u64) -> Result<(), CpuErr>{
        match rtype {
            RegType::GenPurpose => {
                if reg_index >= MAX_REGS {
                    return Err(CpuErr::InvalidRegister(reg_index));
                }

                self.r[reg_index] = value;
            }
            RegType::ProgCounter => {
                self.pc = value;
            }
        }
        
        Ok(())
    }

    pub fn exec(emu: &mut Emulator, inst: Inst, inst_type: InstType) -> Result<(), CpuErr> {

        match inst_type {
            InstType::R => {
                
            }
            InstType::I => {
                
            }
            InstType::S => {
                
            }
            InstType::B => {
                
            }
            InstType::U => {
                
            }
            InstType::J => {
                
            }
        }


        Ok(())
    }
}