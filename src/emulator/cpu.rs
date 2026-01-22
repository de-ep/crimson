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

#[derive(Clone)]
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

    pub fn get_reg(&self, reg_index: usize) -> Result<u64, CpuErr> {
        if reg_index >= MAX_REGS {
            return Err(CpuErr::InvalidRegister(reg_index));
        }
        
        Ok(self.r[reg_index])
    }

    pub fn set_reg(&mut self, reg_index: usize, value: u64) -> Result<(), CpuErr>{
        //since X0 is hardwired to zero
        if reg_index == 0 {
            return Ok(());
        }
        if reg_index >= MAX_REGS {
            return Err(CpuErr::InvalidRegister(reg_index));
        }
        
        self.r[reg_index] = value;
            
  
        Ok(())
    }

    pub fn get_pc(&self) -> u64 {
        self.pc
    }

    pub fn set_pc(&mut self, val: u64) {
        self.pc = val;
    }

}

macro_rules! inc_pc {
    ($cpu: expr) => {
        $cpu.set_pc($cpu.get_pc() + RAW_INST_SIZE);
    
    };
}

macro_rules! dec_pc {
    ($cpu: expr) => {
        $cpu.set_pc($cpu.get_pc() - RAW_INST_SIZE);
    
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

macro_rules! get_circular_mem_vaddr {
    ($vaddr: expr, $dram_len: expr) => {
        if $dram_len <= $vaddr {
            $vaddr - $dram_len
        } else {
            $vaddr
        }
    };
}


/*
    2.1. Programmers' Model for Base Integer ISA
        The standard software calling convention uses register x1 to hold the return address for a call, 
        with register x5 available as an alternate link register.
        The standard calling convention uses register x2 as the stack pointer.
*/

pub fn exec(emu: &mut Emulator, inst: Inst) -> Result<(), CpuErr> {
    let inc_pc = true;

    match inst {
                
        /*
            2.4.1. Integer Register-Immediate Instructions & 
            4.2.1. Integer Register-Immediate Instructions
        */
    
        Inst::Addi { rd, rs1, imm } => {
            /*
                ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the
                result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV rd, rs1
                assembler pseudoinstruction.
            */

            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            
            let value = rs1_val.wrapping_add_signed(imm as i64);
            
            emu.cpu.set_reg(rd as usize, value)?;
        }

        Inst::Addiw { rd, rs1, imm } => {
              
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i32;
            let value = rs1_val.wrapping_add(imm) as i64;
                 
            emu.cpu.set_reg(rd as usize, value as u64)?;
        }
                 
        Inst::Slti { rd, rs1, imm } => {
            /*
                SLTI (set less than immediate) places the value 1 in register rd if register rs1 is less than the sign-
                extended immediate when both are treated as signed numbers, else 0 is written to rd.
            */

            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
            let imm = imm as i64;

            let value = rs1_val < imm ;
            
            emu.cpu.set_reg(rd as usize, value as u64)?;     
        }

        Inst::Sltiu { rd, rs1, imm } => {
            /* 
                SLTIU is similar to SLTI
                but compares the values as unsigned numbers (i.e., the immediate is first sign-extended to XLEN bits
                then treated as an unsigned number). Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals zero, otherwise
                sets rd to 0 (assembler pseudoinstruction SEQZ rd, rs). 
            */
                                 
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let imm = imm as i64 as u64;
                                        
            let value = if rs1_val < imm { 1 } else { 0 };
                                                 

            emu.cpu.set_reg(rd as usize, value)?;
        }
                      
            /* 
                ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1 and the
                sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1 performs a bitwise
                logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
            */
                  
        Inst::Andi { rd, rs1, imm } => {
                             
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
                      
            let value = rs1_val & imm as i64;
                             
            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        Inst::Ori { rd, rs1, imm } => {

            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;

            let value = rs1_val | imm as i64;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        Inst::Xori { rd, rs1, imm } => {

            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
                                               
            let value = rs1_val ^ imm as i64;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

            /*
                SLLI is a logical left shift (zeros are shifted into the lower bits); SRLI is a logical
                right shift (zeros are shifted into the upper bits); and SRAI is an arithmetic right shift (the original sign
                bit is copied into the vacated upper bits).
            */

        Inst::Slli { rd, rs1, shamt } => {
            
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
                            
            let value  = rs1_val << shamt; 
                                    
            emu.cpu.set_reg(rd as usize, value)?;
        }

        Inst::Slliw { rd, rs1, shamt } => {
               
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as u32;
                  
            let value  = rs1_val << shamt; 
                 
            emu.cpu.set_reg(rd as usize, value as i32 as i64 as u64)?;
        }
         
        Inst::Srli { rd, rs1, shamt } => {
             
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
             
            let value  = rs1_val >> shamt; 
             
            emu.cpu.set_reg(rd as usize, value)?;
        }

        Inst::Srliw { rd, rs1, shamt } => {
        
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as u32;
            
            let value  = rs1_val >> shamt;

            emu.cpu.set_reg(rd as usize, value as i32 as i64 as u64)?;
        }

        Inst::Srai { rd, rs1, shamt } => {
            
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
              
            let value  = rs1_val >> shamt; 
            
            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        Inst::Sraiw { rd, rs1, shamt } => {

            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i32;

            let value  = rs1_val >> shamt; 

            emu.cpu.set_reg(rd as usize, value as i64 as u64)?;
        }

        Inst::Lui { rd, imm } => {
            /* 
                LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI places
                the 32-bit U-immediate value into the destination register rd, filling in the lowest 12 bits with zeros.
            */
        
            let value = (imm as i64) << 12; 
            emu.cpu.set_reg(rd as usize, value as u64)?;
        }
        
        Inst::Auipc { rd, imm } => {
            /* 
                AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type format.
                AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros, adds this
                offset to the address of the AUIPC instruction, then places the result in register rd.
            */
            
            let value = ((imm as i64) << 12) + emu.cpu.pc as i64;
            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        /*
            2.4.2. Integer Register-Register Operations &
            4.2.2. Integer Register-Register Operations
        */

            /*
                ADD performs the addition of rs1 and rs2. 
                SUB performs the subtraction of rs2 from rs1. 
                
                Overflows are ignored and the low XLEN bits of results are written to the destination rd. 
            */

        Inst::Add { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;

            let value = rs1_val.wrapping_add(rs2_val);

            emu.cpu.set_reg(rd as usize, value)?;
        }

        Inst::Sub { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;

            let value = rs1_val.wrapping_sub(rs2_val);

            emu.cpu.set_reg(rd as usize, value)?;
        }

            /*
                ADDW and SUBW are RV64I-only instructions that are defined analogously to ADD and SUB but
                operate on 32-bit values and produce signed 32-bit results. Overflows are ignored, and the low 32-bits
                of the result is sign-extended to 64-bits and written to the destination register.
            */


        Inst::Addw { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i32;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i32;

            let value = rs1_val.wrapping_add(rs2_val) as i64;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        Inst::Subw { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i32;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i32;

            let value = rs1_val.wrapping_sub(rs2_val) as i64;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }
    
            /*
                SLT and SLTU perform signed and unsigned compares respectively, writing 1 to rd if rs1 < rs2, 0 otherwise.
            */

        Inst::Slt { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i64;

            let value = rs1_val < rs2_val;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

        Inst::Sltu { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;

            let value = rs1_val < rs2_val;

            emu.cpu.set_reg(rd as usize, value as u64)?;
        }

            /*
                SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts on the value in
                register rs1 by the shift amount held in register rs2.
                In RV64I, only the low 6 bits of rs2 are considered for the shift amount.
            */

        Inst::Sll { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;

            let value = rs1_val << (rs2_val & 0b1111_11);

            emu.cpu.set_reg(rd as usize, value)?;

        }

        Inst::Srl { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;

            let value = rs1_val >> (rs2_val & 0b1111_11);

            emu.cpu.set_reg(rd as usize, value)?;

        }

        Inst::Sra { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i64;

            let value = rs1_val >> (rs2_val & 0b1111_11);

            emu.cpu.set_reg(rd as usize, value as u64)?;

        }

        /*
            SLLW, SRLW, and SRAW are RV64I-only instructions that are analogously defined but operate on
            32-bit values and sign-extend their 32-bit results to 64 bits.
            The shift amount is given by rs2[4:0]. 
        */

        Inst::Sllw { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as u32;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as u32;

            let value = rs1_val << (rs2_val & 0b1111_1);

            emu.cpu.set_reg(rd as usize, value as i32 as i64 as u64)?;

        }

        Inst::Srlw { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as u32;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as u32;

            let value = rs1_val >> (rs2_val & 0b1111_1);

            emu.cpu.set_reg(rd as usize, value as i32 as i64 as u64)?;

        }

        Inst::Sraw { rd, rs1, rs2 } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i32;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i32;

            let value = rs1_val >> (rs2_val & 0b1111_1);

            emu.cpu.set_reg(rd as usize, value as i64 as u64)?;

        }



        _=> handle_undefined()
    }

    if inc_pc {
        inc_pc!(emu.cpu);
    }

    Ok(())
}