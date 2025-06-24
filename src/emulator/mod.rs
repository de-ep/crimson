pub mod emulator {
    const SIZE_INSTRUCTION_TYPE_LOOKUP_TABLE: usize  = 128;
    const DRAM_SIZE:usize = 1024 * 1024;

    enum InstType {
        R,
        I,
        S,
        B,
        U,
        J,
    }

    enum Inst {
        InvalidInst,
    }

    struct Emulator {
        r: [u64; 32],
        pc: u64,
        dram: Vec<u8>,
    }

    impl Emulator {
        fn new() -> Self {

            Emulator {
                r: [0; 32],
                pc: 0,
                dram: vec![0; DRAM_SIZE],
            }
        }

        fn fetch(&self) -> u32 {
            
            0
        } 

        fn decode(&self, inst: u32) -> Inst{
            if (inst <= 0) {
                return Inst::InvalidInst;
            }

            let opcode = inst & 0b111_111_1;

            if let Some(inst_type) = &INSTRUCTION_TYPE_LOOKUP_TABLE[opcode as usize] {
                match inst_type {
                    InstType::R => {}
                    InstType::I => {}
                    InstType::S => {}
                    InstType::B => {}
                    InstType::U => {}
                    InstType::J => {}
            
                    _=> unreachable!(),
                }
            }

            Inst::InvalidInst
        }

        fn exec(&self, inst: Inst) {
            
            match inst {

                _ => {}
            }
        }

    }
     
    
    fn emulate(bytes: Vec<u8>) {

    }


    const INSTRUCTION_TYPE_LOOKUP_TABLE: [Option<InstType>; SIZE_INSTRUCTION_TYPE_LOOKUP_TABLE] = [
    /*        0 */ None,
    /*        1 */ None,
    /*       10 */ None,
    /*       11 */ None,
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
    /*     1111 */ None,
    /*    10000 */ None,
    /*    10001 */ None,
    /*    10010 */ None,
    /*    10011 */ None,
    /*    10100 */ None,
    /*    10101 */ None,
    /*    10110 */ None,
    /*    10111 */ None,
    /*    11000 */ None,
    /*    11001 */ None,
    /*    11010 */ None,
    /*    11011 */ None,
    /*    11100 */ None,
    /*    11101 */ None,
    /*    11110 */ None,
    /*    11111 */ None,
    /*   100000 */ None,
    /*   100001 */ None,
    /*   100010 */ None,
    /*   100011 */ None,
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
    /*   110011 */ None,
    /*   110100 */ None,
    /*   110101 */ None,
    /*   110110 */ None,
    /*   110111 */ None,
    /*   111000 */ None,
    /*   111001 */ None,
    /*   111010 */ None,
    /*   111011 */ None,
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
    /*  1100011 */ None,
    /*  1100100 */ None,
    /*  1100101 */ None,
    /*  1100110 */ None,
    /*  1100111 */ None,
    /*  1101000 */ None,
    /*  1101001 */ None,
    /*  1101010 */ None,
    /*  1101011 */ None,
    /*  1101100 */ None,
    /*  1101101 */ None,
    /*  1101110 */ None,
    /*  1101111 */ None,
    /*  1110000 */ None,
    /*  1110001 */ None,
    /*  1110010 */ None,
    /*  1110011 */ None,
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

}