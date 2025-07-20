const DRAM_SIZE: usize = 1024 * 1024;

const PERM_R: u8 = 1;
const PERM_W: u8 = 1 << 1;
const PERM_X: u8 = 1 << 2;


pub struct Mmu {
    dram: Vec<u8>,
    perm: Vec<u8>,
    dram_size: usize,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            dram: vec![0; DRAM_SIZE],
            perm: vec![0; DRAM_SIZE],
            dram_size: DRAM_SIZE,
        }
    }

    pub fn perm_get<'a>(&'a self, vaddr: usize, size: usize) -> Option<&'a [u8]> {
        let end = vaddr + size;

        if end > self.perm.len() {
            return None;
        }
        
        Some(&self.perm[vaddr..end])
    } 

    pub fn perm_set(&mut self, vaddr: usize, size: usize, perm: u8) -> Result<(), ()> {
        let end = vaddr + size;

        if end > self.perm.len() {
            return Err(());
        }

        for p in &mut self.perm[vaddr..end] {
            *p = perm; 
        }

        Ok(())
    }

    pub fn dram_write(&mut self, vaddr: usize, data: &[u8]) -> Result<(), ()> {
        let end = vaddr + data.len();

        if end > self.dram.len() {
            return Err(());
        }

        self.dram[vaddr..end].copy_from_slice(data);

        Ok(())
    }

    pub fn dram_read<'a>(&'a self, vaddr: usize, size: usize) -> Option<&'a [u8]> {
        let end = vaddr + size;

        if end > self.dram.len() {
            return None;
        }

        Some(&self.dram[vaddr..end])
    }

}
