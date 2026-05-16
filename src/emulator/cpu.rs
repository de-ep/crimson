use std::u64;

use super::{decoder::Inst, exceptions, memory::{self, PERM_R, PERM_W}, Emulator};

pub const MAX_REGS: usize = 32;
pub const RAW_INST_SIZE:u64 = 4;

#[derive(thiserror::Error, Debug)]
pub enum CpuErr {
    #[error("Invalid register: {0}")]
    InvalidRegister(usize),

    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u32),

    #[error("Read Access Fault: offset: {0}, len: {1}")]
    ReadAccessFault(usize, usize),

    #[error("Write Access Fault: offset: {0}, len: {1}")]
    WriteAccessFault(usize, usize),

    #[error("MMU error: {0}")]
    ErrMmu(#[from] memory::MmmuErr),
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

fn inc_pc(cpu: &mut Cpu) {
    cpu.set_pc(cpu.get_pc() + RAW_INST_SIZE);
}

fn dec_pc(cpu: &mut Cpu) {
    cpu.set_pc(cpu.get_pc() - RAW_INST_SIZE);
}

fn handle_undefined() {
    todo!("handle undefined");
}


/*
    1.4. Memory
        The memory address space is circular, so that the byte at address dram.len()-1 is adjacent to the byte at address zero
        [it this for software running in execution env, so it should be implemented here]
*/

fn get_circular_mem_vaddr(vaddr: usize, dram_len: usize) -> usize {
    if dram_len <= vaddr {
        return vaddr - dram_len;
    }

    vaddr
}

/*
    2.1. Programmers' Model for Base Integer ISA
        The standard software calling convention uses register x1 to hold the return address for a call, 
        with register x5 available as an alternate link register.
        The standard calling convention uses register x2 as the stack pointer.
*/

pub fn exec(emu: &mut Emulator, inst: Inst) -> Result<(), CpuErr> {
    let mut increment_program_counter = true;

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
            
            let value = emu.cpu.pc.wrapping_add_signed((imm as i64) << 12);
            emu.cpu.set_reg(rd as usize, value)?;
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


        /*
            2.5. Control Transfer Instructions
        */

            /* 
                JAL stores the
                address of the instruction following the jump ('pc'+4) into register rd.
            */

        Inst::Jal { rd, imm } => {
            let pc = emu.cpu.get_pc();
            
            let return_address = pc + 4;
            emu.cpu.set_reg(rd as usize, return_address as u64)?;
            
            let value = pc.wrapping_add_signed(imm as i64);
            emu.cpu.set_pc(value);

            increment_program_counter = false;
        }

            /* 
                Jalr
                The target address is obtained by adding the immediate to the register rs1, 
                then setting the least-significant bit of the result to zero.

                The address of the instruction following the jump (pc+4) is written to register rd 
            */

        Inst::Jalr { rd, rs1, imm } => {
            let pc = emu.cpu.get_pc();
            
            let return_address = pc + 4;
            emu.cpu.set_reg(rd as usize, return_address)?;
            
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let value = (rs1_val.wrapping_add_signed(imm as i64)) & !1;

            emu.cpu.set_pc(value);

            increment_program_counter = false;
        }

            /*
                BEQ and BNE take the branch if registers rs1 and rs2 are equal or unequal respectively
                BLT and BLTU take the branch if rs1 is less than rs2, using signed and unsigned comparison respectively. 
                BGE and BGEU take the branch if rs1 is greater than or equal to rs2, using signed and unsigned comparison respectively.
            */
        
        Inst::Beq { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;
            
            if rs1_val == rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }
        
        Inst::Bne { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;
            
            if rs1_val != rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }

        Inst::Blt { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i64;
            
            if rs1_val < rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }

        Inst::Bltu { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;
            
            if rs1_val < rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }

        Inst::Bge { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)? as i64;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)? as i64;
            
            if rs1_val >= rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }

        Inst::Bgeu { rs1, rs2, imm } => {
            let rs1_val = emu.cpu.get_reg(rs1 as usize)?;
            let rs2_val = emu.cpu.get_reg(rs2 as usize)?;
            
            if rs1_val >= rs2_val {
                let value = emu.cpu.get_pc().wrapping_add_signed(imm as i64);
            
                emu.cpu.set_pc(value);

                increment_program_counter = false;
            }

        }

            /* 
                The LD instruction loads a 64-bit value from memory into register rd for RV64I.
                The LW instruction loads a 32-bit value from memory and sign-extends this to 64 bits before storing it in register rd for RV64I. 
                The LWU instruction, on the other hand, zero-extends the 32-bit value from memory for RV64I. 
                LH and LHU are defined analogously for 16-bit values, as are LB and LBU for 8-bit values. 
                The SD, SW, SH, and SB instructions store 64-bit, 32-bit, 16-bit, and 8-bit values from the low
                bits of register rs2 to memory respectively
            */

            Inst::Ld { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 8;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lw { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 4;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                let value = value as i32 as i64 as u64;

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lwu { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                    
                let len = 4;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lh { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 2;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                let value = value as i16 as i64 as u64;

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lhu { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 2;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lb { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 1;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                let value = value as i8 as i64 as u64;

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Lbu { rd, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 1;
                let mut value = 0;

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_R == 0) {
                    return Err(CpuErr::ReadAccessFault(offset, len));
                }

                let buf = emu.mmu.dram_read(offset, len)?;

                for (i, val) in buf.iter().enumerate() {
                    value |= (*val as u64) << i*8;
                }

                emu.cpu.set_reg(rd as usize , value)?;
            }

            Inst::Sd { rs2, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 8; 

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_W == 0) {
                    return Err(CpuErr::WriteAccessFault(offset, len));
                }
            
                let value = emu.cpu.get_reg(rs2 as usize)?;

                let data = value.to_le_bytes();

                emu.mmu.dram_write(offset, &data[..len])?;

            }

            Inst::Sw { rs2, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 4; 

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_W == 0) {
                    return Err(CpuErr::WriteAccessFault(offset, len));
                }
            
                let value = emu.cpu.get_reg(rs2 as usize)?;

                let data = value.to_le_bytes();

                emu.mmu.dram_write(offset, &data[..len])?;

            }

            Inst::Sh { rs2, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 2; 

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_W == 0) {
                    return Err(CpuErr::WriteAccessFault(offset, len));
                }
            
                let value = emu.cpu.get_reg(rs2 as usize)?;

                let data = value.to_le_bytes();

                emu.mmu.dram_write(offset, &data[..len])?;

            }

            Inst::Sb { rs2, rs1, imm } => {
                let offset = emu.cpu.get_reg(rs1 as usize)?
                    .wrapping_add_signed(imm as i64) as usize;
                
                let len = 1; 

                let perms = emu.mmu.perm_get(offset, len)?;
                if perms.iter().any(|&perm| perm & PERM_W == 0) {
                    return Err(CpuErr::WriteAccessFault(offset, len));
                }
            
                let value = emu.cpu.get_reg(rs2 as usize)?;

                let data = value.to_le_bytes();

                emu.mmu.dram_write(offset, &data[..len])?;

            }

        _=> handle_undefined()
    }

    if increment_program_counter {
        inc_pc(&mut emu.cpu);
    }

    Ok(())
}


const X0: usize = 0;
const X1: usize = 1;
const X2: usize = 2;
const X3: usize = 3;
const X4: usize = 4;
const X5: usize = 5;
const X6: usize = 6;
const X7: usize = 7;
const X8: usize = 8;
const X9: usize = 9;
const X10: usize = 10;
const X11: usize = 11;
const X12: usize = 12;
const X13: usize = 13;
const X14: usize = 14;
const X15: usize = 15;
const X16: usize = 16;
const X17: usize = 17;
const X18: usize = 18;
const X19: usize = 19;
const X20: usize = 20;
const X21: usize = 21;
const X22: usize = 22;
const X23: usize = 23;
const X24: usize = 24;
const X25: usize = 25;
const X26: usize = 26;
const X27: usize = 27;
const X28: usize = 28;
const X29: usize = 29;
const X30: usize = 30;
const X31: usize = 31;