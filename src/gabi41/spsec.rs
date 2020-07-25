use super::{
    shdr::{ShFlagsValue, ShTypeValue, ShdrGen},
    ElfBasicType,
};

/// 特殊的 section header
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum SpSecShdrGen<T: ElfBasicType> {
    bss(BssShdrGen<T>),
    comment(CommentShdrGen<T>),
    data(DataShdrGen<T>),
    data1(Data1ShdrGen<T>),
    debug(DebugShdrGen<T>),
    dynaminc(DynamicShdrGen<T>),
    dynstr(ShdrGen<T>),
    fini(ShdrGen<T>),
    got(ShdrGen<T>),
    hash(HashShdrGen<T>),
    init(ShdrGen<T>),
    initerp(ShdrGen<T>),
    line(ShdrGen<T>),
    note(ShdrGen<T>),
    plt(ShdrGen<T>),
    rel(ShdrGen<T>),
    rela(ShdrGen<T>),
    rodata(ShdrGen<T>),
    rodata1(ShdrGen<T>),
    shstrtab(ShdrGen<T>),
    strtab(ShdrGen<T>),
    symtab(ShdrGen<T>),
    text(ShdrGen<T>),
}

pub struct BssShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> BssShdrGen<T> {
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::NOBITS.ordinal().into(),
            sh_flags: (ShFlagsValue::ALLOC.ordinal() + ShFlagsValue::WRITE.ordinal()).into(),
            ..Default::default()
        })
    }
}
impl<T: ElfBasicType + std::default::Default> Default for BssShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CommentShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> CommentShdrGen<T> {
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::PROGBITS.ordinal().into(),
            ..Default::default()
        })
    }
}
impl<T: ElfBasicType + std::default::Default> Default for CommentShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DataShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> DataShdrGen<T> {
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::PROGBITS.ordinal().into(),
            sh_flags: (ShFlagsValue::ALLOC.ordinal() + ShFlagsValue::WRITE.ordinal()).into(),
            ..Default::default()
        })
    }
}
impl<T: ElfBasicType + std::default::Default> Default for DataShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Data1ShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> Data1ShdrGen<T> {
    pub fn new() -> Self {
        Self(DataShdrGen::new().0)
    }
}
impl<T: ElfBasicType + std::default::Default> Default for Data1ShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DebugShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> DebugShdrGen<T> {
    pub fn new() -> Self {
        Self(CommentShdrGen::new().0)
    }
}
impl<T: ElfBasicType + std::default::Default> Default for DebugShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

// NOTE: DynamicShdrGen 中的 sh_flags 值和处理器相关，这里不给予实现
pub struct DynamicShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> DynamicShdrGen<T> {
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::DYNAMIC.ordinal().into(),
            ..Default::default()
        })
    }
}

pub struct HashShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> HashShdrGen<T> {
    /// 创建一个新的 HASH section header，只对 sh_type 和 sh_flags 初始化
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::HASH.ordinal().into(),
            sh_flags: ShFlagsValue::ALLOC.ordinal().into(),
            ..Default::default()
        })
    }
}

impl<T: ElfBasicType + std::default::Default> Default for HashShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TextShdrGen<T: ElfBasicType>(ShdrGen<T>);
impl<T: ElfBasicType + std::default::Default> TextShdrGen<T> {
    /// 创建一个新的 HASH section header，只对 sh_type 和 sh_flags 初始化
    pub fn new() -> Self {
        Self(ShdrGen {
            sh_type: ShTypeValue::PROGBITS.ordinal().into(),
            sh_flags: (ShFlagsValue::ALLOC.ordinal() + ShFlagsValue::EXECINSTR.ordinal()).into(),
            ..Default::default()
        })
    }
}

impl<T: ElfBasicType + std::default::Default> Default for TextShdrGen<T> {
    fn default() -> Self {
        Self::new()
    }
}
