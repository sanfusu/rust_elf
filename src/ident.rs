#[repr(C)]
pub struct Ident {
    pub magic: [u8; 4],
    pub file_class: ObjectFileClass,
    pub data_encode: DataEncode,
    pub file_version: u8,
    pub osabi: Osabi,
    pub abi_version: u8,
}

impl_borrow!(Ident);

def_trans! {
    DataEncode: u8,
    ObjectFileClass:u8,
    Osabi:u8
}

const_enum::const_enum! {
    pub ObjectFileClassEnum [ObjectFileClass::data: u8] {
        ELFCLASS32: 1,
        ELFCLASS64: 2
    }
    pub DataEncodeEnum [DataEncode::data: u8] {
        LSB: 1,
        MSB: 2
    }
    pub OsabiEnum [Osabi::data: u8] {
        /// System V ABI
        SYSV: 0,
        /// HP-UX 操作系统
        HPUX: 1,
        /// 独立的嵌入式应用（无操作系统）
        STANDLONE: 255
    }
}

impl Ident {
    pub fn verify(&self) -> Result<(), ()> {
        if self.magic != [0x7f, 'E' as u8, 'L' as u8, 'F' as u8] {
            return Err(());
        }

        Ok(())
    }
}
