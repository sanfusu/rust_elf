use super::gabi41;
use super::Elf;
/// 符号表条目
#[repr(packed)]
pub struct Sym {
    /// 用来保存对象文件中符号字符串表中的索引
    pub name: <Elf as gabi41::ElfBasicType>::Word,
    /// 该成员用来保存与符号相关联的值。
    /// 既可以是绝对值，也可以是一个地址，具体是哪一个取决于上下文环境。
    pub value: <Elf as gabi41::ElfBasicType>::Addr,
    /// 符号相关联的大小。如果是 0，表示没有大小，或者大小未知。
    pub size: <Elf as gabi41::ElfBasicType>::Word,
    /// 用来表示符号类型和绑定的属性，可用值见 [`StBind`](gabi41::symtable::StBind) 和 [`StType`](gabi41::symtable::StType)
    pub info: u8,
    /// 当前值为 0，属于保留字段
    pub other: u8,
    /// 每一个符号表条目都和一些 section 相关。
    /// 该字段保存相关联的 section 在 section header table 中的索引
    pub shndx: <Elf as gabi41::ElfBasicType>::Half,
}
