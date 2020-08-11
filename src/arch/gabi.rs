pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;
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
pub mod e_type {
    pub const ET_NONE: u16 = 0;
    /// 可重定位对象文件
    pub const ET_REL: u16 = 1;
    /// 可执行文件
    pub const ET_EXEC: u16 = 2;
    /// 共享的对象文件
    pub const ET_DYN: u16 = 3;
    /// 核心文件
    pub const ET_CORE: u16 = 4;
    /// 特定处理器使用的下限
    pub const ET_LOPROC: u16 = 0xFF00;
    /// 特定处理器使用的上限
    pub const ET_HIPROC: u16 = 0xFFFF;
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
/// 通过预读 PreEIdent 来识别文件类型，以便进一步操作。
#[repr(C)]
#[derive(Debug)]
pub struct Ident {
    /// 值只能为 `{0x7f, 'E', 'L', 'F'}`。用来识别 Elf 对象文件
    pub magic: [u8; 4],
    /// 用来识别文件类别，可用值为 [`ei_class`](e_ident::ei_class)。
    ///
    ///  架构的[基本类型](crate::BasicType)大小不同。
    pub class: u8,
    /// 用来指定特定处理器在对象文件的编码，可用值为 [`ei_data`](e_ident::ei_data)
    ///
    /// 小端编码和大端编码，一般默认小端编码。
    pub data: u8,
    /// 用来表示 ELF 头部版本号，目前值必须是 [`EV_CURRENT`](e_ident::ei_version::EV_CURRENT)
    pub version: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct Header<T: crate::BasicType, EI> {
    pub e_ident: EI,
    /// 用于表示对象文件的类型，可用值 [`ETypeValue`](e_type)
    pub e_type: T::Half,
    /// 用于表示目标架构，可用值 [`EMachineValue`](e_machine)
    pub e_machine: T::Half,
    /// 用于表示对象文件格式的版本，当前值只能为 [`Current`](e_ident::ei_version::EV_CURRENT)
    pub e_version: T::Word,
    /// 包含程序入口的虚拟地址。如果没有入口点，则值为 0。
    pub e_entry: T::Addr,
    /// 包含 program header table 的文件偏移地址，以字节为单位。
    pub e_phoff: T::Off,
    /// 包含 section header table 的文件偏移地址，以字节为单位。
    pub e_shoff: T::Off,
    /// 包含特定处理器的 flag
    pub e_flags: T::Word,
    /// 包含 ELF header 的大小，以字节为单位。
    pub e_ehsize: T::Half,
    /// 包含 program header table 条目的大小，以字节为单位。
    pub e_phentsize: T::Half,
    /// 包含 program header table 中条目的数量。
    pub e_phnum: T::Half,
    /// 包含 section header table 条目的大小，以字节为单位。
    pub e_shentsize: T::Half,
    /// 包含 section header table 中条目的数量。
    pub e_shnum: T::Half,
    /// 包含 section 名称字符串表的 section 在 section header table 中的索引。
    /// 如果没有 section 名称字符串表，该字段的值为 `SHN_UNDEF`
    pub e_shstrndx: T::Half,
}
