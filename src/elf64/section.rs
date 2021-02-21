pub mod sh_type;

use self::sh_type::ShType;
use super::{Addr, Off, Word, Xword};

#[derive(MetaData)]
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
