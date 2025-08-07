const SIZE_INSTRUCTION_TYPE_LOOKUP_TABLE: usize = 128;

//we need to make this shit more robust
#[derive(Debug, Clone, Copy)]
pub enum InstType {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    //Rtype
    Add {rd: u32, rs1: u32, rs2: u32},
    Sub {rd: u32, rs1: u32, rs2: u32},
    Sll {rd: u32, rs1: u32, rs2: u32},
    Slt {rd: u32, rs1: u32, rs2: u32},
    Sltu {rd: u32, rs1: u32, rs2: u32},
    Xor {rd: u32, rs1: u32, rs2: u32},
    Srl {rd: u32, rs1: u32, rs2: u32},
    Sra {rd: u32, rs1: u32, rs2: u32},
    Or {rd: u32, rs1: u32, rs2: u32},
    And {rd: u32, rs1: u32, rs2: u32},
    Addw {rd: u32, rs1: u32, rs2: u32},
    Subw {rd: u32, rs1: u32, rs2: u32},
    Sllw {rd: u32, rs1: u32, rs2: u32},
    Srlw {rd: u32, rs1: u32, rs2: u32},
    Sraw {rd: u32, rs1: u32, rs2: u32},

    //Itype
    Jalr {rd: u32, rs1: u32, imm: i32},
    Lb {rd: u32, rs1: u32, imm: i32},
    Lh {rd: u32, rs1: u32, imm: i32},
    Lw {rd: u32, rs1: u32, imm: i32},
    Lbu {rd: u32, rs1: u32, imm: i32},
    Lhu {rd: u32, rs1: u32, imm: i32},
    Addi {rd: u32, rs1: u32, imm: i32},
    Slti {rd: u32, rs1: u32, imm: i32},
    Sltiu {rd: u32, rs1: u32, imm: i32},
    Xori {rd: u32, rs1: u32, imm: i32},
    Ori {rd: u32, rs1: u32, imm: i32},
    Andi {rd: u32, rs1: u32, imm: i32},
    Slli {rd: u32, rs1: u32, shamt: u32},
    Srli {rd: u32, rs1: u32, shamt: u32},
    Srai {rd: u32, rs1: u32, shamt: u32},
    Fence {rd: u32, rs1: u32, imm_raw: u32},
    FenceTso,
    Pause,
    Ecall,
    Ebreak,
    Lwu {rd: u32, rs1: u32, imm: i32},
    Ld {rd: u32, rs1: u32, imm: i32},
    Addiw {rd: u32, rs1: u32, imm: i32},
    Slliw {rd: u32, rs1: u32, shamt: u32},
    Srliw {rd: u32, rs1: u32, shamt: u32},
    Sraiw {rd: u32, rs1: u32, shamt: u32},

    //Stype
    Sb {rs2: u32, rs1: u32, imm: i32},
    Sh {rs2: u32, rs1: u32, imm: i32},
    Sw {rs2: u32, rs1: u32, imm: i32},
    Sd {rs2: u32, rs1: u32, imm: i32},

    //Btype
    Beq {rs1: u32, rs2: u32, imm: i32},
    Bne {rs1: u32, rs2: u32, imm: i32},
    Blt {rs1: u32, rs2: u32, imm: i32},
    Bge {rs1: u32, rs2: u32, imm: i32},
    Bltu {rs1: u32, rs2: u32, imm: i32}, 
    Bgeu {rs1: u32, rs2: u32, imm: i32}, 

    //Utype 
    Lui {rd: u32, imm: i32},
    Auipc {rd: u32, imm: i32},
    
    //Jtype
    Jal {rd: u32, imm: i32},

    Undefined,
}



pub fn fetch_inst_type(inst: u32) -> Option<InstType> {
    let opcode = inst & 0b1111_111;
    
    if opcode as usize >= SIZE_INSTRUCTION_TYPE_LOOKUP_TABLE {
        return None;
    }

    INSTRUCTION_TYPE_LOOKUP_TABLE[opcode as usize]

}

