use super::ElfBasicType;

pub const EI_NIDENT: usize = 16;

#[deprecated = "use the struct field instead"]
#[non_exhaustive]
pub enum Elf64IdentIdx {
    MAG0 = 0,
    MAG1,
    MAG2,
    MAG3,
    CLASS = 4,
    DATA = 5,
    VERSION = 6,
    OSABI = 7,
    ABIVERSION = 8,
    PAD = 9,
    NIDENT = 16,
}
#[deprecated = "use the struct field instead"]
#[non_exhaustive]
pub enum Elf32IdentIdx {
    MAG0 = 0,
    MAG1,
    MAG2,
    MAG3,
    CLASS = 4,
    DATA = 5,
    VERSION = 6,
    PAD = 7,
    NIDENT = 16,
}

#[derive(Ordinalize, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// 用于为 [`PreEIdent::class`] 指定值
pub enum EIClassValue {
    /// Elf32
    Class32 = 1,
    // Elf64
    Class64 = 2,
}

#[derive(Ordinalize, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// 用于为 [`PreEIdent::data`] 指定值
pub enum EIDataValue {
    /// 小端法
    Lsb = 1,
    /// 大端法
    Msb = 2,
}

#[derive(Ordinalize, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// 用于为 [`PreEIdent::version`] 指定值
pub enum EIVersionValue {
    Current = 1,
}

#[derive(Ordinalize)]
#[repr(C)]
#[non_exhaustive]
pub enum ETypeValue {
    NONE = 0,
    /// 可重定位对象文件
    REL,
    /// 可执行文件
    EXEC,
    /// 共享的对象文件
    DYN,
    /// 核心文件
    CORE,
    /// 特定环境使用的下限
    LOOS = 0xFE00,
    /// 特定环境使用的上限
    HIOS = 0xFEFF,
    /// 特定处理器使用的下限
    LOPROC = 0xFF00,
    /// 特定处理器使用的上限
    HIPROC = 0xFFFF,
}
#[derive(Ordinalize, Debug)]
#[repr(u8)]
#[non_exhaustive]
pub enum EMachineValue {
    EmNone = 0,
    EmM32 = 1,
    EmSparc = 2,
    Em386 = 3,
    Em68k = 4,
    Em88k = 5,
    Em860 = 7,
    EmMips = 8,
    EmMipsRs4Be = 10,
}

/// 通过预读 PreEIdent 来识别文件类型，以便进一步操作。
#[repr(C)]
pub struct PreEIdent {
    /// 值只能为 `{0x7f, 'E', 'L', 'F'}`。用来识别 Elf 对象文件
    pub magic: [u8; 4],
    /// 用来识别文件类别，可用值为 [`EIClassValue`]。
    ///
    /// [`super::ElfBasicType`] 架构的基本类型大小不同。
    pub class: u8,
    /// 用来指定特定处理器在对象文件的编码，可用值为 [`EIDataValue`]
    ///
    /// 小端编码和大端编码，一般默认小端编码。
    pub data: u8,
    /// 用来表示 ELF 头部版本号，目前值必须是 [`EIVersionValue::Current`]
    pub version: u8,
}

impl Default for PreEIdent {
    fn default() -> Self {
        PreEIdent {
            magic: [0x7f, 'E' as u8, 'L' as u8, 'F' as u8],
            class: EIClassValue::Class64 as u8,
            data: EIDataValue::Lsb as u8,
            version: EIVersionValue::Current as u8,
        }
    }
}

#[repr(C)]
pub struct EhdrGeneral<T: ElfBasicType, EI> {
    pub e_ident: EI,
    /// 用于表示对象文件的类型，可用值 [`ETypeValue`]
    pub e_type: T::Half,
    /// 用于表示目标架构，可用值 [`EMachineValue`]
    pub e_machine: T::Half,
    /// 用于表示对象文件格式的版本，当前值只能为 [`EIVersionValue::Current`]
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

#[cfg(test)]
mod tests {
    use super::*;
    // use std::convert::TryFrom;
    // use enum_ordinalize:
    #[test]
    fn try_from_test() {
        let a: u8 = 1;
        let a = EIDataValue::from_ordinal(a).unwrap();
        println!("{:?}", a)
    }
}
