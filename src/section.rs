pub mod sh_type;

use core::convert::TryInto;

use elface::{MetaData, MetaDataError};

use self::sh_type::ShType;
use super::{Addr, Off, Word, Xword};

#[derive(Layout, MetaData)]
#[repr(packed)]
pub struct Shdr {
    pub sh_name: Word,
    pub sh_type: ShType,
    pub sh_flags: Xword,
    pub sh_addr: Addr,
    pub sh_offset: Off,
    pub sh_size: Xword,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Xword,
    pub sh_entsize: Xword,
}

impl Shdr {
    pub fn try_from_be_bytes(src: &[u8]) -> Result<Self, MetaDataError> {
        Ok(unsafe { Self::from_be_bytes(src.try_into().map_err(|_| MetaDataError)?) })
    }
    pub fn try_from_le_bytes(src: &[u8]) -> Result<Self, MetaDataError> {
        Ok(unsafe { Self::from_le_bytes(src.try_into().map_err(|_| MetaDataError)?) })
    }
}
