#[repr(C)]
#[derive(Default, Debug)]
pub struct Header<T: crate::BasicType> {
    /// section 的名字，值为字符串表 section 的索引
    pub name: T::Word,
    /// 用于对 section 的内容和语义进行分类，可用值为 [`sh_type`]
    pub r#type: T::Word,
    /// section 所支持的各种 bit flag 属性，可用值见 [`sh_flags`]
    pub flags: T::Xword,
    /// 如果 section 在出现在进程的内存映像中，
    /// 该成员会给出该 section 的首地址。
    /// 否则，该成员的值为 0。
    pub addr: T::Addr,
    /// section 在文件中相对于文件开头的偏移地址。
    /// 但如果 section 的类型为 [`sh_type::NOBITS`]（不占用文件空间），
    /// 则给出理论上的位置。
    pub offset: T::Off,
    /// section 在文件中所占用的大小。如果 section 类型为 [`sh_type::NOBITS`]
    /// 则不占用空间，但该字段可以不为 0 .
    pub size: T::Xword,
    /// 用于保存 section header table 的索引链接，
    /// 具体解释取决于 section 的类型
    ///
    /// |`sh_type`|`sh_link`|`sh_info`|
    /// |--|--|--|
    /// |[`sh_type::DYNAMIC`]|用于索引 section 条目中所使用的字符串表|0|
    /// |[`sh_type::HASH`]|用于索引 hash 表所应用的符号表|0|
    /// |[`sh_type::REL`], [`sh_type::RELA`]|用于索引相关联的符号表| 用于索引重定位所应用的 section |
    /// |[`sh_type::SYMTAB`], [`sh_type::DYNSYM`]|用于索引相关联的字符串表|比最后一个本地符号得符号表索引大一（绑定 `STB_LOCAL`）|
    /// |其他|[`SHN_UNDEF`](sh_idx::UNDEF)|0|
    pub link: T::Word,
    /// 用于保存额外的信息，具体解释取决于 section 类型。
    pub info: T::Word,
    /// 地址对齐限制
    pub addralign: T::Xword,
    /// 部分 section 如符号表，具有固定大小的条目，本字段给出这些条目的大小。
    /// 如果本字段值为 0，则该 section 不具有这种固定大小的条目。
    pub entsize: T::Xword,
}

/// You shoud not use the const defined here directly.
/// Use the specified arch's sh_flags instead.
pub mod sh_flags {
    #[macro_export]
    macro_rules! define_sh_flags_basic_const {
        ($elf:ty) => {
            /// 用作 [`Shdr::flags`](super::Header::flags) 的可用值。
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
    define_sh_flags_basic_const!(u32);
}

pub mod sh_idx {
    pub const UNDEF: u16 = 0;
    pub const LORESERVE: u16 = 0xff00;
    pub const LOPROC: u16 = 0xff00;
    pub const HIPROC: u16 = 0xff1f;
    pub const ABS: u16 = 0xfff1;
    pub const COMMON: u16 = 0xfff2;
    pub const HIRESERVE: u16 = 0xffff;
}

/// You shoud not use the const defined here directly.
/// Use the specified arch's sh_type instead.
pub mod sh_type {
    #[macro_export]
    macro_rules! define_sh_type_basic_const {
        ($elf:ty) => {
            // #[derive(Ordinalize, Debug)]
            /// 用作 [`sh_type`](super::Header) 字段的值。
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
    define_sh_type_basic_const!(u32);
}
