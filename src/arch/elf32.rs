pub use crate::arch::gabi::*;
pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;
pub mod e_ident {
    pub mod idx {
        pub use crate::arch::gabi::e_ident::idx::*;
        pub const EI_PAD: u8 = 7;
    }
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Ident {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub pad: [u8; (e_ident::idx::EI_NIDENT - e_ident::idx::EI_PAD) as usize],
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct ElfBasicType {}

#[derive(Default)]
pub struct Elf<'a> {
    pub writer: Option<Box<(dyn std::io::Write + 'a)>>,
    pub reader: Option<Box<(dyn std::io::Read + 'a)>>,
    pub seeker: Option<Box<(dyn std::io::Read + 'a)>>,
    pub file: Option<&'a mut std::fs::File>,
    pub ehdr: Box<Header>,
}
impl<'a> Elf<'a> {
    pub fn new(file: &'a mut std::fs::File) -> Elf {
        let mut ret = Elf {
            ..Default::default()
        };
        ret.reader = Some(Box::new(file.try_clone().unwrap()));
        ret.writer = Some(Box::new(file.try_clone().unwrap()));
        ret.seeker = Some(Box::new(file.try_clone().unwrap()));
        ret.file = Some(file);
        ret
    }

    pub fn set_reader(&'a mut self, r: &'a mut (dyn std::io::Read + 'a)) {
        self.reader = Some(Box::new(r));
    }

    pub fn read_ehdr(&mut self) -> &Box<Header> {
        match self.reader.as_mut() {
            Some(r) => {
                r.read(self.ehdr.as_bytes_mut()).unwrap();
            }
            None => {}
        }
        &self.ehdr
    }
}

impl crate::BasicType for ElfBasicType {
    type Addr = u32;
    type Half = u16;
    type Off = u32;
    type Word = u32;
    type Sword = i32;
    type Xword = Self::Word;
    type Sxword = Self::Sword;
}

pub type Header = crate::arch::gabi::Header<ElfBasicType, Ident>;

pub mod e_type {
    crate::define_e_type_basic_const!(<super::ElfBasicType as crate::BasicType>::Half);
}

#[test]
fn test_file_open() {
    let mut file = std::fs::File::open("./elf").unwrap();
    let mut elf = Elf::new(&mut file);
    println!("{:?}", elf.read_ehdr());
}
