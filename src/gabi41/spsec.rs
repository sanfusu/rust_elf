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

macro_rules! define_spsecshdr {
    (:,$name:ident, $st:path ,{$($sf:path),*}) => {};
    (:new, $name:ident, $st:path ,{$($sf:path),*}) => {
        impl<T: ElfBasicType + std::default::Default> $name<T> {
            pub fn new() -> Self {
                Self(ShdrGen {
                    sh_type: $st.ordinal().into(),
                    sh_flags: ($($sf.ordinal()+)*0).into(),
                    ..Default::default()
                })
            }
        }
    };
    (:default, $name:ident)=>{
         impl<T: ElfBasicType + std::default::Default> Default for $name<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    (:doconly, $name:ident, $st:path, {$($sf:path),*} $(,$docmain:expr)?)=>{
        define_spsecshdr!(ident:$name, concat!(
                "+ sh_type:\n\n\t + ", stringify!([$st]),"\n\n",
                "+ sh_flags: \n\n"
                ,$("\t+ ", stringify!([$sf]),"\n\n"),*
            ) $(,concat!($docmain,"\n\n"))?
        );
    };
    (ident:$name:ident, $docvalue:expr $(,$docmain:expr)? )=>{
       $(#[doc=$docmain])?
        #[doc=$docvalue]
        #[derive(Debug)]
        pub struct $name<T: ElfBasicType>(ShdrGen<T>);
    };
    ($name:ident, $st:path, {$($sf:path),*}, new $(,$docmain:expr)?) => {
        define_spsecshdr!(:doconly, $name, $st ,{$($sf),*} $(,$docmain)?);
        define_spsecshdr!(:new, $name, $st ,{$($sf),*});
    };
    ($name:ident, $st:path, {$($sf:path),*} $(,$docmain:expr)?) => {
        define_spsecshdr!(:doconly, $name, $st ,{$($sf),*} $(,$docmain)?);
        define_spsecshdr!(:new, $name, $st ,{$($sf),*});
        define_spsecshdr!(:default, $name);
    };
    ($name:ident, $st:path, new $(,$docmain:expr)?) => {
        define_spsecshdr!(:doconly, $name, $st ,{} $(,$docmain)?);
        define_spsecshdr!(:new, $name, $st, {});
    };
    ($name:ident, $st:path $(,$docmain:expr)?) => {
        define_spsecshdr!(:doconly, $name, $st ,{} $(,$docmain)?);
        define_spsecshdr!(:new, $name, $st, {});
        define_spsecshdr!(:default, $name);
    };

}

define_spsecshdr!(
    BssShdrGen,
    ShTypeValue::NOBITS,
    {ShFlagsValue::ALLOC, ShFlagsValue::WRITE},
    "该 section 保存未初始化的数据，不占用文件空间"
);
define_spsecshdr!(
    CommentShdrGen,
    ShTypeValue::PROGBITS,
    {},
    "用于保存版本控制信息"
);
define_spsecshdr!(
    DataShdrGen,
    ShTypeValue::PROGBITS,
    {ShFlagsValue::ALLOC,ShFlagsValue::WRITE},
    "用于保存初始化的数据"
);
define_spsecshdr!(
    Data1ShdrGen,
    ShTypeValue::PROGBITS,
    {ShFlagsValue::ALLOC,ShFlagsValue::WRITE},
    "和 [DataShdrGen] 一样"
);
define_spsecshdr!(
    DebugShdrGen,
    ShTypeValue::PROGBITS,
    {},
    "用于保存符号化调式的相关信息"
);
define_spsecshdr!(DynamicShdrGen, ShTypeValue::DYNAMIC, new);
define_spsecshdr!(DynStrShdrGen, ShTypeValue::STRTAB, { ShFlagsValue::ALLOC });
define_spsecshdr!(DynSymShdrGen, ShTypeValue::DYNSYM, { ShFlagsValue::ALLOC });
define_spsecshdr!(FiniShdrGen, ShTypeValue::PROGBITS,{ShFlagsValue::ALLOC,ShFlagsValue::EXECINSTR});
define_spsecshdr!(GotShdrGen, ShTypeValue::PROGBITS);
define_spsecshdr!(HashShdrGen, ShTypeValue::HASH, { ShFlagsValue::ALLOC });
define_spsecshdr!(InitShdrGen, ShTypeValue::PROGBITS, { ShFlagsValue::ALLOC });
define_spsecshdr!(InterpShdrGen, ShTypeValue::PROGBITS);
define_spsecshdr!(LineShdrGen, ShTypeValue::PROGBITS);
define_spsecshdr!(NoteShdrGen, ShTypeValue::NOTE);
define_spsecshdr!(PltShdrGen, ShTypeValue::PROGBITS);
define_spsecshdr!(RelShdrGen, ShTypeValue::REL);
define_spsecshdr!(RelaShdrGen, ShTypeValue::RELA);
define_spsecshdr!(RodataShdrGen, ShTypeValue::PROGBITS, {
    ShFlagsValue::ALLOC
});
define_spsecshdr!(Rodata1ShdrGen, ShTypeValue::PROGBITS, {
    ShFlagsValue::ALLOC
});
define_spsecshdr!(ShStrTabShdrGen, ShTypeValue::STRTAB);
define_spsecshdr!(StrTabShdrGen, ShTypeValue::STRTAB);
define_spsecshdr!(SymTabShdrGen, ShTypeValue::SYMTAB);
define_spsecshdr!(TextShdrGen,ShTypeValue::PROGBITS,{ShFlagsValue::ALLOC,ShFlagsValue::EXECINSTR});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf64::Elf as Elf64;
    #[test]
    fn test() {
        let a = BssShdrGen::<Elf64>::new();
        println!("{:?}", a);
    }
}
