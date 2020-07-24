use super::{
    shdr::{ShFlagsValue, ShTypeValue, ShdrGeneral},
    ElfBasicType,
};

/// 特殊的 section header
#[allow(non_camel_case_types)]
pub enum SpSecShdrGeneral<T: ElfBasicType + Sized> {
    bss(ShdrGeneral<T>),
    comment(ShdrGeneral<T>),
    data(ShdrGeneral<T>),
    data1(ShdrGeneral<T>),
    debug(ShdrGeneral<T>),
    dynaminc(ShdrGeneral<T>),
    dynstr(ShdrGeneral<T>),
    fini(ShdrGeneral<T>),
    got(ShdrGeneral<T>),
    hash(HashShdrGeneral<T>),
    init(ShdrGeneral<T>),
    initerp(ShdrGeneral<T>),
    line(ShdrGeneral<T>),
    note(ShdrGeneral<T>),
    plt(ShdrGeneral<T>),
    rel(ShdrGeneral<T>),
    rela(ShdrGeneral<T>),
    rodata(ShdrGeneral<T>),
    rodata1(ShdrGeneral<T>),
    shstrtab(ShdrGeneral<T>),
    strtab(ShdrGeneral<T>),
    symtab(ShdrGeneral<T>),
    text(ShdrGeneral<T>),
}

impl<T: ElfBasicType> HashShdrGeneral<T> {
    /// 创建一个新的 HASH section header，只对 sh_type 和 sh_flags 初始化
    pub fn new() -> Self {
        Self {
            sh_name: Default::default(),
            sh_type: ShTypeValue::HASH.ordinal().into(),
            sh_flags: ShFlagsValue::ALLOC.ordinal().into(),
            sh_addr: Default::default(),
            sh_offset: Default::default(),
            sh_size: Default::default(),
            sh_link: Default::default(),
            sh_info: Default::default(),
            sh_addralign: Default::default(),
            sh_entsize: Default::default(),
        }
    }
}

pub type HashShdrGeneral<T> = ShdrGeneral<T>;
