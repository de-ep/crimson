use std::{fs, io::Error, path::Path};
use super::memory::{self, Mmu};
use thiserror::Error;

const MAGIC_ELF: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];        //.ELF

enum FileType {
    Elf,
}

#[derive(Error, Debug)]
pub enum LoaderErr {
    #[error("Unable to read file: {0}")]
    UnableToReadFile(#[from] std::io::Error),

    #[error("Unsupported file type")]
    UnsupportedFileType,
    
    #[error("Invalid file")]
    InvalidFile,
    
    #[error("DRAM I/O Fail: {0}")]
    DramIoFail(#[from] memory::MmmuErr),
}

pub fn load_file_to_dram<P: AsRef<Path>>(mmu: &mut Mmu, file: &P) -> Result<usize, LoaderErr> {
    let data = fs::read(file)?;

    if data.len() > MAGIC_ELF.len() {
        if MAGIC_ELF == data[0..MAGIC_ELF.len()] {
            // right now loader does not support binaries compiled with pie
            return elf_loader::load_elf_to_dram(mmu, &data);
        } 
    }
    
    Err(LoaderErr::UnsupportedFileType)    
}

mod elf_loader {
    use crate::emulator::memory::{self, Mmu};
    use crate::emulator::loader::LoaderErr;

    const EI_NIDENT: usize = 16;
    const EI_CLASS: usize = 4;
    const ELFCLASS64: u8 = 02;
    const EI_DATA: usize =  5;
    const ELFDATA2LSB: u8 = 01;
    const ET_EXEC: u16 = 02;
    const EM_RISCV: u16 = 243;
    const PT_LOAD: u32 = 01;
    const PF_R:u32 = 0x4;
    const PF_W:u32 = 0x2;
    const PF_X:u32 = 0x1;

    /*
           typedef struct {
               uint32_t   sh_name;
               uint32_t   sh_type;
               uint64_t   sh_flags;
               Elf64_Addr sh_addr;
               Elf64_Off  sh_offset;
               uint64_t   sh_size;
               uint32_t   sh_link;
               uint32_t   sh_info;
               uint64_t   sh_addralign;
               uint64_t   sh_entsize;
           } Elf64_Shdr;
    */
    #[derive(Debug)]
    #[repr(C)]
    struct SectionHeader {
        sh_name: u32,
        sh_type: u32,
        sh_flags: u64,
        sh_addr: u64,
        sh_offset: u64,
        sh_size: u64,
        sh_link: u32,
        sh_info: u32,
        sh_addralign: u64,
        sh_entsize: u64,
    }

    /*           
             typedef struct {
               uint32_t   p_type;
               uint32_t   p_flagsz;
               Elf64_Off  p_offset;
               Elf64_Addr p_vaddr;
               Elf64_Addr p_paddr;
               uint64_t   p_filesz;
               uint64_t   p_memsz;
               uint64_t   p_align;
           } Elf64_Phdr; 
    */
    #[derive(Debug)]
    #[repr(C)]
    struct ProgramHeader{
        p_type: u32,
        p_flagsz: u32,
        p_offset: u64,
        p_vaddr: u64,
        p_paddr: u64,
        p_filesz: u64,
        p_memsz: u64,
        p_align: u64,
    }

    /*         
                 typedef struct {
                   unsigned char e_ident[EI_NIDENT];
                   uint16_t      e_type;
                   uint16_t      e_machine;
                   uint32_t      e_version;
                   ElfN_Addr     e_entry;
                   ElfN_Off      e_phoff;       //offset to prgram_header
                   ElfN_Off      e_shoff;
                   uint32_t      e_flags;
                   uint16_t      e_ehsize;
                   uint16_t      e_phentsize;   //size in bytes of one entry of program_header_table
                   uint16_t      e_phnum;       //number of entries in the program_header_table.
                   uint16_t      e_shentsize;
                   uint16_t      e_shnum;
                   uint16_t      e_shstrndx;
               } ElfN_Ehdr;

                The following types are used for N-bit architectures (N=32,64,
                ElfN stands for Elf32 or Elf64, uintN_t stands for uint32_t or
                uint64_t):
                    ElfN_Addr       Unsigned program address, uintN_t
                    ElfN_Off        Unsigned file offset, uintN_t
     */
    #[derive(Debug)]
    #[repr(C)]
    struct ElfHeader {
        e_ident: [u8; EI_NIDENT],
        e_type: u16,
        e_machine: u16,
        e_version: u32,
        e_entry: u64,
        e_phoff: u64,
        e_shoff: u64,
        e_flags: u32,
        e_ehsize: u16,
        e_phentsize: u16,
        e_phnum: u16,
        e_shentsize: u16,
        e_shnum: u16,
        e_shstrndx: u16,

    }
    #[derive(Debug)]
    struct Elf <'a>{
        elf_header: &'a ElfHeader,
        program_headers: Option<Vec<&'a ProgramHeader>>,
        section_headers: Option<Vec<&'a SectionHeader>>,
    }


    fn parse_elf(data: &[u8]) -> Result<Elf, LoaderErr> {
        let elf_header;
        let mut program_headers = None;
        let mut section_headers = None;


        //parsing elf_header
        if size_of::<ElfHeader>() > data.len() {
            return Err(LoaderErr::InvalidFile);
        }

        elf_header = unsafe {
            &*(data.as_ptr() as *const ElfHeader)
        };

        //EI_CLASS, architecture for this binary: 
        if elf_header.e_ident[EI_CLASS] != ELFCLASS64 {
            return Err(LoaderErr::UnsupportedFileType);
        } 


        //parsing program_headers
        if size_of::<ProgramHeader>() != elf_header.e_phentsize as usize {
            return Err(LoaderErr::InvalidFile);
        }
        if elf_header.e_phoff as usize + (elf_header.e_phentsize as usize * elf_header.e_phnum as usize) > data.len() {
            return Err(LoaderErr::InvalidFile);
        }
        if elf_header.e_phoff != 0 && elf_header.e_phnum == 0 {
            return Err(LoaderErr::InvalidFile);
        }
        
        if elf_header.e_phoff != 0 {
            let mut headers= Vec::new();
            
            for i in 0..elf_header.e_phnum {
                let ph_offset =  elf_header.e_phoff as usize + i as usize * size_of::<ProgramHeader>();

                headers.push(
                    unsafe {
                        &*((data.as_ptr().add(ph_offset)) as * const ProgramHeader)
                    }
                );
            }
            program_headers = Some(headers);
        }
        

        //parsing section_headers
        if size_of::<SectionHeader>() != elf_header.e_shentsize as usize {
            return Err(LoaderErr::InvalidFile);
        }
        if elf_header.e_shoff as usize + (elf_header.e_shentsize * elf_header.e_shnum) as usize > data.len() {
            return Err(LoaderErr::InvalidFile);
        }
        if elf_header.e_shoff != 0 && elf_header.e_shnum == 0 {
            return Err(LoaderErr::InvalidFile);
        }

        if elf_header.e_shoff != 0 {
            let mut headers= Vec::new();
            
            for i in 0..elf_header.e_shnum {
                let sh_offset =  elf_header.e_shoff as usize + i as usize * size_of::<SectionHeader>();

                headers.push(
                    unsafe {
                        &*((data.as_ptr().add(sh_offset)) as * const SectionHeader)
                    }
                );
            }
            section_headers = Some(headers);
        }



        Ok(
            Elf { 
                elf_header: elf_header, 
                program_headers: program_headers,
                section_headers: section_headers,     
            }
        )
    }


    pub fn load_elf_to_dram(mmu: &mut Mmu, data: &[u8]) -> Result<usize, LoaderErr> {
        let elf = parse_elf(data)?;

        
        //EI_DATA, endian
        if elf.elf_header.e_ident[EI_DATA] != ELFDATA2LSB {
            return Err(LoaderErr::UnsupportedFileType);
        }

        //identifies the object file type
        if elf.elf_header.e_type != ET_EXEC {
            return  Err(LoaderErr::UnsupportedFileType);
        }

        //e_machine TODO
        if elf.elf_header.e_machine != EM_RISCV {
            return  Err(LoaderErr::UnsupportedFileType);
        }

        if elf.elf_header.e_entry == 0 {
            return  Err(LoaderErr::UnsupportedFileType);
        }


        //loading program
        if let Some(program_headers) = elf.program_headers {
            for header in program_headers {
                if header.p_type == PT_LOAD {
                    let dest = header.p_vaddr as usize;
                    let src = header.p_offset as usize;
                    let size_in_file = header.p_filesz as usize;
                    let size_in_mem = header.p_memsz as usize;
                    let mut perm: u8 = 0;
                    let src_end = src + size_in_file;

                    if size_in_file > size_in_mem {
                        return Err(LoaderErr::InvalidFile);
                    }
                    
                    //mapping data to dram
                    if size_in_file != 0 {
                        mmu.dram_write(dest, &data[src..src_end])?;

                    }

                    if size_in_mem > size_in_file {
                        let vaddr: usize = dest + size_in_file;
                        let size: usize = size_in_mem - size_in_file;

                        mmu.dram_set(0, vaddr, size)?;
                    }
                    
                    //setting permissions
                    
                    if header.p_flagsz & PF_R == PF_R {
                        perm |= memory::PERM_R;
                    }
                    if header.p_flagsz & PF_W == PF_W {
                        perm |= memory::PERM_W;
                    }
                    if header.p_flagsz & PF_X == PF_X {
                        perm |= memory::PERM_X;
                    }

                    mmu.perm_set(dest, size_in_mem, perm)?;
                }
            }
            Ok(elf.elf_header.e_entry as usize)
        }
        else {
            Err(LoaderErr::InvalidFile)
        }
    }

}