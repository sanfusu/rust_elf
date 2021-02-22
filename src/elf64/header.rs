use super::{Addr, Half, Off, Word};

define_transparent_meta_data! {
    pub ObjectFileClass, u8 {
        Wellknown: {
            pub ELF_CLASS32 = 1
            pub ELF_CLASS64 = 2
        }
        ValidRange: {
            ELF_CLASS_RANGE = ELF_CLASS32 ..= ELF_CLASS64
        }
        Default: ELF_CLASS64
    }
}

define_transparent_meta_data! {
    pub DataEncoding, u8 {
        Wellknown: {
            pub ELF_LSB = 1
            pub ELF_MSB = 2
        }
        ValidRange:{
            ELF_DATA_ENCODING_RANGE = ELF_LSB..=ELF_MSB
        }
        Default: ELF_LSB
    }
}

#[derive(MetaData, Default)]
pub struct Ident {
    pub mag: [u8; 4],
    pub file_class: ObjectFileClass,
    pub data_encode: DataEncoding,
    pub file_version: u8,
    pub osabi: u8,
    pub abi_version: u8,
}
#[derive(MetaData, Ehdr)]
#[repr(packed)]
pub struct Ehdr {
    pub e_ident: Ident,
    _pad: [u8; 16 - core::mem::size_of::<Ident>()],

    pub e_type: Half,
    pub e_machine: Half,
    pub e_version: Word,
    pub e_entry: Addr,
    #[phoff]
    pub e_phoff: Off,
    #[shoff]
    pub e_shoff: Off,
    pub e_flags: Word,
    pub e_ehsize: Half,
    #[phentsize]
    pub e_phentsize: Half,
    #[phnum]
    pub e_phnum: Half,
    #[shentsize]
    pub e_shentsize: Half,
    #[shnum]
    pub e_shnum: Half,
    pub e_shstrndx: Half,
}
