pub mod arch;
pub mod utils;

use std::{cmp, convert::TryInto, fmt::Debug, ops};
use utils::AsBytes;

#[derive(Debug)]
pub enum Error {
    DataLoss,
    InvalidMagic,
    InvalidClass,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

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

pub trait Validity {
    fn is_valid(&self) -> std::io::Result<()>;
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

pub(crate) fn is_elf(ident: &[u8]) -> bool {
    if ident.len() < e_ident::idx::EI_MAG3 {
        false
    } else if [
        e_ident::ei_mag::ELFMAG0,
        e_ident::ei_mag::ELFMAG1,
        e_ident::ei_mag::ELFMAG2,
        e_ident::ei_mag::ELFMAG3,
    ] == ident[e_ident::idx::EI_MAG0..=e_ident::idx::EI_MAG3]
    {
        true
    } else {
        false
    }
}
use std::io::{self, Seek};
pub fn new<'a>(file: &'a mut std::fs::File) -> io::Result<Option<Elf<'a>>> {
    let mut ident: [u8; e_ident::idx::EI_NIDENT] = [0; e_ident::idx::EI_NIDENT];
    file.seek(io::SeekFrom::Start(0)).unwrap();
    file.read(&mut ident).unwrap();

    if is_elf(&ident) {
        match ident[e_ident::idx::EI_CLASS] {
            ELFCLASS32 => Ok(Some(Elf32(elf32::Elf::new_without_class_check(file)))),
            ELFCLASS64 => Ok(Some(Elf64(elf64::Elf::new_without_class_check(file)))),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                Error::InvalidClass,
            )),
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            Error::InvalidMagic,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() -> Result<(), ()> {
        let mut file = std::fs::File::open("./test/elf64_example").unwrap();
        match self::new(&mut file).unwrap() {
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
    #[test]
    fn test_is_elf_with_err_data() -> Result<(), ()> {
        let err_data = [0x7f, 'e' as u8];
        if is_elf(&err_data) {
            Err(())
        } else {
            Ok(())
        }
    }
}
