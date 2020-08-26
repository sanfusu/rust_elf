macro_rules! impl_convert_from_block_mem_for_plain_struct {
    ($struct:ty) => {
        impl std::convert::TryFrom<&[u8]> for &$struct {
            type Error = crate::Error;
            fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
                if src.len() < std::mem::size_of::<Self>() {
                    return Err(crate::Error::DataLoss);
                } else {
                    unsafe { Ok(&*(src.as_ptr() as *const $struct)) }
                }
            }
        }
        impl From<&[u8; std::mem::size_of::<$struct>()]> for &$struct {
            fn from(src: &[u8; std::mem::size_of::<$struct>()]) -> Self {
                assert_eq!(
                    src.as_ptr() as usize % std::mem::align_of::<$struct>(),
                    0,
                    "Miss aligned"
                );
                unsafe { &*(src.as_ptr() as *const $struct) }
            }
        }
        impl From<[u8; std::mem::size_of::<$struct>()]> for $struct {
            fn from(src: [u8; std::mem::size_of::<$struct>()]) -> Self {
                assert_eq!(
                    src.as_ptr() as usize % std::mem::align_of::<$struct>(),
                    0,
                    "Miss aligned"
                );
                unsafe { *(src.as_ptr() as *const $struct) }
            }
        }
        impl AsRef<[u8]> for $struct {
            fn as_ref(&self) -> &[u8] {
                unsafe {
                    std::slice::from_raw_parts(
                        self as *const $struct as *const u8,
                        std::mem::size_of::<$struct>(),
                    )
                }
            }
        }
        impl AsMut<[u8]> for $struct {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe {
                    std::slice::from_raw_parts_mut(
                        self as *mut $struct as *mut u8,
                        std::mem::size_of::<$struct>(),
                    )
                }
            }
        }
    };
}
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
macro_rules! define_e_type_basic_const {
    ($elf:ty) => {
        pub const NONE: $elf = 0;
        /// 可重定位对象文件
        pub const REL: $elf = 1;
        /// 可执行文件
        pub const EXEC: $elf = 2;
        /// 共享的对象文件
        pub const DYN: $elf = 3;
        /// 核心文件
        pub const CORE: $elf = 4;
        /// 特定处理器使用的下限
        pub const LOPROC: $elf = 0xFF00;
        /// 特定处理器使用的上限
        pub const HIPROC: $elf = 0xFFFF;
    };
}
macro_rules! define_sh_flags_basic_const {
    ($elf:ty) => {
        /// 用作 [`Shdr::flags`](super::header::Shdr::flags) 的可用值。
        /// 具有 WRITE 属性标签的 section 在进程执行时应当是可写的。   
        pub const WRITE: $elf = 0x1;
        /// 在进程执行时，具有该属性的 section 占用内存。
        /// 一些控制 section 不占有对象文件的内存映像，
        /// 因而该属性标签对于这些控制 section 处于关闭状态
        pub const ALLOC: $elf = 0x2;
        /// 具有该属性标签的 section 包含可执行的机器指令
        pub const EXECINSTR: $elf = 0x4;
        pub const MASKPROC: $elf = 0xf000_0000;
    };
}
macro_rules! define_sh_type_basic_const {
    ($elf:ty) => {
        // #[derive(Ordinalize, Debug)]
        /// 用作 [`sh_type`](crate::arch::gabi::section::header::Shdr::type) 字段的值。
        pub const NULL: $elf = 0;
        /// 包含程序定义的信息
        pub const PROGBITS: $elf = 1;
        /// 包含连接器符号表
        pub const SYMTAB: $elf = 2;
        /// 包含字符串表
        pub const STRTAB: $elf = 3;
        /// 包含 "Rela" 类型的重定位条目
        pub const RELA: $elf = 4;
        /// 包含符号 hash 表
        pub const HASH: $elf = 5;
        /// 包含动态链接表
        pub const DYNAMIC: $elf = 6;
        /// 包含 note 信息
        pub const NOTE: $elf = 7;
        /// 包含未初始化的空间，不占用文件空间
        pub const NOBITS: $elf = 8;
        /// 包含 "Rel" 类型的重定位条目
        pub const REL: $elf = 9;
        /// 保留
        pub const SHLIB: $elf = 10;
        /// 包含动态加载器符号表
        pub const DYNSYM: $elf = 11;
        /// 用于特定处理器，下限
        pub const LOPROC: $elf = 0x7000_0000;
        /// 用于特定处理器，上限
        pub const HIPROC: $elf = 0x7fff_ffff;
        /// 为应用程序保留，下限
        pub const LOUSER: $elf = 0x8000_0000;
        /// 为应用程序保留，上限
        pub const HIUSER: $elf = 0xffff_ffff;
    };
}

macro_rules! define_p_type_basic_const {
    ($elf:ty) => {
        pub const NULL: $elf = 0;
        pub const LOAD: $elf = 1;
        pub const DYNAMIC: $elf = 2;
        pub const INTERP: $elf = 3;
        pub const NOTE: $elf = 4;
        pub const SHLIB: $elf = 5;
        pub const PHDR: $elf = 6;
        pub const LOPROC: $elf = 0x7000_0000;
        pub const HIPROC: $elf = 0x7fff_ffff;
    };
}

macro_rules! define_p_flags_basic_const {
    ($elf:ty) => {
        pub const X: $elf = 0x1;
        pub const W: $elf = 0x2;
        pub const R: $elf = 0x4;
        pub const MASKPROC: $elf = 0xf000_0000;
    };
}

macro_rules! define_d_tag_basic_const {
    ($elf:ty) => {
        pub const NULL: $elf = 0;
        pub const NEEDED: $elf = 1;
        pub const PLTRELSZ: $elf = 2;
        pub const PLTGOT: $elf = 3;
        pub const HASH: $elf = 4;
        pub const STRTAB: $elf = 5;
        pub const SYMTAB: $elf = 6;
        pub const RELA: $elf = 7;
        pub const RELASZ: $elf = 8;
        pub const RELAENT: $elf = 9;
        pub const STRSZ: $elf = 10;
        pub const SYMENT: $elf = 11;
        pub const INIT: $elf = 12;
        pub const FINI: $elf = 13;
        pub const SONAME: $elf = 14;
        pub const RPATH: $elf = 15;
        pub const SYMBOLIC: $elf = 16;
        pub const REL: $elf = 17;
        pub const RELSZ: $elf = 18;
        pub const RELENT: $elf = 19;
        pub const PLTREL: $elf = 20;
        pub const DEBUG: $elf = 21;
        pub const TEXTREL: $elf = 22;
        pub const JMPREL: $elf = 23;
        pub const BIND_NOW: $elf = 24;
        pub const LOPROC: $elf = 0x7000_0000;
        pub const HIPROC: $elf = 0x7fff_ffff;
    };
}

pub mod elf32;
pub mod elf64;
pub mod gabi;
