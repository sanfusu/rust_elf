#[derive(Ordinalize, Debug)]
#[repr(u32)]
#[non_exhaustive]
/// 用作 [`ShdrGeneral::sh_type`] 字段的值。
pub enum ShTypeValue {
    NULL = 0,
    /// 包含程序定义的信息
    PROGBITS = 1,
    /// 包含连接器符号表
    SYMTAB,
    /// 包含字符串表
    STRTAB = 3,
    /// 包含 "Rela" 类型的重定位条目
    RELA,
    /// 包含符号 hash 表
    HASH = 5,
    /// 包含动态链接表
    DYNAMIC,
    /// 包含 note 信息
    NOTE = 7,
    /// 包含未初始化的空间，不占用文件空间
    NOBITS,
    /// 包含 "Rel" 类型的重定位条目
    REL = 9,
    /// 保留
    SHLIB,
    /// 包含动态加载器符号表
    DYNSYM = 11,
    /// elf-64 特有（特定环境使用）
    LOOS = 0x6000_0000,
    /// elf-64 特有（特定环境使用）
    HIOS = 0x6fff_ffff,
    /// 用于特定处理器，下限
    LOPROC = 0x7000_0000,
    /// 用于特定处理器，上限
    HIPROC = 0x7fff_ffff,
    /// 为应用程序保留，下限
    LOUSER = 0x8000_0000,
    /// 为应用程序保留，上限
    HIUSER = 0xffff_ffff,
}
#[derive(Ordinalize, Debug)]
#[repr(u16)]
#[non_exhaustive]
pub enum SpSecIndices {
    UNDEF = 0,
    LOPROC = 0xFF00,
    HIPROC = 0xFF1F,
    LOOS = 0xFF20,
    HIOS = 0xFF3F,
    ABS = 0xFFF1,
    COMMON = 0xFFF2,
}

#[derive(Ordinalize, Debug)]
#[repr(u32)]
#[non_exhaustive]
/// 用作 [`ShdrGeneral::sh_flags`] 的可用值。
pub enum ShFlagsValue {
    /// 具有 WRITE 属性标签的 section 在进程执行时应当是可写的。   
    WRITE = 0x1,
    /// 在进程执行时，具有该属性的 section 占用内存。
    /// 一些控制 section 不占有对象文件的内存映像，
    /// 因而该属性标签对于这些控制 section 处于关闭状态
    ALLOC = 0x2,
    /// 具有该属性标签的 section 包含可执行的机器指令
    EXECINSTR = 0x4,
    /// 用于特定环境，Elf64 中使用
    MASKOS = 0x0f00_0000,
    /// 该掩码中的所有 bit 位，都是为特定处理器语义而保留。
    MASKPROC = 0xf000_0000,
}

use super::ElfBasicType;
pub struct ShdrGeneral<T: ElfBasicType> {
    /// section 的名字，值为字符串表 section 的索引
    pub sh_name: T::Word,
    /// 用于对 section 的内容和语义进行分类，可用值为 [`ShTypeValue`]
    pub sh_type: T::Word,
    /// section 所支持的各种 bit flag 属性，可用值见 [`ShFlagsValue`]
    pub sh_flags: T::Xword,
    /// 如果 section 在出现在进程的内存映像中，
    /// 该成员会给出该 section 的首地址。
    /// 否则，该成员的值为 0。
    pub sh_addr: T::Addr,
    /// section 在文件中相对于文件开头的偏移地址。
    /// 但如果 section 的类型为 [`ShTypeValue::NOBITS`]（不占用文件空间），
    /// 则给出理论上的位置。
    pub sh_offset: T::Off,
    /// section 在文件中所占用的大小。如果 section 类型为 [`ShTypeValue::NOBITS`]
    /// 则不占用空间，但该字段可以不为 0 .
    pub sh_size: T::Xword,
    /// 用于保存 section header table 的索引链接，
    /// 具体解释取决于 section 的类型
    ///
    /// |`sh_type`|`sh_link`|`sh_info`|
    /// |--|--|--|
    /// |[`ShTypeValue::DYNAMIC`]|用于索引 section 条目中所使用的字符串表|0|
    /// |[`ShTypeValue::HASH`]|用于索引 hash 表所应用的符号表|0|
    /// |[`ShTypeValue::REL`], [`ShTypeValue::RELA`]|用于索引相关联的符号表| 用于索引重定位所应用的 section |
    /// |[`ShTypeValue::SYMTAB`], [`ShTypeValue::DYNSYM`]|用于索引相关联的字符串表|比最后一个本地符号得符号表索引大一（绑定 `STB_LOCAL`）|
    /// |其他|[`SpSecIndices::UNDEF`]|0|
    pub sh_link: T::Word,
    /// 用于保存额外的信息，具体解释取决于 section 类型。
    pub sh_info: T::Word,
    /// 地址对齐限制
    pub sh_addralign: T::Xword,
    /// 部分 section 如符号表，具有固定大小的条目，本字段给出这些条目的大小。
    /// 如果本字段值为 0，则该 section 不具有这种固定大小的条目。
    pub sh_entsize: T::Xword,
}
