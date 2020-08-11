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

pub mod sh_flags {
    /// 用作 [`Shdr::flags`](super::Header::flags) 的可用值。
    /// 具有 WRITE 属性标签的 section 在进程执行时应当是可写的。   
    pub const WRITE: u32 = 0x1;
    /// 在进程执行时，具有该属性的 section 占用内存。
    /// 一些控制 section 不占有对象文件的内存映像，
    /// 因而该属性标签对于这些控制 section 处于关闭状态
    pub const ALLOC: u32 = 0x2;
    /// 具有该属性标签的 section 包含可执行的机器指令
    pub const EXECINSTR: u32 = 0x4;
    pub const MASKPROC: u32 = 0xf000_0000;
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

pub mod sh_type {
    // #[derive(Ordinalize, Debug)]
    /// 用作 [`sh_type`](super::Header) 字段的值。
    pub const NULL: u32 = 0;
    /// 包含程序定义的信息
    pub const PROGBITS: u32 = 1;
    /// 包含连接器符号表
    pub const SYMTAB: u32 = 2;
    /// 包含字符串表
    pub const STRTAB: u32 = 3;
    /// 包含 "Rela" 类型的重定位条目
    pub const RELA: u32 = 4;
    /// 包含符号 hash 表
    pub const HASH: u32 = 5;
    /// 包含动态链接表
    pub const DYNAMIC: u32 = 6;
    /// 包含 note 信息
    pub const NOTE: u32 = 7;
    /// 包含未初始化的空间，不占用文件空间
    pub const NOBITS: u32 = 8;
    /// 包含 "Rel" 类型的重定位条目
    pub const REL: u32 = 9;
    /// 保留
    pub const SHLIB: u32 = 10;
    /// 包含动态加载器符号表
    pub const DYNSYM: u32 = 11;
    /// 用于特定处理器，下限
    pub const LOPROC: u32 = 0x7000_0000;
    /// 用于特定处理器，上限
    pub const HIPROC: u32 = 0x7fff_ffff;
    /// 为应用程序保留，下限
    pub const LOUSER: u32 = 0x8000_0000;
    /// 为应用程序保留，上限
    pub const HIUSER: u32 = 0xffff_ffff;
}
