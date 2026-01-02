use super::{decoder::Inst, Emulator};

pub const MAX_REGS: usize = 32;
pub const RAW_INST_SIZE:u64 = 4;

#[derive(thiserror::Error, Debug)]
pub enum CpuErr {
    #[error("Invalid register: {0}")]
    InvalidRegister(usize),

    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u32),
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
                
                //since X0 is hardwired to zero
                if reg_index == 0 {
                    return Ok(());
                }
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

}

macro_rules! inc_pc {
    ($cpu: expr) => {
      $cpu.pc += RAW_INST_SIZE;  
    
    };
}

macro_rules! dec_pc {
    ($cpu: expr) => {
      $cpu.pc -= RAW_INST_SIZE;  
    
    };
}

fn handle_undefined() {
    todo!("handle undefined");
}


/*
    1.4. Memory
        The memory address space is circular, so that the byte at address dram.len()-1 is adjacent to the byte at address zero
        [it this for software running in execution env, so it should be implemented here]
*/


pub fn exec(emu: &mut Emulator, inst: Inst) -> Result<(), CpuErr> {
    let inc_pc = true;

    match inst {
                
        /*
            2.4.1. Integer Register-Immediate Instructions
        */
    
        Inst::Addi { rd, rs1, imm } => {
            /*
                ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the
                result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV rd, rs1
                assembler pseudoinstruction.
            */

            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)?;
            
            let value = rs1_val.wrapping_add_signed(imm as i64);
            
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value)?;
        }

        Inst::Addiw { rd, rs1, imm } => {
              
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as u32;
            let value = rs1_val.wrapping_add_signed(imm);
                 
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }
                 
        Inst::Slti { rd, rs1, imm } => {
            /*
                SLTI (set less than immediate) places the value 1 in register rd if register rs1 is less than the sign-
                extended immediate when both are treated as signed numbers, else 0 is written to rd.
            */
                      
                 
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64;
            let imm = imm as i64;

            let value = if rs1_val < imm { 1 } else { 0 };
            
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value)?;     
        }

        Inst::Sltiu { rd, rs1, imm } => {
            /* 
                SLTIU is similar to SLTI
                but compares the values as unsigned numbers (i.e., the immediate is first sign-extended to XLEN bits
                then treated as an unsigned number). Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals zero, otherwise
                sets rd to 0 (assembler pseudoinstruction SEQZ rd, rs). 
            */
                                 
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)?;
            let imm = imm as i64 as u64;
                                        
            let value = if rs1_val < imm { 1 } else { 0 };
                                                 

            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value)?;
        }
                      
            /* 
                ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1 and the
                sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1 performs a bitwise
                logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
            */
                  
        Inst::Andi { rd, rs1, imm } => {
                             
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64;
                      
            let value = rs1_val & imm as i64;
                             
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }

        Inst::Ori { rd, rs1, imm } => {

            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64;

            let value = rs1_val | imm as i64;

            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }

        Inst::Xori { rd, rs1, imm } => {

            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64;
                                               
            let value = rs1_val ^ imm as i64;

            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }

            /*
                SLLI is a logical left shift (zeros are shifted into the lower bits); SRLI is a logical
                right shift (zeros are shifted into the upper bits); and SRAI is an arithmetic right shift (the original sign
                bit is copied into the vacated upper bits).
            */

        Inst::Slli { rd, rs1, shamt } => {
            
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)?;
                            
            let value  = rs1_val << shamt; 
                                    
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value)?;
        }

        Inst::Slliw { rd, rs1, shamt } => {
               
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as u32;
                  
            let value  = rs1_val << shamt; 
                 
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }
         
        Inst::Srli { rd, rs1, shamt } => {
             
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)?;
             
            let value  = rs1_val >> shamt; 
             
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value)?;
        }

        Inst::Srliw { rd, rs1, shamt } => {
        
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as u32;
            
            let value  = rs1_val >> shamt;

            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }

        Inst::Srai { rd, rs1, shamt } => {
            
            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64;
              
            let value  = rs1_val >> shamt; 
            
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }

        Inst::Sraiw { rd, rs1, shamt } => {

            let rs1_val = emu.cpu.get_reg(RegType::GenPurpose, rs1 as usize)? as i64 as i32;

            let value  = rs1_val >> shamt; 

            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as i64 as u64)?;
        }

        Inst::Lui { rd, imm } => {
            /* 
                LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI places
                the 32-bit U-immediate value into the destination register rd, filling in the lowest 12 bits with zeros.
            */
        
            let value = (imm as i64) << 12; 
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }
        
        Inst::Auipc { rd, imm } => {
            /* 
                AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type format.
                AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros, adds this
                offset to the address of the AUIPC instruction, then places the result in register rd.
            */
            
            let value = ((imm as i64) << 12) + emu.cpu.pc as i64;
            emu.cpu.set_reg(RegType::GenPurpose, rd as usize, value as u64)?;
        }


        _=> handle_undefined()
    }

    if inc_pc {
        inc_pc!(emu.cpu);
    }

    Ok(())
}