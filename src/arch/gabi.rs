pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;
// use crate::AsBytes;
pub mod e_ident {
    pub mod idx {
        pub const EI_MAG0: u8 = 0;
        pub const EI_MAG1: u8 = 1;
        pub const EI_MAG2: u8 = 2;
        pub const EI_MAG3: u8 = 3;
        pub const EI_CLASS: u8 = 4;
        pub const EI_DATA: u8 = 5;
        pub const EI_VERSION: u8 = 6;
        pub const EI_NIDENT: u8 = 16;
    }
    pub mod ei_mag {
        pub const ELFMAG0: u8 = 0x7f;
        pub const ELFMAG1: u8 = 'E' as u8;
        pub const ELFMAG2: u8 = 'L' as u8;
        pub const ELFMAG3: u8 = 'F' as u8;
    }
    pub mod ei_class {
        pub const ELFCLASSNONE: u8 = 0;
        pub const ELFCLASS32: u8 = 1;
        pub const ELFCLASS64: u8 = 2;
    }
    pub mod ei_data {
        pub const ELFDATANONE: u8 = 0;
        pub const ELFDATA2LSB: u8 = 1;
        pub const ELFDATA2MSB: u8 = 2;
    }
    pub mod ei_version {
        pub const EV_NONE: u32 = 0;
        pub const EV_CURRENT: u32 = 1;
    }
}

/// You should not use these constant directly.
/// Use the arch specified constant instead.
pub mod e_type {
    #[macro_export]
    macro_rules! define_e_type_basic_const {
        ($elf:ty) => {
            pub const ET_NONE: $elf = 0;
            /// 可重定位对象文件
            pub const ET_REL: $elf = 1;
            /// 可执行文件
            pub const ET_EXEC: $elf = 2;
            /// 共享的对象文件
            pub const ET_DYN: $elf = 3;
            /// 核心文件
            pub const ET_CORE: $elf = 4;
            /// 特定处理器使用的下限
            pub const ET_LOPROC: $elf = 0xFF00;
            /// 特定处理器使用的上限
            pub const ET_HIPROC: $elf = 0xFFFF;
        };
    }
    define_e_type_basic_const!(u16);
}

pub mod e_machine {
    pub const EM_NONE: u16 = 0;
    pub const EM_M32: u16 = 1;
    pub const EM_SPARC: u16 = 2;
    pub const EM_386: u16 = 3;
    pub const EM_68K: u16 = 4;
    pub const EM_88K: u16 = 5;
    pub const EM_860K: u16 = 7;
    pub const EM_MIPS: u16 = 8;
    pub const EM_MIPS_RS4_BE: u16 = 10;
    pub const RESERVED_LO: u16 = 11;
    pub const RESERVED_HI: u16 = 16;
}
use derive::AsSlice;
#[repr(C)]
#[derive(Debug, Default, AsSlice)]
pub struct Header<T: crate::BasicType, EI: Sized> {
    pub ident: EI,
    /// 用于表示对象文件的类型，可用值 [`ETypeValue`](e_type)
    pub r#type: T::Half,
    /// 用于表示目标架构，可用值 [`EMachineValue`](e_machine)
    pub machine: T::Half,
    /// 用于表示对象文件格式的版本，当前值只能为 [`Current`](e_ident::ei_version::EV_CURRENT)
    pub version: T::Word,
    /// 包含程序入口的虚拟地址。如果没有入口点，则值为 0。
    pub entry: T::Addr,
    /// 包含 program header table 的文件偏移地址，以字节为单位。
    pub phoff: T::Off,
    /// 包含 section header table 的文件偏移地址，以字节为单位。
    pub shoff: T::Off,
    /// 包含特定处理器的 flag
    pub flags: T::Word,
    /// 包含 ELF header 的大小，以字节为单位。
    pub ehsize: T::Half,
    /// 包含 program header table 条目的大小，以字节为单位。
    pub phentsize: T::Half,
    /// 包含 program header table 中条目的数量。
    pub phnum: T::Half,
    /// 包含 section header table 条目的大小，以字节为单位。
    pub shentsize: T::Half,
    /// 包含 section header table 中条目的数量。
    pub shnum: T::Half,
    /// 包含 section 名称字符串表的 section 在 section header table 中的索引。
    /// 如果没有 section 名称字符串表，该字段的值为 `SHN_UNDEF`
    pub shstrndx: T::Half,
}

#[derive(Default)]
pub struct Elf<'a, Header: crate::AsBytes> {
    pub writer: Option<Box<(dyn std::io::Write + 'a)>>,
    pub reader: Option<Box<(dyn std::io::Read + 'a)>>,
    pub seeker: Option<Box<(dyn std::io::Read + 'a)>>,
    pub file: Option<&'a mut std::fs::File>,
    pub ehdr: Box<Header>,
}
use crate::AsBytes;

impl<'a, Header: std::default::Default + AsBytes> Elf<'a, Header> {
    pub fn new(file: &'a mut std::fs::File) -> Elf<Header> {
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
