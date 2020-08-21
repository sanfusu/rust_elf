pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

use header::IEhdr;

// use crate::AsBytes;
pub(crate) mod e_ident {
    pub(crate) mod idx {
        pub const EI_MAG0: usize = 0;
        pub const EI_MAG1: usize = 1;
        pub const EI_MAG2: usize = 2;
        pub const EI_MAG3: usize = 3;
        pub const EI_CLASS: usize = 4;
        pub const EI_DATA: usize = 5;
        pub const EI_VERSION: usize = 6;
        pub const EI_NIDENT: usize = 16;
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
        pub const EV_NONE: u8 = 0;
        pub const EV_CURRENT: u8 = 1;
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
    // 只有那些同时具有 32 和 64 模式的 machine 可以在这里定义
    #[macro_export]
    macro_rules! define_e_machine_basic_constant {
        ($elf:ty) => {
            pub const EM_NONE: $elf = 0;
            pub const EM_M32: $elf = 1;
            pub const EM_SPARC: $elf = 2;
            pub const EM_386: $elf = 3;
            pub const EM_68K: $elf = 4;
            pub const EM_88K: $elf = 5;
            pub const EM_860K: $elf = 7;
            pub const EM_MIPS: $elf = 8;
            pub const EM_MIPS_RS4_BE: $elf = 10;
        };
    }
    define_e_machine_basic_constant!(u16);
}

pub mod header {
    use derive::AsSlice;
    pub trait IEhdr<T: crate::IBasicType> {
        fn get_phoff(&self) -> T::Off;
        fn get_shoff(&self) -> T::Off;
        fn get_shdr_size(&self) -> T::Half;
        fn get_shdr_num(&self) -> T::Half;
    }

    #[repr(C)]
    #[derive(Debug, Default, AsSlice)]
    pub struct Ehdr<T: crate::IBasicType, EI: Sized> {
        pub ident: EI,
        /// 用于表示对象文件的类型，可用值 [`ETypeValue`](super::e_type)
        pub r#type: T::Half,
        /// 用于表示目标架构，可用值 [`EMachineValue`](super::e_machine)
        pub machine: T::Half,
        /// 用于表示对象文件格式的版本，当前值只能为 [`Current`](super::e_ident::ei_version::EV_CURRENT)
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
    impl<T: crate::IBasicType, EI> IEhdr<T> for Ehdr<T, EI> {
        fn get_phoff(&self) -> T::Off {
            self.phoff
        }
        fn get_shoff(&self) -> T::Off {
            self.shoff
        }
        fn get_shdr_size(&self) -> T::Half {
            self.shentsize * self.shnum
        }
        fn get_shdr_num(&self) -> T::Half {
            self.shnum
        }
    }
}

use crate::AsBytes;
use std::{
    default::Default,
    fs,
    io::{self, Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct Elf<'a, T, Ehdr, Shdr>
where
    T: crate::IBasicType,
    Ehdr: crate::AsBytes + IEhdr<T> + Default,
    Shdr: crate::AsBytes + Default,
{
    file: &'a mut fs::File,
    ehdr: Option<Box<Ehdr>>,
    shdr_table: Option<Box<Vec<Shdr>>>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, Ehdr, Shdr, T> Elf<'a, T, Ehdr, Shdr>
where
    T: crate::IBasicType,
    Ehdr: Default + AsBytes + crate::Validity + IEhdr<T>,
    Shdr: crate::AsBytes + Default,
{
    /// crate 内部使用，避免过度的有效性检查
    pub(crate) fn new_without_validity_check(file: &'a mut fs::File) -> Elf<T, Ehdr, Shdr> {
        Elf {
            file,
            ehdr: Default::default(),
            shdr_table: Default::default(),
            phantom: Default::default(),
        }
    }
    pub fn new(file: &'a mut fs::File) -> io::Result<Elf<'a, T, Ehdr, Shdr>> {
        let mut ret: Elf<T, Ehdr, Shdr> = Elf::new_without_validity_check(file);
        let var_name = ret.read_ehdr()?;
        let x: &[u8] = var_name.as_bytes();
        crate::is_elf(x)?;
        ret.ehdr.as_ref().unwrap().is_valid().map(|_| ret)
    }

    pub fn read_ehdr(&mut self) -> io::Result<&Box<Ehdr>> {
        match self.ehdr {
            Some(ref v) => Ok(v),
            None => {
                self.ehdr = Some(Default::default());
                self.file.seek(SeekFrom::Start(0))?;
                let len = self.file.read(self.ehdr.as_mut().unwrap().as_bytes_mut())?;
                if len < std::mem::size_of_val(&self.ehdr) {
                    Err(crate::Error::DataLoss.into())
                } else {
                    Ok(self.ehdr.as_ref().unwrap())
                }
            }
        }
    }

    pub fn read_shdr_table(&mut self) -> io::Result<&Box<Vec<Shdr>>> {
        match self.shdr_table {
            Some(ref v) => Ok(v),
            None => match self.ehdr {
                None => Err(crate::Error::InvalidEhdr.into()),
                Some(ref v) => {
                    self.shdr_table = Some(Default::default());
                    self.file.seek(SeekFrom::Start(v.get_shoff().into()))?;
                    for _ in 0..v.get_shdr_num().into() {
                        let mut s: Shdr = Default::default();
                        self.file.read(s.as_bytes_mut())?;
                        self.shdr_table.as_mut().unwrap().push(s);
                    }
                    Ok(self.shdr_table.as_ref().unwrap())
                }
            },
        }
    }
}
