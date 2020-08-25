pub mod header {
    use derive::AsSlice;
    #[derive(AsSlice, Default, Debug)]
    #[repr(C)]
    pub struct Shdr<T: crate::IBasicType> {
        /// section 的名字，值为字符串表 section 的索引
        pub name: T::Word,
        /// 用于对 section 的内容和语义进行分类，可用值为 [`super::TYPE`]
        pub r#type: T::Word,
        /// section 所支持的各种 bit flag 属性，可用值见 [`super::FLAGS`]
        pub flags: T::Xword,
        /// 如果 section 在出现在进程的内存映像中，
        /// 该成员会给出该 section 的首地址。
        /// 否则，该成员的值为 0。
        pub addr: T::Addr,
        /// section 在文件中相对于文件开头的偏移地址。
        /// 但如果 section 的类型为 [`super::TYPE::NOBITS`]（不占用文件空间），
        /// 则给出理论上的位置。
        pub offset: T::Off,
        /// section 在文件中所占用的大小。如果 section 类型为 [`super::TYPE::NOBITS`]
        /// 则不占用空间，但该字段可以不为 0 .
        pub size: T::Xword,
        /// 用于保存 section header table 的索引链接，
        /// 具体解释取决于 section 的类型
        ///
        /// |`super::TYPE`|`sh_link`|`sh_info`|
        /// |--|--|--|
        /// |[`super::TYPE::DYNAMIC`]|用于索引 section 条目中所使用的字符串表|0|
        /// |[`super::TYPE::HASH`]|用于索引 hash 表所应用的符号表|0|
        /// |[`super::TYPE::REL`], [`super::TYPE::RELA`]|用于索引相关联的符号表| 用于索引重定位所应用的 section |
        /// |[`super::TYPE::SYMTAB`], [`super::TYPE::DYNSYM`]|用于索引相关联的字符串表|比最后一个本地符号得符号表索引大一（绑定 `STB_LOCAL`）|
        /// |其他|[`SHN_UNDEF`](super::IDX::UNDEF)|0|
        pub link: T::Word,
        /// 用于保存额外的信息，具体解释取决于 section 类型。
        pub info: T::Word,
        /// 地址对齐限制
        pub addralign: T::Xword,
        /// 部分 section 如符号表，具有固定大小的条目，本字段给出这些条目的大小。
        /// 如果本字段值为 0，则该 section 不具有这种固定大小的条目。
        pub entsize: T::Xword,
    }
}
/// You shoud not use the const defined here directly.
/// Use the specified arch's FLAGS instead.
#[allow(non_snake_case)]
pub mod FLAGS {

    define_sh_flags_basic_const!(u32);
}

#[allow(non_snake_case)]
pub(crate) mod IDX {
    pub const UNDEF: u16 = 0;
    pub const LORESERVE: u16 = 0xff00;
    pub const LOPROC: u16 = 0xff00;
    pub const HIPROC: u16 = 0xff1f;
    pub const ABS: u16 = 0xfff1;
    pub const COMMON: u16 = 0xfff2;
    pub const HIRESERVE: u16 = 0xffff;
}

/// You shoud not use the const defined here directly.
/// Use the specified arch's TYPE instead.
#[allow(non_snake_case)]
pub mod TYPE {
    define_sh_type_basic_const!(u32);
}

#[cfg(test)]
mod test {
    use crate::{arch::elf32::section::header::Shdr as Shdr32, AsBytes};
    #[test]
    fn as_slices_test() {
        let mut var = Shdr32 {
            ..Default::default()
        };
        let b = var.as_mut();
        b[1] = 1u8;
        assert_eq!(b.len(), std::mem::size_of_val(b));
        println!("{:?}", var.as_bytes_mut());
        println!("{:?}", var.as_bytes_mut().len());
    }
}
