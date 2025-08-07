const DRAM_SIZE: usize = 1024 * 1024;       //1MB
const MAX_DRAM_SIZE: usize = DRAM_SIZE * 8;

pub const PERM_R: u8 = 1;
pub const PERM_W: u8 = 1 << 1;
pub const PERM_X: u8 = 1 << 2;


#[derive(thiserror::Error, Debug)]
pub enum MmmuErr {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

pub struct Mmu {
    dram: Vec<u8>,
    perm: Vec<u8>,
}

macro_rules! bound_check {
    ($end: expr, $dram_size: expr) => {
        if $end > $dram_size {
            Err(MmmuErr::IndexOutOfBounds($end))
        }
        else {
            Ok(())
        }
    };
}

macro_rules! bound_check_and_resize {
    ($end: expr, $mmu: expr) => {{
        while let Err(_) = bound_check!($end, $mmu.dram.len()) {
            if $mmu.dram.len() * 2 <= MAX_DRAM_SIZE {
                $mmu.dram.resize($mmu.dram.len() * 2, 0);
                $mmu.perm.resize($mmu.perm.len() * 2, 0);
            }
            else {
                return Err(MmmuErr::IndexOutOfBounds($end));
            }
        }

        Ok(())
    }};
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            dram: vec![0; DRAM_SIZE],
            perm: vec![0; DRAM_SIZE],
        }
    }

    pub fn perm_get(&self, vaddr: usize, size: usize) -> Result<&[u8], MmmuErr> {
        let end = vaddr + size;

        bound_check!(end, self.dram.len())?;
        
        Ok(&self.perm[vaddr..end])
    } 

    pub fn perm_set(&mut self, vaddr: usize, size: usize, perm: u8) -> Result<(), MmmuErr> {
        let end = vaddr + size;

        bound_check_and_resize!(end, self)?;

        for p in &mut self.perm[vaddr..end] {
            *p = perm; 
        }

        Ok(())
    }

    pub fn dram_write(&mut self, vaddr: usize, data: &[u8]) -> Result<(), MmmuErr> {
        let end = vaddr + data.len();

        bound_check_and_resize!(end, self)?;

        self.dram[vaddr..end].copy_from_slice(data);

        Ok(())
    }

    pub fn dram_set(&mut self, val: u8, vaddr: usize, size: usize) -> Result<(), MmmuErr> {
        let end = vaddr + size;

        bound_check_and_resize!(end, self)?;

        self.dram[vaddr..end].fill(val);

        Ok(())
    }

    pub fn dram_read(&self, vaddr: usize, size: usize) -> Result<&[u8], MmmuErr> {
        let end = vaddr + size;

        bound_check!(end, self.dram.len())?;

        Ok(&self.dram[vaddr..end])
    }

}
