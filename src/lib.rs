pub mod arch;
pub mod utils;

use std::{cmp, convert::TryInto, fmt::Debug, ops};
use utils::AsBytes;

pub trait BasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>
        + ops::BitOrAssign
        + ops::BitOr<Output = Self::Xword>
        + ops::BitAndAssign
        + ops::BitAnd<Output = Self::Xword>
        + cmp::PartialEq;
    type Sxword: Debug + Default + From<i32> + Copy;
    type Addr: Debug + Default + From<u32> + Into<u64> + Copy;
    type Half: Debug + Default + From<u16> + Copy;
    type Off: Debug + Default + From<u32> + Copy;
    type Sword: Debug + Default + From<i32> + Copy;
    type Word: Debug
        + Default
        + Into<u32>
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>;
}

#[non_exhaustive]
pub enum Elf<'a> {
    Elf64(elf64::Elf<'a>),
    Elf32(elf32::Elf<'a>),
}

use arch::{
    elf32, elf64,
    gabi::e_ident::{
        self,
        ei_class::{ELFCLASS32, ELFCLASS64},
    },
};
use std::io::Read;
use Elf::{Elf32, Elf64};

fn is_elf(ident: &[u8; e_ident::idx::EI_NIDENT]) -> bool {
    if [
        e_ident::ei_mag::ELFMAG0,
        e_ident::ei_mag::ELFMAG1,
        e_ident::ei_mag::ELFMAG2,
        e_ident::ei_mag::ELFMAG3,
    ] == ident[e_ident::idx::EI_MAG0..e_ident::idx::EI_MAG3 + 1]
    {
        true
    } else {
        false
    }
}
use std::io::Seek;
pub fn new<'a>(file: &'a mut std::fs::File) -> Option<Elf<'a>> {
    let mut ident: [u8; e_ident::idx::EI_NIDENT] = [0; e_ident::idx::EI_NIDENT];
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.read(&mut ident).unwrap();

    if is_elf(&ident) {
        match ident[e_ident::idx::EI_CLASS] {
            ELFCLASS32 => Some(Elf32(elf32::Elf::new(file))),
            ELFCLASS64 => Some(Elf64(elf64::Elf::new(file))),
            _ => None,
        }
    } else {
        None
    }
}

#[test]
fn test_new() -> Result<(), ()> {
    let mut file = std::fs::File::open("./test/elf64_example").unwrap();
    match self::new(&mut file) {
        Some(elf) => match elf {
            Elf64(mut v) => {
                println!("Elf64: {:?}", v.read_ehdr());
                Ok(())
            }
            Elf32(mut v) => {
                println!("Elf32: {:?}", v.read_ehdr());
                Ok(())
            }
        },
        None => Err(()),
    }
}
