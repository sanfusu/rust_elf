pub mod arch;
pub mod utils;

use std::{
    cmp,
    convert::TryInto,
    fmt::{self, Debug},
    ops,
};
use utils::AsBytes;

#[derive(Debug)]
pub enum Error {
    DataLoss,
    InvalidShentSize,
    InvalidMagic,
    InvalidClass,
    InvalidEhdr,
}

impl std::convert::Into<io::Error> for Error {
    fn into(self) -> io::Error {
        let x = match self {
            Error::DataLoss => (io::ErrorKind::InvalidInput, Error::DataLoss),
            Error::InvalidMagic => (io::ErrorKind::InvalidInput, Error::DataLoss),
            Error::InvalidClass => (io::ErrorKind::InvalidInput, Error::DataLoss),
            Error::InvalidEhdr => (io::ErrorKind::InvalidData, Error::InvalidEhdr),
            Error::InvalidShentSize => (io::ErrorKind::InvalidData, Error::InvalidShentSize),
        };
        io::Error::new(x.0, x.1)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

pub trait IBasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>
        + ops::BitOrAssign
        + ops::BitOr<Output = Self::Xword>
        + ops::BitAndAssign
        + ops::BitAnd<Output = Self::Xword>
        + cmp::PartialEq
        + fmt::LowerHex;
    type Sxword: Debug + fmt::LowerHex + Default + From<i32> + Copy;
    type Addr: Debug + Default + fmt::LowerHex + From<u32> + Into<u64> + Copy;
    type Half: Debug
        + Default
        + From<u16>
        + fmt::LowerHex
        + Copy
        + ops::Mul<Output = Self::Half>
        + Into<u64>;
    type Off: Debug + Default + From<u32> + Copy + fmt::LowerHex + Into<u64>;
    type Sword: Debug + Default + From<i32> + fmt::LowerHex + Copy;
    type Word: Debug
        + Default
        + Into<u32>
        + From<u32>
        + fmt::LowerHex
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

pub(crate) fn is_elf(ident: &[u8]) -> io::Result<bool> {
    if ident.len() < e_ident::idx::EI_MAG3 {
        Err(Error::DataLoss.into())
    } else if [
        e_ident::ei_mag::ELFMAG0,
        e_ident::ei_mag::ELFMAG1,
        e_ident::ei_mag::ELFMAG2,
        e_ident::ei_mag::ELFMAG3,
    ] == ident[e_ident::idx::EI_MAG0..=e_ident::idx::EI_MAG3]
    {
        Ok(true)
    } else {
        Err(Error::InvalidMagic.into())
    }
}
use std::io::{self, Seek};
pub fn new<'a>(file: &'a mut std::fs::File) -> io::Result<Option<Elf<'a>>> {
    let mut ident: [u8; e_ident::idx::EI_NIDENT] = [0; e_ident::idx::EI_NIDENT];
    file.seek(io::SeekFrom::Start(0))?;
    file.read(&mut ident)?;

    is_elf(&ident)?;
    match ident[e_ident::idx::EI_CLASS] {
        ELFCLASS32 => Ok(Some(Elf32(elf32::Elf::new_without_validity_check(file)))),
        ELFCLASS64 => Ok(Some(Elf64(elf64::Elf::new_without_validity_check(file)))),
        _ => Err(Error::InvalidClass.into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() -> io::Result<()> {
        let mut file = std::fs::File::open("./test/elf64_example")?;
        match self::new(&mut file)? {
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
            None => Err(Error::InvalidClass.into()),
        }
    }
    #[test]
    fn test_is_elf_with_err_data() -> io::Result<()> {
        let err_data = [0x7f, 'e' as u8];
        is_elf(&err_data).expect_err("msg");
        Ok(())
    }
}