pub fn decode(inst: u32) -> Inst {
    if inst <= 0 {
        return Inst::Undefined;
    }

    let opcode = inst & 0b1111_111;

    if let Some(inst_type) = fetch_inst_type(inst) {
        match inst_type {
            
            InstType::R => {
                let opcode = opcode;
                let rd = (inst >> 7) & 0b1111_1;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = (inst >> 15) & 0b1111_1;
                let rs2 = (inst >> 20) & 0b1111_1; 
                let funct7 = (inst >> 25) & 0b1111_111;

                match opcode {
                    0b0110011 => {
                        match funct3 {
                            0b000 => {
                                match funct7 {
                                    0b0000000 => return Inst::Add { rd: rd, rs1: rs1, rs2: rs2 },
                                    0b0100000 => return Inst::Sub { rd: rd, rs1: rs1, rs2: rs2 },
                                    _=> return Inst::Undefined
                                }
                            }
                            0b001 => return Inst::Sll { rd: rd, rs1: rs1, rs2: rs2 },
                            0b010 => return Inst::Slt { rd: rd, rs1: rs1, rs2: rs2 },
                            0b011 => return Inst::Sltu { rd: rd, rs1: rs1, rs2: rs2 },
                            0b100 => return Inst::Xor { rd: rd, rs1: rs1, rs2: rs2 },
                            0b101 => {
                                match funct7 {
                                    0b0000000 => return Inst::Srl { rd: rd, rs1: rs1, rs2: rs2 },
                                    0b0100000 => return Inst::Sra { rd: rd, rs1: rs1, rs2: rs2 },
                                    _=> return Inst::Undefined
                                }
                            }
                            0b110 => return Inst::Or { rd: rd, rs1: rs1, rs2: rs2 },
                            0b111 => return Inst::And { rd: rd, rs1: rs1, rs2: rs2 },
                            _=> return Inst::Undefined
                        }
                    }
                    0b0111011 => {
                        match funct3 {
                            0b000 => {
                                match funct7 {
                                    0b0000000 => return Inst::Addw { rd: rd, rs1: rs1, rs2: rs2 },
                                    0b0100000 => return Inst::Subw { rd: rd, rs1: rs1, rs2: rs2 },
                                    _=> return Inst::Undefined
                                }
                            }
                            0b001 => return Inst::Sllw { rd: rd, rs1: rs1, rs2: rs2 },
                            0b101 => {
                                match funct7 {
                                    0b0000000 => return Inst::Srlw { rd: rd, rs1: rs1, rs2: rs2 },
                                    0b0100000 => return Inst::Sraw { rd: rd, rs1: rs1, rs2: rs2 },
                                    _=> return Inst::Undefined
                                }
                            }
                            _=> return Inst::Undefined,
                        }
                    }                       
                    _=> return Inst::Undefined
                }   
            }
            InstType::I => {
                let opcode = opcode;
                let rd = (inst >> 7) & 0b1111_1;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = (inst >> 15) & 0b1111_1;
                let imm = (inst >> 20) &0b1111_1111_1111 ;

                //incase inst has shamt 
                let shamt = (inst >> 20) & 0b1111_1; 
                let funct7 = (inst >> 25) & 0b1111_111;

                //sign extending imm
                let imm_raw = imm;
                let imm = ((imm as i32) << 20) >> 20;

                match opcode {
                    0b1100111 => return Inst::Jalr { rd: rd, rs1: rs1, imm: imm },
                    0b0000011 => {
                        match funct3 {
                            0b000 => return Inst::Lb { rd: rd, rs1: rs1, imm: imm },
                            0b001 => return Inst::Lh { rd: rd, rs1: rs1, imm: imm },
                            0b010 => return Inst::Lw { rd: rd, rs1: rs1, imm: imm },
                            0b100 => return Inst::Lbu { rd: rd, rs1: rs1, imm: imm },
                            0b101 => return Inst::Lhu { rd: rd, rs1: rs1, imm: imm },
                            0b110 => return Inst::Lwu { rd: rd, rs1: rs1, imm: imm },
                            0b011 => return Inst::Ld { rd: rd, rs1: rs1, imm: imm },
                            _=> return Inst::Undefined
                        }
                    }
                    0b0010011 => {
                        match funct3 {
                            0b000 => return Inst::Addi { rd: rd, rs1: rs1, imm: imm },
                            0b010 => return Inst::Slti { rd: rd, rs1: rs1, imm: imm },
                            0b011 => return Inst::Sltiu { rd: rd, rs1: rs1, imm: imm },
                            0b100 => return Inst::Xori  { rd: rd, rs1: rs1, imm: imm },
                            0b110 => return Inst::Ori  { rd: rd, rs1: rs1, imm: imm },
                            0b111 => return Inst::Andi { rd: rd, rs1: rs1, imm: imm },
                            0b001 => return Inst::Slli { rd: rd, rs1: rs1, shamt: shamt },
                            0b101 => {
                                match funct7 {
                                    0b0000000 => return Inst::Srli { rd: rd, rs1: rs1, shamt: shamt },
                                    0b0100000 => return Inst::Srai { rd: rd, rs1: rs1, shamt: shamt },
                                    _=> return Inst::Undefined
                                }
                            }
                            _=> return Inst::Undefined
                        }
                    }
                    0b0001111 => {
                        match (rd, rs1, funct3) {
                               (0, 0, 0) => {
                                    match imm_raw {
                                        0b100000110011 => return Inst::FenceTso,
                                        0b000000010000 => return Inst::Pause,
                                        _=> {},
                                    }
                                }     
                               (_, _, 0) => return Inst::Fence { rd: rd, rs1: rs1, imm_raw: imm_raw },                                
                               _=> return Inst::Undefined,
                           }
                        
                        
                    }
                    0b1110011 => {
                        if rd == 0 && funct3 == 0 && rs1 == 0 {
                            match imm {
                                0 => return Inst::Ecall,
                                1 => return Inst::Ebreak,
                                _=> return Inst::Undefined,
                            }
                        }
                        return Inst::Undefined;
                    }
                    0b0011011 => {
                        match funct3 {
                            0b000 => return Inst::Addiw { rd: rd, rs1: rs1, imm: imm },
                            0b001 => return Inst::Slliw { rd: rd, rs1: rs1, shamt: shamt },
                            0b101 => {
                                match funct7 >> 5 {
                                    0b0000000 => return Inst::Srliw { rd: rd, rs1: rs1, shamt: shamt },
                                    0b0100000 => return Inst::Sraiw { rd: rd, rs1: rs1, shamt: shamt },
                                    _=> return Inst::Undefined,
                                }
                            }

                            _=> return Inst::Undefined,
                        }
                    }
                    _=> return Inst::Undefined
                }
            }
            InstType::S => {
                let opcode = opcode;
                let imm1 = (inst >> 7) & 0b1111_1;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = (inst >> 15) & 0b1111_1;
                let rs2 = (inst >> 20) & 0b1111_1; 
                let imm2 = (inst >> 25) & 0b1111_111;

                //merging and sign extending imm
                let imm = (imm1) | (imm2 << 5);
                let imm = ((imm as i32) << 20) >> 20;

                match opcode {
                    0b0100011 => {
                        match funct3 {
                            0b000 => return Inst::Sb { rs1: rs1, rs2: rs2, imm: imm },
                            0b001 => return Inst::Sh { rs1: rs1, rs2: rs2, imm: imm },
                            0b010 => return Inst::Sw { rs1: rs1, rs2: rs2, imm: imm },
                            0b011 => return Inst::Sd { rs1: rs1, rs2: rs2, imm: imm },
                            _=> return Inst::Undefined,
                        }
                    }
                    _=> return Inst::Undefined,
                }
            }
            InstType::B => {
                let opcode = opcode;
                let imm1 = (inst >> 7) & 0b1111_1;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = (inst >> 15) & 0b1111_1;
                let rs2 = (inst >> 20) & 0b1111_1;
                let imm2 = (inst >> 25) & 0b1111_111;

                //merging and sign extending imm
                let imm11 = (imm1 & 1) << 11;
                let imm41 = (imm1 >> 1) << 1;
                let imm105 = (imm2 & 0b1111_11) << 5;
                let imm12 = (imm2 >> 6) << 12;
                let imm = imm41 | imm105 | imm11 | imm12;
                let imm = ((imm as i32) << 20) >> 20;  

                match opcode {
                    0b1100011 => {
                        match funct3 {
                            0b000 => return Inst::Beq { rs1: rs1, rs2: rs2, imm: imm }, 
                            0b001 => return Inst::Bne { rs1: rs1, rs2: rs2, imm: imm }, 
                            0b100 => return Inst::Blt { rs1: rs1, rs2: rs2, imm: imm }, 
                            0b101 => return Inst::Bge { rs1: rs1, rs2: rs2, imm: imm }, 
                            0b110 => return Inst::Bltu { rs1: rs1, rs2: rs2, imm: imm },  
                            0b111 => return Inst::Bgeu { rs1: rs1, rs2: rs2, imm: imm }, 
                            _=> return Inst::Undefined,
                        }
                    }
                    _=> return Inst::Undefined,
                }
            }
            InstType::U => {
                let opcode = opcode;
                let rd = (inst >> 7) & 0b1111_1;
                let imm = inst >> 12;

                //sign extend imm
                let imm = ((inst as i32) << 12) >> 12;

                match opcode {
                    0b0110111 => return Inst::Lui { rd: rd, imm: imm },
                    0b0010111 => return Inst::Auipc { rd: rd, imm: imm },
                    _=> return Inst::Undefined,
                }
            }
            InstType::J => {
                let opcode = opcode;
                let rd = (inst >> 7 ) & 0b1111_1;
                let imm = inst >> 12;

                //merging and sign extending imm
                let imm1912 = ((inst >> 12) & 0b1111_1111) << 12;
                let imm11 = ((inst >> 20) & 1) << 11;
                let imm101 = ((inst >> 21) & 0b1111_1111_11) << 1;
                let imm20 = (inst >> 31) << 20;
                let imm = imm101 | imm11 | imm1912 | imm20;
                let imm = ((imm as i32) << 12) >> 12;

                match opcode {
                    0b1101111 => return Inst::Jal { rd: rd, imm: imm },
                    _=> return Inst::Undefined,
                }
            }
        }
    }

    return Inst::Undefined
}

