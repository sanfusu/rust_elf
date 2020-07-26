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
    ($name:ident, $st:path, {$($sf:path),*}, $doc:expr) => {
        #[doc=$doc]
        #[derive(Debug)]
        pub struct $name<T: ElfBasicType>(ShdrGen<T>);
        impl<T: ElfBasicType + std::default::Default> $name<T> {
            pub fn new() -> Self {
                Self(ShdrGen {
                    sh_type: $st.ordinal().into(),
                    sh_flags: ($($sf.ordinal()+)*0).into(),
                    ..Default::default()
                })
            }
        }
        impl<T: ElfBasicType + std::default::Default> Default for $name<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    ($name:ident, $st:path, {$($sf:path),*}) => {
        define_spsecshdr!($name, $st, {$($sf),*},
            concat!(
                "+ sh_type:\n\n\t + ", stringify!([$st]),"\n\n",
                "+ sh_flags:\n\n",
                $("\t+ ", stringify!(
                    [$sf]
                ),"\n\n"),*
            )
        );
    };
    ($name:ident, $st:path) => {
        define_spsecshdr!($name, $st, {},
            concat!(
                "+ sh_type:\n\n\t + ", stringify!([$st]),"\n\n",
                "+ sh_flags: unknown",
            )
        );
    };
    ($name:ident, $st:path, {$($sf:path),*}, new, $doc:expr) => {
        #[doc=$doc]
        pub struct $name<T: ElfBasicType>(ShdrGen<T>);
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
    ($name:ident, $st:path, new) => {
        define_spsecshdr!($name, $st, {},new,
            concat!(
                "+ sh_type:\n\n\t + ", stringify!([$st]),"\n\n",
                "+ sh_flags: unknown",
            )
        );
    };
    ($name:ident)=>{
        pub struct $name<T: ElfBasicType>(ShdrGen<T>);
    };
}

define_spsecshdr!(BssShdrGen, ShTypeValue::NOBITS,{ShFlagsValue::ALLOC, ShFlagsValue::WRITE});
define_spsecshdr!(CommentShdrGen, ShTypeValue::PROGBITS);
define_spsecshdr!(DataShdrGen,ShTypeValue::PROGBITS,{ShFlagsValue::ALLOC,ShFlagsValue::WRITE});
define_spsecshdr!(Data1ShdrGen,ShTypeValue::PROGBITS,{ShFlagsValue::ALLOC,ShFlagsValue::WRITE});
define_spsecshdr!(DebugShdrGen, ShTypeValue::PROGBITS);
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
