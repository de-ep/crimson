const MAX_REGS: usize = 32;

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

    pub fn get_reg(&self, rtype: RegType, reg_index: usize) -> Option<u64> {
        match rtype {
            RegType::GenPurpose => {
                if reg_index >= MAX_REGS {
                    return None;
                }

                Some(self.r[reg_index])
            }
            RegType::ProgCounter => {
                Some(self.pc)
            }
        }
        
    }

    pub fn set_reg(&mut self, rtype: RegType, reg_index: usize, value: u64) -> Result<(), ()>{
        match rtype {
            RegType::GenPurpose => {
                if reg_index >= MAX_REGS {
                    return Err(());
                }

                self.r[reg_index] = value;
            }
            RegType::ProgCounter => {
                self.pc = value;
            }
        }
        
        Ok(())
    }

}