const INSTRUCTION_TYPE_LOOKUP_TABLE: [Option<InstType>; SIZE_INSTRUCTION_TYPE_LOOKUP_TABLE] = [
    /*        0 */ None,
    /*        1 */ None,
    /*       10 */ None,
    /*       11 */ Some(InstType::I),
    /*      100 */ None,
    /*      101 */ None,
    /*      110 */ None,
    /*      111 */ None,
    /*     1000 */ None,
    /*     1001 */ None,
    /*     1010 */ None,
    /*     1011 */ None,
    /*     1100 */ None,
    /*     1101 */ None,
    /*     1110 */ None,
    /*     1111 */ Some(InstType::I),
    /*    10000 */ None,
    /*    10001 */ None,
    /*    10010 */ None,
    /*    10011 */ Some(InstType::I),
    /*    10100 */ None,
    /*    10101 */ None,
    /*    10110 */ None,
    /*    10111 */ Some(InstType::U),
    /*    11000 */ None,
    /*    11001 */ None,
    /*    11010 */ None,
    /*    11011 */ Some(InstType::I),
    /*    11100 */ None,
    /*    11101 */ None,
    /*    11110 */ None,
    /*    11111 */ None,
    /*   100000 */ None,
    /*   100001 */ None,
    /*   100010 */ None,
    /*   100011 */ Some(InstType::S),
    /*   100100 */ None,
    /*   100101 */ None,
    /*   100110 */ None,
    /*   100111 */ None,
    /*   101000 */ None,
    /*   101001 */ None,
    /*   101010 */ None,
    /*   101011 */ None,
    /*   101100 */ None,
    /*   101101 */ None,
    /*   101110 */ None,
    /*   101111 */ None,
    /*   110000 */ None,
    /*   110001 */ None,
    /*   110010 */ None,
    /*   110011 */ Some(InstType::R),
    /*   110100 */ None,
    /*   110101 */ None,
    /*   110110 */ None,
    /*   110111 */ Some(InstType::U),
    /*   111000 */ None,
    /*   111001 */ None,
    /*   111010 */ None,
    /*   111011 */ Some(InstType::R),
    /*   111100 */ None,
    /*   111101 */ None,
    /*   111110 */ None,
    /*   111111 */ None,
    /*  1000000 */ None,
    /*  1000001 */ None,
    /*  1000010 */ None,
    /*  1000011 */ None,
    /*  1000100 */ None,
    /*  1000101 */ None,
    /*  1000110 */ None,
    /*  1000111 */ None,
    /*  1001000 */ None,
    /*  1001001 */ None,
    /*  1001010 */ None,
    /*  1001011 */ None,
    /*  1001100 */ None,
    /*  1001101 */ None,
    /*  1001110 */ None,
    /*  1001111 */ None,
    /*  1010000 */ None,
    /*  1010001 */ None,
    /*  1010010 */ None,
    /*  1010011 */ None,
    /*  1010100 */ None,
    /*  1010101 */ None,
    /*  1010110 */ None,
    /*  1010111 */ None,
    /*  1011000 */ None,
    /*  1011001 */ None,
    /*  1011010 */ None,
    /*  1011011 */ None,
    /*  1011100 */ None,
    /*  1011101 */ None,
    /*  1011110 */ None,
    /*  1011111 */ None,
    /*  1100000 */ None,
    /*  1100001 */ None,
    /*  1100010 */ None,
    /*  1100011 */ Some(InstType::B),
    /*  1100100 */ None,
    /*  1100101 */ None,
    /*  1100110 */ None,
    /*  1100111 */ Some(InstType::I),
    /*  1101000 */ None,
    /*  1101001 */ None,
    /*  1101010 */ None,
    /*  1101011 */ None,
    /*  1101100 */ None,
    /*  1101101 */ None,
    /*  1101110 */ None,
    /*  1101111 */ Some(InstType::J),
    /*  1110000 */ None,
    /*  1110001 */ None,
    /*  1110010 */ None,
    /*  1110011 */ Some(InstType::I),
    /*  1110100 */ None,
    /*  1110101 */ None,
    /*  1110110 */ None,
    /*  1110111 */ None,
    /*  1111000 */ None,
    /*  1111001 */ None,
    /*  1111010 */ None,
    /*  1111011 */ None,
    /*  1111100 */ None,
    /*  1111101 */ None,
    /*  1111110 */ None,
    /*  1111111 */ None,
];