#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate elf_proc;
use elf32::ehdr::ident::encode::Encode;
pub mod elf32 {
    pub mod basic_type {
        pub type Addr = u32;
        pub type Off = u32;
        pub type Half = u16;
        pub type Word = u32;
        pub type Sword = u32;
    }
    pub mod chunk {
        use std::ops::{Deref, DerefMut};
        /// 用来表示 section 中的数据块，一个 section 可能会有多个数据块，
        /// 每一个数据块的大小由 [`Shdr::sh_entsize`](crate::elf32::section::Shdr::sh_entsize) 决定
        ///
        /// 对 Vec<u8> 类型的简单封装，实现了 Deref。
        pub struct DataChunk {
            data: Vec<u8>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for DataChunk {
            #[inline]
            fn default() -> DataChunk {
                DataChunk {
                    data: ::core::default::Default::default(),
                }
            }
        }
        impl DataChunk {
            pub fn new(data: Vec<u8>) -> Self {
                Self { data }
            }
        }
        impl Deref for DataChunk {
            type Target = Vec<u8>;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
        impl DerefMut for DataChunk {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }
    }
    pub mod ehdr {
        pub mod e_type {
            use std::ops::{Range, RangeInclusive};
            pub enum Type {
                None,
                Rel,
                Exec,
                Dyn,
                Core,
                Processor(u16),
                Unknown(u16),
            }
            impl Into<u16> for Type {
                fn into(self) -> u16 {
                    match self {
                        Type::None => NONE,
                        Type::Rel => REL,
                        Type::Exec => EXEC,
                        Type::Dyn => DYN,
                        Type::Core => CORE,
                        Type::Processor(v) if PROCRANGE.contains(&v) => v,
                        Type::Processor(_) => ::std::rt::begin_panic(
                            "You should not construct a out range processor specified Elf type",
                        ),
                        Type::Unknown(v) if UNKNRANGE.contains(&v) => v,
                        Type::Unknown(_) => ::std::rt::begin_panic(
                            "The unknown specified Elf type is not in the range",
                        ),
                    }
                }
            }
            impl From<u16> for Type {
                fn from(v: u16) -> Self {
                    match v {
                        NONE => Type::None,
                        REL => Type::Rel,
                        EXEC => Type::Exec,
                        DYN => Type::Dyn,
                        CORE => Type::Core,
                        LOPROC..=HIPROC => Type::Processor(v),
                        _ => Type::Unknown(v),
                    }
                }
            }
            const NONE: u16 = 0;
            const REL: u16 = 1;
            const EXEC: u16 = 2;
            const DYN: u16 = 3;
            const CORE: u16 = 4;
            const LOPROC: u16 = 0xff00;
            const HIPROC: u16 = 0xffff;
            const PROCRANGE: RangeInclusive<u16> = LOPROC..=HIPROC;
            const UNKNRANGE: Range<u16> = CORE..LOPROC;
        }
        pub mod ident {
            pub mod class {
                pub enum Class {
                    Class32,
                    Class64,
                    Invalid,
                }
                impl std::convert::From<u8> for Class {
                    fn from(val: u8) -> Self {
                        match val {
                            CLASS32 => Class::Class32,
                            CLASS64 => Class::Class64,
                            _ => Class::Invalid,
                        }
                    }
                }
                impl std::convert::Into<u8> for Class {
                    fn into(self) -> u8 {
                        match self {
                            Class::Class32 => CLASS32,
                            Class::Class64 => CLASS64,
                            Class::Invalid => ::std::rt::begin_panic(
                                "It's an invalid Class, you shouldn't construct an invalid Class",
                            ),
                        }
                    }
                }
                const CLASS32: u8 = 1;
                const CLASS64: u8 = 2;
            }
            pub mod encode {
                pub enum Encode {
                    Lsb,
                    Msb,
                    Invalid,
                }
                impl std::convert::From<u8> for Encode {
                    fn from(val: u8) -> Self {
                        match val {
                            LSB => Encode::Lsb,
                            MSB => Encode::Msb,
                            _ => Encode::Invalid,
                        }
                    }
                }
                impl std::convert::Into<u8> for Encode {
                    fn into(self) -> u8 {
                        match self {
                            Encode::Lsb => LSB,
                            Encode::Msb => MSB,
                            Encode::Invalid => ::std::rt::begin_panic(
                                "It's an invalid Encode, you shouldn't construct an invalid encode",
                            ),
                        }
                    }
                }
                const LSB: u8 = 1;
                const MSB: u8 = 2;
            }
            pub mod machine {
                pub enum Machine {
                    None,
                    M32,
                    Sparc,
                    I386,
                    M68k,
                    M88k,
                    I860,
                    Mips,
                    MipsRs4Be,
                    Unknown(u8),
                }
                impl std::convert::Into<u8> for Machine {
                    fn into(self) -> u8 {
                        match self {
                            Machine::None => ET_NONE,
                            Machine::M32 => EM_M32,
                            Machine::Sparc => EM_SPARC,
                            Machine::I386 => EM_386,
                            Machine::M68k => EM_68K,
                            Machine::M88k => EM_88K,
                            Machine::I860 => EM_860,
                            Machine::Mips => EM_MIPS,
                            Machine::MipsRs4Be => EM_MIPS_RS4_BE,
                            Machine::Unknown(v) => v,
                        }
                    }
                }
                impl std::convert::From<u8> for Machine {
                    fn from(v: u8) -> Self {
                        match v {
                            ET_NONE => Machine::None,
                            EM_M32 => Machine::M32,
                            EM_SPARC => Machine::Sparc,
                            EM_386 => Machine::I386,
                            EM_68K => Machine::M68k,
                            EM_88K => Machine::M88k,
                            EM_860 => Machine::I860,
                            EM_MIPS => Machine::Mips,
                            EM_MIPS_RS4_BE => Machine::MipsRs4Be,
                            _ => Machine::Unknown(v),
                        }
                    }
                }
                const ET_NONE: u8 = 0;
                const EM_M32: u8 = 1;
                const EM_SPARC: u8 = 2;
                const EM_386: u8 = 3;
                const EM_68K: u8 = 4;
                const EM_88K: u8 = 5;
                const EM_860: u8 = 7;
                const EM_MIPS: u8 = 8;
                const EM_MIPS_RS4_BE: u8 = 10;
            }
            pub mod version {
                pub enum Version {
                    Current,
                    Invalid,
                }
                impl std::convert::Into<u32> for Version {
                    fn into(self) -> u32 {
                        match self {
                            Version::Current => 1,
                            Version::Invalid => {
                                ::std::rt::begin_panic("You should not construct a invalid version")
                            }
                        }
                    }
                }
                impl std::convert::From<u32> for Version {
                    fn from(val: u32) -> Self {
                        match val {
                            CURRENT => Version::Current,
                            _ => Version::Invalid,
                        }
                    }
                }
                const CURRENT: u32 = 1;
            }
            use self::{class::Class, encode::Encode, version::Version};
            pub(crate) const MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];
            pub(crate) const CLASS_IDX: usize = 4;
            pub(crate) const DATA_IDX: usize = 5;
            pub(crate) const VERSION_IDX: usize = 6;
            pub struct Wrapper<'a> {
                pub(crate) id: &'a [u8; 16],
            }
            impl Wrapper<'_> {
                pub fn version(&self) -> Version {
                    (self.id[VERSION_IDX] as u32).into()
                }
                /// Class 只能是 [`Class::Class32`]，所以不提供写入访问
                pub fn class(&self) -> Class {
                    self.id[CLASS_IDX].into()
                }
                pub fn encode(&self) -> Encode {
                    self.id[DATA_IDX].into()
                }
            }
            pub struct WrapperMut<'a> {
                pub(crate) id: &'a mut [u8; 16],
            }
            impl WrapperMut<'_> {
                pub fn encode(&mut self, ec: Encode) {
                    self.id[DATA_IDX] = ec.into();
                }
            }
            #[repr(packed)]
            pub struct Ident {
                src: [u8; 16],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for Ident {
                #[inline]
                fn default() -> Ident {
                    Ident {
                        src: ::core::default::Default::default(),
                    }
                }
            }
            impl crate::interface::MetaData<[u8; std::mem::size_of::<Ident>()]> for Ident {
                fn to_le(self) -> Self {
                    Self { src: self.src }
                }
                fn to_be(self) -> Self {
                    Self { src: self.src }
                }
                fn from_le(value: Self) -> Self {
                    Self { src: value.src }
                }
                fn from_be(value: Self) -> Self {
                    Self { src: value.src }
                }
                fn to_be_bytes(self) -> [u8; std::mem::size_of::<Ident>()] {
                    let tmp = self.to_be();
                    let mut ret: [u8; std::mem::size_of::<Ident>()] =
                        [0; std::mem::size_of::<Ident>()];
                    (&mut ret[..]).copy_from_slice(tmp.as_slice());
                    ret
                }
                fn to_le_bytes(self) -> [u8; std::mem::size_of::<Ident>()] {
                    let tmp = self.to_le();
                    let mut ret: [u8; std::mem::size_of::<Ident>()] =
                        [0; std::mem::size_of::<Ident>()];
                    (&mut ret[..]).copy_from_slice(tmp.as_slice());
                    ret
                }
                fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Ident>()] {
                    let mut ret: [u8; std::mem::size_of::<Ident>()] =
                        [0; std::mem::size_of::<Ident>()];
                    (&mut ret[..]).copy_from_slice(self.as_slice());
                    ret
                }
                fn from_be_bytes(src: [u8; std::mem::size_of::<Ident>()]) -> Self {
                    let mut tmp: Self = Self {
                        src: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    Self::from_be(tmp)
                }
                fn from_le_bytes(src: [u8; std::mem::size_of::<Ident>()]) -> Self {
                    let mut tmp: Self = Self {
                        src: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    Self::from_le(tmp)
                }
                fn from_ne_bytes(src: [u8; std::mem::size_of::<Ident>()]) -> Self {
                    let mut tmp: Self = Self {
                        src: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    tmp
                }
            }
        }
        use ident::Ident;
        use self::ident::version::Version;
        use super::basic_type::*;
        use crate::{interface::MetaData, EndWrapper, EndWrapperMut, Wrapper, WrapperMut};
        #[repr(packed)]
        pub struct Ehdr {
            e_ident: Ident,
            e_type: Half,
            e_machine: Half,
            e_version: Word,
            pub e_entry: Addr,
            #[phoff]
            pub e_phoff: Off,
            #[shoff]
            pub e_shoff: Off,
            pub e_flags: Word,
            pub e_ehsize: Half,
            #[phentsize]
            pub e_phentsize: Half,
            #[phnum]
            pub e_phnum: Half,
            #[shentsize]
            pub e_shentsize: Half,
            #[shnum]
            pub e_shnum: Half,
            pub e_shstrndx: Half,
        }
        impl crate::interface::MetaData<[u8; std::mem::size_of::<Ehdr>()]> for Ehdr {
            fn to_le(self) -> Self {
                Self {
                    e_ident: self.e_ident.to_le(),
                    e_type: self.e_type.to_le(),
                    e_machine: self.e_machine.to_le(),
                    e_version: self.e_version.to_le(),
                    e_entry: self.e_entry.to_le(),
                    e_phoff: self.e_phoff.to_le(),
                    e_shoff: self.e_shoff.to_le(),
                    e_flags: self.e_flags.to_le(),
                    e_ehsize: self.e_ehsize.to_le(),
                    e_phentsize: self.e_phentsize.to_le(),
                    e_phnum: self.e_phnum.to_le(),
                    e_shentsize: self.e_shentsize.to_le(),
                    e_shnum: self.e_shnum.to_le(),
                    e_shstrndx: self.e_shstrndx.to_le(),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    e_ident: self.e_ident.to_be(),
                    e_type: self.e_type.to_be(),
                    e_machine: self.e_machine.to_be(),
                    e_version: self.e_version.to_be(),
                    e_entry: self.e_entry.to_be(),
                    e_phoff: self.e_phoff.to_be(),
                    e_shoff: self.e_shoff.to_be(),
                    e_flags: self.e_flags.to_be(),
                    e_ehsize: self.e_ehsize.to_be(),
                    e_phentsize: self.e_phentsize.to_be(),
                    e_phnum: self.e_phnum.to_be(),
                    e_shentsize: self.e_shentsize.to_be(),
                    e_shnum: self.e_shnum.to_be(),
                    e_shstrndx: self.e_shstrndx.to_be(),
                }
            }
            fn from_le(value: Self) -> Self {
                Self {
                    e_ident: Ident::from_le(value.e_ident),
                    e_type: Half::from_le(value.e_type),
                    e_machine: Half::from_le(value.e_machine),
                    e_version: Word::from_le(value.e_version),
                    e_entry: Addr::from_le(value.e_entry),
                    e_phoff: Off::from_le(value.e_phoff),
                    e_shoff: Off::from_le(value.e_shoff),
                    e_flags: Word::from_le(value.e_flags),
                    e_ehsize: Half::from_le(value.e_ehsize),
                    e_phentsize: Half::from_le(value.e_phentsize),
                    e_phnum: Half::from_le(value.e_phnum),
                    e_shentsize: Half::from_le(value.e_shentsize),
                    e_shnum: Half::from_le(value.e_shnum),
                    e_shstrndx: Half::from_le(value.e_shstrndx),
                }
            }
            fn from_be(value: Self) -> Self {
                Self {
                    e_ident: Ident::from_be(value.e_ident),
                    e_type: Half::from_be(value.e_type),
                    e_machine: Half::from_be(value.e_machine),
                    e_version: Word::from_be(value.e_version),
                    e_entry: Addr::from_be(value.e_entry),
                    e_phoff: Off::from_be(value.e_phoff),
                    e_shoff: Off::from_be(value.e_shoff),
                    e_flags: Word::from_be(value.e_flags),
                    e_ehsize: Half::from_be(value.e_ehsize),
                    e_phentsize: Half::from_be(value.e_phentsize),
                    e_phnum: Half::from_be(value.e_phnum),
                    e_shentsize: Half::from_be(value.e_shentsize),
                    e_shnum: Half::from_be(value.e_shnum),
                    e_shstrndx: Half::from_be(value.e_shstrndx),
                }
            }
            fn to_be_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
                let tmp = self.to_be();
                let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_le_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
                let tmp = self.to_le();
                let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
                let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
                (&mut ret[..]).copy_from_slice(self.as_slice());
                ret
            }
            fn from_be_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
                let mut tmp: Self = Self {
                    e_ident: Default::default(),
                    e_type: Default::default(),
                    e_machine: Default::default(),
                    e_version: Default::default(),
                    e_entry: Default::default(),
                    e_phoff: Default::default(),
                    e_shoff: Default::default(),
                    e_flags: Default::default(),
                    e_ehsize: Default::default(),
                    e_phentsize: Default::default(),
                    e_phnum: Default::default(),
                    e_shentsize: Default::default(),
                    e_shnum: Default::default(),
                    e_shstrndx: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_be(tmp)
            }
            fn from_le_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
                let mut tmp: Self = Self {
                    e_ident: Default::default(),
                    e_type: Default::default(),
                    e_machine: Default::default(),
                    e_version: Default::default(),
                    e_entry: Default::default(),
                    e_phoff: Default::default(),
                    e_shoff: Default::default(),
                    e_flags: Default::default(),
                    e_ehsize: Default::default(),
                    e_phentsize: Default::default(),
                    e_phnum: Default::default(),
                    e_shentsize: Default::default(),
                    e_shnum: Default::default(),
                    e_shstrndx: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_le(tmp)
            }
            fn from_ne_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
                let mut tmp: Self = Self {
                    e_ident: Default::default(),
                    e_type: Default::default(),
                    e_machine: Default::default(),
                    e_version: Default::default(),
                    e_entry: Default::default(),
                    e_phoff: Default::default(),
                    e_shoff: Default::default(),
                    e_flags: Default::default(),
                    e_ehsize: Default::default(),
                    e_phentsize: Default::default(),
                    e_phnum: Default::default(),
                    e_shentsize: Default::default(),
                    e_shnum: Default::default(),
                    e_shstrndx: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                tmp
            }
        }
        impl crate::interface::Ehdr for Ehdr {
            fn shdr_table_range(&self) -> std::ops::RangeInclusive<usize> {
                (self.e_shoff as usize)
                    ..=(self.e_shnum as usize) * (self.e_shentsize as usize)
                        + (self.e_shoff as usize)
            }
            fn phdr_table_range(&self) -> std::ops::RangeInclusive<usize> {
                (self.e_phoff as usize)
                    ..=(self.e_phnum as usize) * (self.e_phentsize as usize)
                        + (self.e_phoff as usize)
            }
        }
        impl Default for Ehdr {
            fn default() -> Self {
                let tmp = [0u8; std::mem::size_of::<Ehdr>()];
                let mut ret = Ehdr::from_le_bytes(tmp);
                ret.e_version = Version::Current.into();
                ret.as_mut_slice()[0..std::mem::size_of_val(&ident::MAGIC)]
                    .copy_from_slice(&ident::MAGIC);
                ret
            }
        }
        impl Ehdr {
            pub fn getter(&self) -> crate::Wrapper<Self> {
                crate::Wrapper { src: self }
            }
            pub fn setter(&mut self) -> crate::WrapperMut<Self> {
                crate::WrapperMut { src: self }
            }
            pub fn getter_le(&self) -> EndWrapper<Self> {
                EndWrapper::<Self> {
                    src: self,
                    endiness: ident::encode::Encode::Lsb,
                }
            }
            pub fn setter_le(&mut self) -> EndWrapperMut<Self> {
                EndWrapperMut::<Self> {
                    src: self,
                    endiness: ident::encode::Encode::Lsb,
                }
            }
        }
        impl<'a> EndWrapper<'a, Ehdr> {}
        impl crate::Wrapper<'_, Ehdr> {
            pub fn version(&self) -> Version {
                self.src.e_version.into()
            }
            pub fn ident(&self) -> Wrapper<Ident> {
                Wrapper::<Ident> {
                    src: &self.src.e_ident,
                }
            }
            pub fn obj_type(&self) -> e_type::Type {
                self.src.e_type.into()
            }
        }
        impl crate::WrapperMut<'_, Ehdr> {
            pub fn ident(&mut self) -> WrapperMut<Ident> {
                WrapperMut::<Ident> {
                    src: &mut self.src.e_ident,
                }
            }
            pub fn obj_type(&mut self, ty: e_type::Type) {
                self.src.e_type = ty.into();
            }
        }
    }
    pub mod section {
        pub mod reloc {
            use super::super::basic_type::*;
            pub struct Rel {
                pub offset: Addr,
                pub info: Word,
            }
            pub struct Rela {
                pub offset: Addr,
                pub info: Word,
                pub addend: Sword,
            }
            impl Rel {
                pub fn r_sym_idx(&self) -> usize {
                    (self.info >> 8) as usize
                }
                pub fn r_type_idx(&self) -> usize {
                    (self.info & 0xf) as usize
                }
            }
            impl Rela {
                pub fn r_sym_idx(&self) -> usize {
                    (self.info >> 8) as usize
                }
                pub fn r_type_idx(&self) -> usize {
                    (self.info & 0xf) as usize
                }
            }
        }
        pub mod sh_flags {
            pub enum Flags {
                Write,
                Alloc,
                ExecInstr,
                Processor(u32),
                Unknown(u32),
            }
            pub struct Wrapper<'a> {
                pub(crate) shdr: &'a super::Shdr,
            }
            impl Wrapper<'_> {
                pub fn is_writeable(&self) -> bool {
                    self.shdr.sh_flags & WRITE != 0
                }
                pub fn is_allocable(&self) -> bool {
                    self.shdr.sh_flags & ALLOC != 0
                }
                pub fn any_processor(&self) -> Option<u32> {
                    let processor = self.shdr.sh_flags & MASK_PROCESSOR;
                    if processor != 0 {
                        Some(processor)
                    } else {
                        None
                    }
                }
                pub fn any_unknown(&self) -> Option<u32> {
                    let unknown = self.shdr.sh_flags & MASK_PROCESSOR;
                    if unknown != 0 {
                        Some(unknown)
                    } else {
                        None
                    }
                }
            }
            pub struct WrapperMut<'a> {
                pub(crate) shdr: &'a mut super::Shdr,
            }
            impl<'a> WrapperMut<'a> {
                /// 设置 Ehdr 中 sh_flags 的值。
                pub fn with(&'a mut self, val: Flags) -> &'a mut Self {
                    match val {
                        Flags::Write => self.shdr.sh_flags |= WRITE,
                        Flags::Alloc => self.shdr.sh_flags |= ALLOC,
                        Flags::ExecInstr => self.shdr.sh_flags |= EXECINSTR,
                        Flags::Processor(v) => self.shdr.sh_flags |= v & MASK_PROCESSOR,
                        Flags::Unknown(v) => self.shdr.sh_flags |= v & MASK_UNKNOWN,
                    };
                    self
                }
            }
            impl std::convert::Into<u32> for Flags {
                fn into(self) -> u32 {
                    match self {
                        Flags::Write => WRITE,
                        Flags::Alloc => ALLOC,
                        Flags::ExecInstr => EXECINSTR,
                        Flags::Processor(v) if (v & !MASK_PROCESSOR) != 0 => {
                            ::std::rt::begin_panic("Invalid processor bit")
                        }
                        Flags::Unknown(v) if (v & !MASK_UNKNOWN) != 0 => {
                            ::std::rt::begin_panic("Invalid unknown bit")
                        }
                        Flags::Processor(v) => v & MASK_PROCESSOR,
                        Flags::Unknown(v) => v & MASK_UNKNOWN,
                    }
                }
            }
            const WRITE: u32 = 0x1;
            const ALLOC: u32 = 0x2;
            const EXECINSTR: u32 = 0x4;
            const MASK_PROCESSOR: u32 = 0xf0000000;
            const MASK_UNKNOWN: u32 = (!WRITE) & (!ALLOC) & (!EXECINSTR) & (!MASK_PROCESSOR);
        }
        pub mod sh_type {
            use std::ops::RangeInclusive;
            pub enum Type {
                Null,
                Progbits,
                Symtab,
                Strtab,
                Rela,
                Hash,
                Dynamic,
                Note,
                Nobits,
                Rel,
                Shlib,
                Dynsym,
                Processor(u32),
                User(u32),
                Unknown(u32),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Type {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match (&*self,) {
                        (&Type::Null,) => {
                            let mut debug_trait_builder = f.debug_tuple("Null");
                            debug_trait_builder.finish()
                        }
                        (&Type::Progbits,) => {
                            let mut debug_trait_builder = f.debug_tuple("Progbits");
                            debug_trait_builder.finish()
                        }
                        (&Type::Symtab,) => {
                            let mut debug_trait_builder = f.debug_tuple("Symtab");
                            debug_trait_builder.finish()
                        }
                        (&Type::Strtab,) => {
                            let mut debug_trait_builder = f.debug_tuple("Strtab");
                            debug_trait_builder.finish()
                        }
                        (&Type::Rela,) => {
                            let mut debug_trait_builder = f.debug_tuple("Rela");
                            debug_trait_builder.finish()
                        }
                        (&Type::Hash,) => {
                            let mut debug_trait_builder = f.debug_tuple("Hash");
                            debug_trait_builder.finish()
                        }
                        (&Type::Dynamic,) => {
                            let mut debug_trait_builder = f.debug_tuple("Dynamic");
                            debug_trait_builder.finish()
                        }
                        (&Type::Note,) => {
                            let mut debug_trait_builder = f.debug_tuple("Note");
                            debug_trait_builder.finish()
                        }
                        (&Type::Nobits,) => {
                            let mut debug_trait_builder = f.debug_tuple("Nobits");
                            debug_trait_builder.finish()
                        }
                        (&Type::Rel,) => {
                            let mut debug_trait_builder = f.debug_tuple("Rel");
                            debug_trait_builder.finish()
                        }
                        (&Type::Shlib,) => {
                            let mut debug_trait_builder = f.debug_tuple("Shlib");
                            debug_trait_builder.finish()
                        }
                        (&Type::Dynsym,) => {
                            let mut debug_trait_builder = f.debug_tuple("Dynsym");
                            debug_trait_builder.finish()
                        }
                        (&Type::Processor(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Processor");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Type::User(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("User");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Type::Unknown(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Unknown");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl std::convert::From<u32> for Type {
                fn from(val: u32) -> Self {
                    match val {
                        NULL => Type::Null,
                        PROGBITS => Type::Progbits,
                        SYMTAB => Type::Symtab,
                        STRTAB => Type::Strtab,
                        RELA => Type::Rela,
                        HASH => Type::Hash,
                        DYNAMIC => Type::Dynamic,
                        NOTE => Type::Note,
                        NOBITS => Type::Nobits,
                        REL => Type::Rel,
                        SHLIB => Type::Shlib,
                        DYNSYM => Type::Dynsym,
                        LOPROC..=HIPROC => Type::Processor(val),
                        LOUSER..=HIUSER => Type::User(val),
                        _ => Type::Unknown(val),
                    }
                }
            }
            impl std::convert::Into<u32> for Type {
                /// 将 [`Type`] 转换为 u32 类型，如果类似 [`Type::Processor`] 中的值不在范围类，
                /// 则会 panic（大部分情况下这属于编码赋值问题，而非运行时错误，可以通过检查代码排查）
                fn into(self) -> u32 {
                    match self {
                        Type::Null => NULL,
                        Type::Progbits => PROGBITS,
                        Type::Symtab => SYMTAB,
                        Type::Strtab => STRTAB,
                        Type::Rela => RELA,
                        Type::Hash => HASH,
                        Type::Dynamic => DYNAMIC,
                        Type::Note => NOTE,
                        Type::Nobits => NOBITS,
                        Type::Rel => REL,
                        Type::Shlib => SHLIB,
                        Type::Dynsym => DYNSYM,
                        Type::Processor(v) if PROCRANGE.contains(&v) => v,
                        Type::User(v) if USERRANGE.contains(&v) => v,
                        Type::Unknown(v) if UNKNRANGE.contains(&v) => v,
                        Type::Processor(v) => {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                &["Invalid processor specified sh_type(", ")"],
                                &match (&v,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                        Type::User(v) => {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                &["Invalid user specified sh_type(", ")"],
                                &match (&v,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                        Type::Unknown(v) => {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                &["Invalid unknow specified sh_type(", ")"],
                                &match (&v,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                    }
                }
            }
            const NULL: u32 = 0;
            const PROGBITS: u32 = 1;
            const SYMTAB: u32 = 2;
            const STRTAB: u32 = 3;
            const RELA: u32 = 4;
            const HASH: u32 = 5;
            const DYNAMIC: u32 = 6;
            const NOTE: u32 = 7;
            const NOBITS: u32 = 8;
            const REL: u32 = 9;
            const SHLIB: u32 = 10;
            const DYNSYM: u32 = 11;
            const LOUNKN: u32 = 12;
            const HIUNKN: u32 = 0x6fffffff;
            const LOPROC: u32 = 0x70000000;
            const HIPROC: u32 = 0x7fffffff;
            const LOUSER: u32 = 0x80000000;
            const HIUSER: u32 = 0xffffffff;
            const PROCRANGE: RangeInclusive<u32> = LOPROC..=HIPROC;
            const USERRANGE: RangeInclusive<u32> = LOUSER..=HIUSER;
            const UNKNRANGE: RangeInclusive<u32> = LOUNKN..=HIUNKN;
        }
        pub mod sym {
            pub mod bind {
                pub enum Bind {
                    Local,
                    Global,
                    Weak,
                    Processor(u8),
                    Unknown(u8),
                }
                impl Into<u8> for Bind {
                    fn into(self) -> u8 {
                        match self {
                            Bind::Local => LOCAL,
                            Bind::Global => GLOBAL,
                            Bind::Weak => WEAK,
                            Bind::Processor(v) if v >= LOPROC && v <= HIPROC => v,
                            Bind::Processor(_) => ::std::rt::begin_panic(
                                "You should not construct a out range Processor Sym Bind variant",
                            ),
                            Bind::Unknown(v) => v,
                        }
                    }
                }
                impl From<u8> for Bind {
                    fn from(v: u8) -> Self {
                        match v {
                            LOCAL => Bind::Local,
                            GLOBAL => Bind::Global,
                            WEAK => Bind::Weak,
                            LOPROC..=HIPROC => Bind::Processor(v),
                            _ => Bind::Unknown(v),
                        }
                    }
                }
                const LOCAL: u8 = 0;
                const GLOBAL: u8 = 1;
                const WEAK: u8 = 2;
                const LOPROC: u8 = 13;
                const HIPROC: u8 = 15;
            }
            pub mod st_type {
                use std::ops::RangeInclusive;
                pub enum Type {
                    None,
                    Object,
                    Func,
                    Section,
                    Processor(u8),
                    File,
                    Unknown(u8),
                }
                impl Into<u8> for Type {
                    fn into(self) -> u8 {
                        match self {
                            Type::None => NOTYPE,
                            Type::Object => OBJECT,
                            Type::Func => FUNC,
                            Type::Section => SECTION,
                            Type::Processor(v) if PROCRANGE.contains(&v) => v,
                            Type::Processor(_) => ::std::rt::begin_panic(
                                "You should not construct a processor sym type out of range",
                            ),
                            Type::File => FILE,
                            Type::Unknown(v) => v,
                        }
                    }
                }
                impl From<u8> for Type {
                    fn from(v: u8) -> Self {
                        match v {
                            NOTYPE => Type::None,
                            OBJECT => Type::Object,
                            FUNC => Type::Func,
                            SECTION => Type::Section,
                            FILE => Type::File,
                            LOPROC..=HIPROC => Type::Processor(v),
                            _ => Type::Unknown(v),
                        }
                    }
                }
                const NOTYPE: u8 = 0;
                const OBJECT: u8 = 1;
                const FUNC: u8 = 2;
                const SECTION: u8 = 3;
                const FILE: u8 = 4;
                const LOPROC: u8 = 13;
                const HIPROC: u8 = 15;
                const PROCRANGE: RangeInclusive<u8> = LOPROC..=HIPROC;
            }
            use super::super::basic_type::*;
            #[repr(packed)]
            pub struct Sym {
                pub st_name: Word,
                pub st_value: Addr,
                pub st_size: Word,
                pub st_info: u8,
                pub st_other: u8,
                pub st_shndx: Half,
            }
            impl crate::interface::MetaData<[u8; std::mem::size_of::<Sym>()]> for Sym {
                fn to_le(self) -> Self {
                    Self {
                        st_name: self.st_name.to_le(),
                        st_value: self.st_value.to_le(),
                        st_size: self.st_size.to_le(),
                        st_info: self.st_info.to_le(),
                        st_other: self.st_other.to_le(),
                        st_shndx: self.st_shndx.to_le(),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        st_name: self.st_name.to_be(),
                        st_value: self.st_value.to_be(),
                        st_size: self.st_size.to_be(),
                        st_info: self.st_info.to_be(),
                        st_other: self.st_other.to_be(),
                        st_shndx: self.st_shndx.to_be(),
                    }
                }
                fn from_le(value: Self) -> Self {
                    Self {
                        st_name: Word::from_le(value.st_name),
                        st_value: Addr::from_le(value.st_value),
                        st_size: Word::from_le(value.st_size),
                        st_info: u8::from_le(value.st_info),
                        st_other: u8::from_le(value.st_other),
                        st_shndx: Half::from_le(value.st_shndx),
                    }
                }
                fn from_be(value: Self) -> Self {
                    Self {
                        st_name: Word::from_be(value.st_name),
                        st_value: Addr::from_be(value.st_value),
                        st_size: Word::from_be(value.st_size),
                        st_info: u8::from_be(value.st_info),
                        st_other: u8::from_be(value.st_other),
                        st_shndx: Half::from_be(value.st_shndx),
                    }
                }
                fn to_be_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
                    let tmp = self.to_be();
                    let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
                    (&mut ret[..]).copy_from_slice(tmp.as_slice());
                    ret
                }
                fn to_le_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
                    let tmp = self.to_le();
                    let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
                    (&mut ret[..]).copy_from_slice(tmp.as_slice());
                    ret
                }
                fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
                    let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
                    (&mut ret[..]).copy_from_slice(self.as_slice());
                    ret
                }
                fn from_be_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
                    let mut tmp: Self = Self {
                        st_name: Default::default(),
                        st_value: Default::default(),
                        st_size: Default::default(),
                        st_info: Default::default(),
                        st_other: Default::default(),
                        st_shndx: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    Self::from_be(tmp)
                }
                fn from_le_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
                    let mut tmp: Self = Self {
                        st_name: Default::default(),
                        st_value: Default::default(),
                        st_size: Default::default(),
                        st_info: Default::default(),
                        st_other: Default::default(),
                        st_shndx: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    Self::from_le(tmp)
                }
                fn from_ne_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
                    let mut tmp: Self = Self {
                        st_name: Default::default(),
                        st_value: Default::default(),
                        st_size: Default::default(),
                        st_info: Default::default(),
                        st_other: Default::default(),
                        st_shndx: Default::default(),
                    };
                    tmp.read_from_slice(src.as_ref());
                    tmp
                }
            }
        }
        use super::basic_type::*;
        #[repr(packed)]
        pub struct Shdr {
            pub sh_name: Word,
            pub sh_type: Word,
            pub sh_flags: Word,
            pub sh_addr: Addr,
            #[offset]
            pub sh_offset: Off,
            #[size]
            pub sh_size: Word,
            pub sh_link: Word,
            pub sh_info: Word,
            pub sh_addralign: Word,
            #[entsize]
            pub sh_entsize: Word,
        }
        impl crate::interface::MetaData<[u8; std::mem::size_of::<Shdr>()]> for Shdr {
            fn to_le(self) -> Self {
                Self {
                    sh_name: self.sh_name.to_le(),
                    sh_type: self.sh_type.to_le(),
                    sh_flags: self.sh_flags.to_le(),
                    sh_addr: self.sh_addr.to_le(),
                    sh_offset: self.sh_offset.to_le(),
                    sh_size: self.sh_size.to_le(),
                    sh_link: self.sh_link.to_le(),
                    sh_info: self.sh_info.to_le(),
                    sh_addralign: self.sh_addralign.to_le(),
                    sh_entsize: self.sh_entsize.to_le(),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    sh_name: self.sh_name.to_be(),
                    sh_type: self.sh_type.to_be(),
                    sh_flags: self.sh_flags.to_be(),
                    sh_addr: self.sh_addr.to_be(),
                    sh_offset: self.sh_offset.to_be(),
                    sh_size: self.sh_size.to_be(),
                    sh_link: self.sh_link.to_be(),
                    sh_info: self.sh_info.to_be(),
                    sh_addralign: self.sh_addralign.to_be(),
                    sh_entsize: self.sh_entsize.to_be(),
                }
            }
            fn from_le(value: Self) -> Self {
                Self {
                    sh_name: Word::from_le(value.sh_name),
                    sh_type: Word::from_le(value.sh_type),
                    sh_flags: Word::from_le(value.sh_flags),
                    sh_addr: Addr::from_le(value.sh_addr),
                    sh_offset: Off::from_le(value.sh_offset),
                    sh_size: Word::from_le(value.sh_size),
                    sh_link: Word::from_le(value.sh_link),
                    sh_info: Word::from_le(value.sh_info),
                    sh_addralign: Word::from_le(value.sh_addralign),
                    sh_entsize: Word::from_le(value.sh_entsize),
                }
            }
            fn from_be(value: Self) -> Self {
                Self {
                    sh_name: Word::from_be(value.sh_name),
                    sh_type: Word::from_be(value.sh_type),
                    sh_flags: Word::from_be(value.sh_flags),
                    sh_addr: Addr::from_be(value.sh_addr),
                    sh_offset: Off::from_be(value.sh_offset),
                    sh_size: Word::from_be(value.sh_size),
                    sh_link: Word::from_be(value.sh_link),
                    sh_info: Word::from_be(value.sh_info),
                    sh_addralign: Word::from_be(value.sh_addralign),
                    sh_entsize: Word::from_be(value.sh_entsize),
                }
            }
            fn to_be_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
                let tmp = self.to_be();
                let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_le_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
                let tmp = self.to_le();
                let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
                let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
                (&mut ret[..]).copy_from_slice(self.as_slice());
                ret
            }
            fn from_be_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
                let mut tmp: Self = Self {
                    sh_name: Default::default(),
                    sh_type: Default::default(),
                    sh_flags: Default::default(),
                    sh_addr: Default::default(),
                    sh_offset: Default::default(),
                    sh_size: Default::default(),
                    sh_link: Default::default(),
                    sh_info: Default::default(),
                    sh_addralign: Default::default(),
                    sh_entsize: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_be(tmp)
            }
            fn from_le_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
                let mut tmp: Self = Self {
                    sh_name: Default::default(),
                    sh_type: Default::default(),
                    sh_flags: Default::default(),
                    sh_addr: Default::default(),
                    sh_offset: Default::default(),
                    sh_size: Default::default(),
                    sh_link: Default::default(),
                    sh_info: Default::default(),
                    sh_addralign: Default::default(),
                    sh_entsize: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_le(tmp)
            }
            fn from_ne_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
                let mut tmp: Self = Self {
                    sh_name: Default::default(),
                    sh_type: Default::default(),
                    sh_flags: Default::default(),
                    sh_addr: Default::default(),
                    sh_offset: Default::default(),
                    sh_size: Default::default(),
                    sh_link: Default::default(),
                    sh_info: Default::default(),
                    sh_addralign: Default::default(),
                    sh_entsize: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                tmp
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for Shdr {
            #[inline]
            fn default() -> Shdr {
                Shdr {
                    sh_name: ::core::default::Default::default(),
                    sh_type: ::core::default::Default::default(),
                    sh_flags: ::core::default::Default::default(),
                    sh_addr: ::core::default::Default::default(),
                    sh_offset: ::core::default::Default::default(),
                    sh_size: ::core::default::Default::default(),
                    sh_link: ::core::default::Default::default(),
                    sh_info: ::core::default::Default::default(),
                    sh_addralign: ::core::default::Default::default(),
                    sh_entsize: ::core::default::Default::default(),
                }
            }
        }
        impl crate::interface::Shdr for Shdr {
            fn data_range(&self) -> std::ops::RangeInclusive<usize> {
                (self.sh_offset as usize)..=self.sh_size as usize
            }
            fn entsize(&self) -> usize {
                self.sh_entsize as usize
            }
        }
        pub struct Wrapper<'a> {
            shdr: &'a Shdr,
        }
        impl Wrapper<'_> {
            pub fn sh_type(&self) -> sh_type::Type {
                self.shdr.sh_type.into()
            }
            pub fn sh_flags(&self) -> sh_flags::Wrapper {
                sh_flags::Wrapper { shdr: &self.shdr }
            }
        }
        pub struct WrapperMut<'a> {
            shdr: &'a mut Shdr,
        }
        impl WrapperMut<'_> {
            pub fn sh_type(&mut self, val: sh_type::Type) {
                self.shdr.sh_type = val.into();
            }
            pub fn sh_flags(&mut self) -> sh_flags::WrapperMut {
                sh_flags::WrapperMut {
                    shdr: &mut self.shdr,
                }
            }
        }
    }
    pub mod segment {
        use super::basic_type::*;
        #[repr(packed)]
        pub struct Phdr {
            pub p_type: Word,
            pub p_offset: Off,
            pub p_vaddr: Addr,
            pub p_paddr: Addr,
            pub p_filesz: Word,
            pub p_memsz: Word,
            pub p_flags: Word,
            pub p_align: Word,
        }
        impl crate::interface::MetaData<[u8; std::mem::size_of::<Phdr>()]> for Phdr {
            fn to_le(self) -> Self {
                Self {
                    p_type: self.p_type.to_le(),
                    p_offset: self.p_offset.to_le(),
                    p_vaddr: self.p_vaddr.to_le(),
                    p_paddr: self.p_paddr.to_le(),
                    p_filesz: self.p_filesz.to_le(),
                    p_memsz: self.p_memsz.to_le(),
                    p_flags: self.p_flags.to_le(),
                    p_align: self.p_align.to_le(),
                }
            }
            fn to_be(self) -> Self {
                Self {
                    p_type: self.p_type.to_be(),
                    p_offset: self.p_offset.to_be(),
                    p_vaddr: self.p_vaddr.to_be(),
                    p_paddr: self.p_paddr.to_be(),
                    p_filesz: self.p_filesz.to_be(),
                    p_memsz: self.p_memsz.to_be(),
                    p_flags: self.p_flags.to_be(),
                    p_align: self.p_align.to_be(),
                }
            }
            fn from_le(value: Self) -> Self {
                Self {
                    p_type: Word::from_le(value.p_type),
                    p_offset: Off::from_le(value.p_offset),
                    p_vaddr: Addr::from_le(value.p_vaddr),
                    p_paddr: Addr::from_le(value.p_paddr),
                    p_filesz: Word::from_le(value.p_filesz),
                    p_memsz: Word::from_le(value.p_memsz),
                    p_flags: Word::from_le(value.p_flags),
                    p_align: Word::from_le(value.p_align),
                }
            }
            fn from_be(value: Self) -> Self {
                Self {
                    p_type: Word::from_be(value.p_type),
                    p_offset: Off::from_be(value.p_offset),
                    p_vaddr: Addr::from_be(value.p_vaddr),
                    p_paddr: Addr::from_be(value.p_paddr),
                    p_filesz: Word::from_be(value.p_filesz),
                    p_memsz: Word::from_be(value.p_memsz),
                    p_flags: Word::from_be(value.p_flags),
                    p_align: Word::from_be(value.p_align),
                }
            }
            fn to_be_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
                let tmp = self.to_be();
                let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_le_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
                let tmp = self.to_le();
                let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
                let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
                (&mut ret[..]).copy_from_slice(self.as_slice());
                ret
            }
            fn from_be_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
                let mut tmp: Self = Self {
                    p_type: Default::default(),
                    p_offset: Default::default(),
                    p_vaddr: Default::default(),
                    p_paddr: Default::default(),
                    p_filesz: Default::default(),
                    p_memsz: Default::default(),
                    p_flags: Default::default(),
                    p_align: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_be(tmp)
            }
            fn from_le_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
                let mut tmp: Self = Self {
                    p_type: Default::default(),
                    p_offset: Default::default(),
                    p_vaddr: Default::default(),
                    p_paddr: Default::default(),
                    p_filesz: Default::default(),
                    p_memsz: Default::default(),
                    p_flags: Default::default(),
                    p_align: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_le(tmp)
            }
            fn from_ne_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
                let mut tmp: Self = Self {
                    p_type: Default::default(),
                    p_offset: Default::default(),
                    p_vaddr: Default::default(),
                    p_paddr: Default::default(),
                    p_filesz: Default::default(),
                    p_memsz: Default::default(),
                    p_flags: Default::default(),
                    p_align: Default::default(),
                };
                tmp.read_from_slice(src.as_ref());
                tmp
            }
        }
    }
}
pub mod elf64 {
    pub mod sh_type {
        pub struct Wrapper<'a> {
            shdr: &'a super::Shdr,
        }
        impl Wrapper<'_> {
            pub fn get(&self) -> Type {
                self.shdr.sh_type.into()
            }
        }
        pub struct WrapperMut<'a> {
            shdr: &'a mut super::Shdr,
        }
        impl WrapperMut<'_> {
            pub fn with(&mut self, val: Type) {
                self.shdr.sh_type = val.into();
            }
        }
        pub enum Type {
            Null,
            Progbits,
            Symtab,
            Strtab,
            Rela,
            Hash,
            Dynamic,
            Note,
            Nobits,
            Rel,
            Shlib,
            Dynsym,
            Processor(u32),
            Os(u32),
            Unknown(u32),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Type::Null,) => {
                        let mut debug_trait_builder = f.debug_tuple("Null");
                        debug_trait_builder.finish()
                    }
                    (&Type::Progbits,) => {
                        let mut debug_trait_builder = f.debug_tuple("Progbits");
                        debug_trait_builder.finish()
                    }
                    (&Type::Symtab,) => {
                        let mut debug_trait_builder = f.debug_tuple("Symtab");
                        debug_trait_builder.finish()
                    }
                    (&Type::Strtab,) => {
                        let mut debug_trait_builder = f.debug_tuple("Strtab");
                        debug_trait_builder.finish()
                    }
                    (&Type::Rela,) => {
                        let mut debug_trait_builder = f.debug_tuple("Rela");
                        debug_trait_builder.finish()
                    }
                    (&Type::Hash,) => {
                        let mut debug_trait_builder = f.debug_tuple("Hash");
                        debug_trait_builder.finish()
                    }
                    (&Type::Dynamic,) => {
                        let mut debug_trait_builder = f.debug_tuple("Dynamic");
                        debug_trait_builder.finish()
                    }
                    (&Type::Note,) => {
                        let mut debug_trait_builder = f.debug_tuple("Note");
                        debug_trait_builder.finish()
                    }
                    (&Type::Nobits,) => {
                        let mut debug_trait_builder = f.debug_tuple("Nobits");
                        debug_trait_builder.finish()
                    }
                    (&Type::Rel,) => {
                        let mut debug_trait_builder = f.debug_tuple("Rel");
                        debug_trait_builder.finish()
                    }
                    (&Type::Shlib,) => {
                        let mut debug_trait_builder = f.debug_tuple("Shlib");
                        debug_trait_builder.finish()
                    }
                    (&Type::Dynsym,) => {
                        let mut debug_trait_builder = f.debug_tuple("Dynsym");
                        debug_trait_builder.finish()
                    }
                    (&Type::Processor(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Processor");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Os(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Os");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Unknown(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Unknown");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for Type {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for Type {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Type {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Type {
            #[inline]
            fn eq(&self, other: &Type) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Type::Processor(ref __self_0), &Type::Processor(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&Type::Os(ref __self_0), &Type::Os(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&Type::Unknown(ref __self_0), &Type::Unknown(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &Type) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Type::Processor(ref __self_0), &Type::Processor(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&Type::Os(ref __self_0), &Type::Os(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&Type::Unknown(ref __self_0), &Type::Unknown(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            _ => false,
                        }
                    } else {
                        true
                    }
                }
            }
        }
        impl std::convert::From<u32> for Type {
            fn from(val: u32) -> Self {
                match val {
                    NULL => Type::Null,
                    PROGBITS => Type::Progbits,
                    SYMTAB => Type::Symtab,
                    STRTAB => Type::Strtab,
                    RELA => Type::Rela,
                    HASH => Type::Hash,
                    DYNAMIC => Type::Dynamic,
                    NOTE => Type::Note,
                    NOBITS => Type::Nobits,
                    REL => Type::Rel,
                    SHLIB => Type::Shlib,
                    DYNSYM => Type::Dynsym,
                    LOPROC..=HIPROC => Type::Processor(val),
                    LOOS..=HIOS => Type::Os(val),
                    _ => Type::Unknown(val),
                }
            }
        }
        impl std::convert::Into<u32> for Type {
            fn into(self) -> u32 {
                match self {
                    Type::Null => NULL,
                    Type::Progbits => PROGBITS,
                    Type::Symtab => SYMTAB,
                    Type::Strtab => STRTAB,
                    Type::Rela => RELA,
                    Type::Hash => HASH,
                    Type::Dynamic => DYNAMIC,
                    Type::Note => NOTE,
                    Type::Nobits => NOBITS,
                    Type::Rel => REL,
                    Type::Shlib => SHLIB,
                    Type::Dynsym => DYNSYM,
                    Type::Processor(v) if v < LOPROC || v > HIPROC => {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Invalid processor specified sh_type(", ")"],
                            &match (&v,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                    Type::Os(v) if v < LOOS || v > HIOS => {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Invalid os specified sh_type(", ")"],
                            &match (&v,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                    Type::Unknown(v)
                        if (v >= LOOS && v <= HIOS) || (v >= LOPROC && v <= HIPROC) =>
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Invalid unknown specified sh_type(", ")"],
                            &match (&v,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                    Type::Processor(v) | Type::Os(v) | Type::Unknown(v) => v,
                }
            }
        }
        const NULL: u32 = 0;
        const PROGBITS: u32 = 1;
        const SYMTAB: u32 = 2;
        const STRTAB: u32 = 3;
        const RELA: u32 = 4;
        const HASH: u32 = 5;
        const DYNAMIC: u32 = 6;
        const NOTE: u32 = 7;
        const NOBITS: u32 = 8;
        const REL: u32 = 9;
        const SHLIB: u32 = 10;
        const DYNSYM: u32 = 11;
        const LOOS: u32 = 0x60000000;
        const HIOS: u32 = 0x6FFFFFFF;
        const LOPROC: u32 = 0x70000000;
        const HIPROC: u32 = 0x7FFFFFFF;
    }
    pub type Addr = u64;
    pub type Off = u64;
    pub type Half = u16;
    pub type Word = u32;
    pub type Sword = u32;
    pub type Xword = u64;
    pub type Sxword = i64;
    #[repr(packed)]
    pub struct Ehdr {
        pub e_ident: [u8; 16],
        pub e_type: Half,
        pub e_machine: Half,
        pub e_version: Word,
        pub e_entry: Addr,
        #[phoff]
        pub e_phoff: Off,
        #[shoff]
        pub e_shoff: Off,
        pub e_flags: Word,
        pub e_ehsize: Half,
        #[phentsize]
        pub e_phentsize: Half,
        #[phnum]
        pub e_phnum: Half,
        #[shentsize]
        pub e_shentsize: Half,
        #[shnum]
        pub e_shnum: Half,
        pub e_shstrndx: Half,
    }
    impl crate::interface::MetaData<[u8; std::mem::size_of::<Ehdr>()]> for Ehdr {
        fn to_le(self) -> Self {
            Self {
                e_type: self.e_type.to_le(),
                e_machine: self.e_machine.to_le(),
                e_version: self.e_version.to_le(),
                e_entry: self.e_entry.to_le(),
                e_phoff: self.e_phoff.to_le(),
                e_shoff: self.e_shoff.to_le(),
                e_flags: self.e_flags.to_le(),
                e_ehsize: self.e_ehsize.to_le(),
                e_phentsize: self.e_phentsize.to_le(),
                e_phnum: self.e_phnum.to_le(),
                e_shentsize: self.e_shentsize.to_le(),
                e_shnum: self.e_shnum.to_le(),
                e_shstrndx: self.e_shstrndx.to_le(),
                e_ident: self.e_ident,
            }
        }
        fn to_be(self) -> Self {
            Self {
                e_type: self.e_type.to_be(),
                e_machine: self.e_machine.to_be(),
                e_version: self.e_version.to_be(),
                e_entry: self.e_entry.to_be(),
                e_phoff: self.e_phoff.to_be(),
                e_shoff: self.e_shoff.to_be(),
                e_flags: self.e_flags.to_be(),
                e_ehsize: self.e_ehsize.to_be(),
                e_phentsize: self.e_phentsize.to_be(),
                e_phnum: self.e_phnum.to_be(),
                e_shentsize: self.e_shentsize.to_be(),
                e_shnum: self.e_shnum.to_be(),
                e_shstrndx: self.e_shstrndx.to_be(),
                e_ident: self.e_ident,
            }
        }
        fn from_le(value: Self) -> Self {
            Self {
                e_type: Half::from_le(value.e_type),
                e_machine: Half::from_le(value.e_machine),
                e_version: Word::from_le(value.e_version),
                e_entry: Addr::from_le(value.e_entry),
                e_phoff: Off::from_le(value.e_phoff),
                e_shoff: Off::from_le(value.e_shoff),
                e_flags: Word::from_le(value.e_flags),
                e_ehsize: Half::from_le(value.e_ehsize),
                e_phentsize: Half::from_le(value.e_phentsize),
                e_phnum: Half::from_le(value.e_phnum),
                e_shentsize: Half::from_le(value.e_shentsize),
                e_shnum: Half::from_le(value.e_shnum),
                e_shstrndx: Half::from_le(value.e_shstrndx),
                e_ident: value.e_ident,
            }
        }
        fn from_be(value: Self) -> Self {
            Self {
                e_type: Half::from_be(value.e_type),
                e_machine: Half::from_be(value.e_machine),
                e_version: Word::from_be(value.e_version),
                e_entry: Addr::from_be(value.e_entry),
                e_phoff: Off::from_be(value.e_phoff),
                e_shoff: Off::from_be(value.e_shoff),
                e_flags: Word::from_be(value.e_flags),
                e_ehsize: Half::from_be(value.e_ehsize),
                e_phentsize: Half::from_be(value.e_phentsize),
                e_phnum: Half::from_be(value.e_phnum),
                e_shentsize: Half::from_be(value.e_shentsize),
                e_shnum: Half::from_be(value.e_shnum),
                e_shstrndx: Half::from_be(value.e_shstrndx),
                e_ident: value.e_ident,
            }
        }
        fn to_be_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
            let tmp = self.to_be();
            let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_le_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
            let tmp = self.to_le();
            let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Ehdr>()] {
            let mut ret: [u8; std::mem::size_of::<Ehdr>()] = [0; std::mem::size_of::<Ehdr>()];
            (&mut ret[..]).copy_from_slice(self.as_slice());
            ret
        }
        fn from_be_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
            let mut tmp: Self = Self {
                e_type: Default::default(),
                e_machine: Default::default(),
                e_version: Default::default(),
                e_entry: Default::default(),
                e_phoff: Default::default(),
                e_shoff: Default::default(),
                e_flags: Default::default(),
                e_ehsize: Default::default(),
                e_phentsize: Default::default(),
                e_phnum: Default::default(),
                e_shentsize: Default::default(),
                e_shnum: Default::default(),
                e_shstrndx: Default::default(),
                e_ident: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_be(tmp)
        }
        fn from_le_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
            let mut tmp: Self = Self {
                e_type: Default::default(),
                e_machine: Default::default(),
                e_version: Default::default(),
                e_entry: Default::default(),
                e_phoff: Default::default(),
                e_shoff: Default::default(),
                e_flags: Default::default(),
                e_ehsize: Default::default(),
                e_phentsize: Default::default(),
                e_phnum: Default::default(),
                e_shentsize: Default::default(),
                e_shnum: Default::default(),
                e_shstrndx: Default::default(),
                e_ident: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_le(tmp)
        }
        fn from_ne_bytes(src: [u8; std::mem::size_of::<Ehdr>()]) -> Self {
            let mut tmp: Self = Self {
                e_type: Default::default(),
                e_machine: Default::default(),
                e_version: Default::default(),
                e_entry: Default::default(),
                e_phoff: Default::default(),
                e_shoff: Default::default(),
                e_flags: Default::default(),
                e_ehsize: Default::default(),
                e_phentsize: Default::default(),
                e_phnum: Default::default(),
                e_shentsize: Default::default(),
                e_shnum: Default::default(),
                e_shstrndx: Default::default(),
                e_ident: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            tmp
        }
    }
    impl crate::interface::Ehdr for Ehdr {
        fn shdr_table_range(&self) -> std::ops::RangeInclusive<usize> {
            (self.e_shoff as usize)
                ..=(self.e_shnum as usize) * (self.e_shentsize as usize) + (self.e_shoff as usize)
        }
        fn phdr_table_range(&self) -> std::ops::RangeInclusive<usize> {
            (self.e_phoff as usize)
                ..=(self.e_phnum as usize) * (self.e_phentsize as usize) + (self.e_phoff as usize)
        }
    }
    #[repr(packed)]
    pub struct Shdr {
        pub sh_name: Word,
        pub sh_type: Word,
        pub sh_flags: Xword,
        pub sh_addr: Addr,
        pub sh_offset: Off,
        pub sh_size: Xword,
        pub sh_link: Word,
        pub sh_info: Word,
        pub sh_addralign: Xword,
        pub sh_entsize: Xword,
    }
    impl crate::interface::MetaData<[u8; std::mem::size_of::<Shdr>()]> for Shdr {
        fn to_le(self) -> Self {
            Self {
                sh_name: self.sh_name.to_le(),
                sh_type: self.sh_type.to_le(),
                sh_flags: self.sh_flags.to_le(),
                sh_addr: self.sh_addr.to_le(),
                sh_offset: self.sh_offset.to_le(),
                sh_size: self.sh_size.to_le(),
                sh_link: self.sh_link.to_le(),
                sh_info: self.sh_info.to_le(),
                sh_addralign: self.sh_addralign.to_le(),
                sh_entsize: self.sh_entsize.to_le(),
            }
        }
        fn to_be(self) -> Self {
            Self {
                sh_name: self.sh_name.to_be(),
                sh_type: self.sh_type.to_be(),
                sh_flags: self.sh_flags.to_be(),
                sh_addr: self.sh_addr.to_be(),
                sh_offset: self.sh_offset.to_be(),
                sh_size: self.sh_size.to_be(),
                sh_link: self.sh_link.to_be(),
                sh_info: self.sh_info.to_be(),
                sh_addralign: self.sh_addralign.to_be(),
                sh_entsize: self.sh_entsize.to_be(),
            }
        }
        fn from_le(value: Self) -> Self {
            Self {
                sh_name: Word::from_le(value.sh_name),
                sh_type: Word::from_le(value.sh_type),
                sh_flags: Xword::from_le(value.sh_flags),
                sh_addr: Addr::from_le(value.sh_addr),
                sh_offset: Off::from_le(value.sh_offset),
                sh_size: Xword::from_le(value.sh_size),
                sh_link: Word::from_le(value.sh_link),
                sh_info: Word::from_le(value.sh_info),
                sh_addralign: Xword::from_le(value.sh_addralign),
                sh_entsize: Xword::from_le(value.sh_entsize),
            }
        }
        fn from_be(value: Self) -> Self {
            Self {
                sh_name: Word::from_be(value.sh_name),
                sh_type: Word::from_be(value.sh_type),
                sh_flags: Xword::from_be(value.sh_flags),
                sh_addr: Addr::from_be(value.sh_addr),
                sh_offset: Off::from_be(value.sh_offset),
                sh_size: Xword::from_be(value.sh_size),
                sh_link: Word::from_be(value.sh_link),
                sh_info: Word::from_be(value.sh_info),
                sh_addralign: Xword::from_be(value.sh_addralign),
                sh_entsize: Xword::from_be(value.sh_entsize),
            }
        }
        fn to_be_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
            let tmp = self.to_be();
            let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_le_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
            let tmp = self.to_le();
            let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Shdr>()] {
            let mut ret: [u8; std::mem::size_of::<Shdr>()] = [0; std::mem::size_of::<Shdr>()];
            (&mut ret[..]).copy_from_slice(self.as_slice());
            ret
        }
        fn from_be_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
            let mut tmp: Self = Self {
                sh_name: Default::default(),
                sh_type: Default::default(),
                sh_flags: Default::default(),
                sh_addr: Default::default(),
                sh_offset: Default::default(),
                sh_size: Default::default(),
                sh_link: Default::default(),
                sh_info: Default::default(),
                sh_addralign: Default::default(),
                sh_entsize: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_be(tmp)
        }
        fn from_le_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
            let mut tmp: Self = Self {
                sh_name: Default::default(),
                sh_type: Default::default(),
                sh_flags: Default::default(),
                sh_addr: Default::default(),
                sh_offset: Default::default(),
                sh_size: Default::default(),
                sh_link: Default::default(),
                sh_info: Default::default(),
                sh_addralign: Default::default(),
                sh_entsize: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_le(tmp)
        }
        fn from_ne_bytes(src: [u8; std::mem::size_of::<Shdr>()]) -> Self {
            let mut tmp: Self = Self {
                sh_name: Default::default(),
                sh_type: Default::default(),
                sh_flags: Default::default(),
                sh_addr: Default::default(),
                sh_offset: Default::default(),
                sh_size: Default::default(),
                sh_link: Default::default(),
                sh_info: Default::default(),
                sh_addralign: Default::default(),
                sh_entsize: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            tmp
        }
    }
    #[repr(packed)]
    pub struct Sym {
        pub st_name: Word,
        pub st_info: u8,
        pub st_other: u8,
        pub st_shndx: Half,
        pub st_value: Addr,
        pub st_size: Xword,
    }
    impl crate::interface::MetaData<[u8; std::mem::size_of::<Sym>()]> for Sym {
        fn to_le(self) -> Self {
            Self {
                st_name: self.st_name.to_le(),
                st_info: self.st_info.to_le(),
                st_other: self.st_other.to_le(),
                st_shndx: self.st_shndx.to_le(),
                st_value: self.st_value.to_le(),
                st_size: self.st_size.to_le(),
            }
        }
        fn to_be(self) -> Self {
            Self {
                st_name: self.st_name.to_be(),
                st_info: self.st_info.to_be(),
                st_other: self.st_other.to_be(),
                st_shndx: self.st_shndx.to_be(),
                st_value: self.st_value.to_be(),
                st_size: self.st_size.to_be(),
            }
        }
        fn from_le(value: Self) -> Self {
            Self {
                st_name: Word::from_le(value.st_name),
                st_info: u8::from_le(value.st_info),
                st_other: u8::from_le(value.st_other),
                st_shndx: Half::from_le(value.st_shndx),
                st_value: Addr::from_le(value.st_value),
                st_size: Xword::from_le(value.st_size),
            }
        }
        fn from_be(value: Self) -> Self {
            Self {
                st_name: Word::from_be(value.st_name),
                st_info: u8::from_be(value.st_info),
                st_other: u8::from_be(value.st_other),
                st_shndx: Half::from_be(value.st_shndx),
                st_value: Addr::from_be(value.st_value),
                st_size: Xword::from_be(value.st_size),
            }
        }
        fn to_be_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
            let tmp = self.to_be();
            let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_le_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
            let tmp = self.to_le();
            let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Sym>()] {
            let mut ret: [u8; std::mem::size_of::<Sym>()] = [0; std::mem::size_of::<Sym>()];
            (&mut ret[..]).copy_from_slice(self.as_slice());
            ret
        }
        fn from_be_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
            let mut tmp: Self = Self {
                st_name: Default::default(),
                st_info: Default::default(),
                st_other: Default::default(),
                st_shndx: Default::default(),
                st_value: Default::default(),
                st_size: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_be(tmp)
        }
        fn from_le_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
            let mut tmp: Self = Self {
                st_name: Default::default(),
                st_info: Default::default(),
                st_other: Default::default(),
                st_shndx: Default::default(),
                st_value: Default::default(),
                st_size: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_le(tmp)
        }
        fn from_ne_bytes(src: [u8; std::mem::size_of::<Sym>()]) -> Self {
            let mut tmp: Self = Self {
                st_name: Default::default(),
                st_info: Default::default(),
                st_other: Default::default(),
                st_shndx: Default::default(),
                st_value: Default::default(),
                st_size: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            tmp
        }
    }
    #[repr(packed)]
    pub struct Phdr {
        pub p_type: Word,
        pub p_flags: Word,
        pub p_offset: Off,
        pub p_vaddr: Addr,
        pub p_paddr: Addr,
        pub p_filesz: Xword,
        pub p_memsz: Xword,
        pub p_align: Xword,
    }
    impl crate::interface::MetaData<[u8; std::mem::size_of::<Phdr>()]> for Phdr {
        fn to_le(self) -> Self {
            Self {
                p_type: self.p_type.to_le(),
                p_flags: self.p_flags.to_le(),
                p_offset: self.p_offset.to_le(),
                p_vaddr: self.p_vaddr.to_le(),
                p_paddr: self.p_paddr.to_le(),
                p_filesz: self.p_filesz.to_le(),
                p_memsz: self.p_memsz.to_le(),
                p_align: self.p_align.to_le(),
            }
        }
        fn to_be(self) -> Self {
            Self {
                p_type: self.p_type.to_be(),
                p_flags: self.p_flags.to_be(),
                p_offset: self.p_offset.to_be(),
                p_vaddr: self.p_vaddr.to_be(),
                p_paddr: self.p_paddr.to_be(),
                p_filesz: self.p_filesz.to_be(),
                p_memsz: self.p_memsz.to_be(),
                p_align: self.p_align.to_be(),
            }
        }
        fn from_le(value: Self) -> Self {
            Self {
                p_type: Word::from_le(value.p_type),
                p_flags: Word::from_le(value.p_flags),
                p_offset: Off::from_le(value.p_offset),
                p_vaddr: Addr::from_le(value.p_vaddr),
                p_paddr: Addr::from_le(value.p_paddr),
                p_filesz: Xword::from_le(value.p_filesz),
                p_memsz: Xword::from_le(value.p_memsz),
                p_align: Xword::from_le(value.p_align),
            }
        }
        fn from_be(value: Self) -> Self {
            Self {
                p_type: Word::from_be(value.p_type),
                p_flags: Word::from_be(value.p_flags),
                p_offset: Off::from_be(value.p_offset),
                p_vaddr: Addr::from_be(value.p_vaddr),
                p_paddr: Addr::from_be(value.p_paddr),
                p_filesz: Xword::from_be(value.p_filesz),
                p_memsz: Xword::from_be(value.p_memsz),
                p_align: Xword::from_be(value.p_align),
            }
        }
        fn to_be_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
            let tmp = self.to_be();
            let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_le_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
            let tmp = self.to_le();
            let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
            (&mut ret[..]).copy_from_slice(tmp.as_slice());
            ret
        }
        fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Phdr>()] {
            let mut ret: [u8; std::mem::size_of::<Phdr>()] = [0; std::mem::size_of::<Phdr>()];
            (&mut ret[..]).copy_from_slice(self.as_slice());
            ret
        }
        fn from_be_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
            let mut tmp: Self = Self {
                p_type: Default::default(),
                p_flags: Default::default(),
                p_offset: Default::default(),
                p_vaddr: Default::default(),
                p_paddr: Default::default(),
                p_filesz: Default::default(),
                p_memsz: Default::default(),
                p_align: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_be(tmp)
        }
        fn from_le_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
            let mut tmp: Self = Self {
                p_type: Default::default(),
                p_flags: Default::default(),
                p_offset: Default::default(),
                p_vaddr: Default::default(),
                p_paddr: Default::default(),
                p_filesz: Default::default(),
                p_memsz: Default::default(),
                p_align: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            Self::from_le(tmp)
        }
        fn from_ne_bytes(src: [u8; std::mem::size_of::<Phdr>()]) -> Self {
            let mut tmp: Self = Self {
                p_type: Default::default(),
                p_flags: Default::default(),
                p_offset: Default::default(),
                p_vaddr: Default::default(),
                p_paddr: Default::default(),
                p_filesz: Default::default(),
                p_memsz: Default::default(),
                p_align: Default::default(),
            };
            tmp.read_from_slice(src.as_ref());
            tmp
        }
    }
}
pub mod interface {
    use std::ops::RangeInclusive;
    pub trait MetaData<T: AsRef<[u8]> + Sized>: Sized {
        fn as_slice<'a>(&'a self) -> &'a [u8] {
            unsafe {
                std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
            }
        }
        fn as_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
            unsafe {
                std::slice::from_raw_parts_mut(
                    self as *mut Self as *mut u8,
                    std::mem::size_of::<Self>(),
                )
            }
        }
        fn read_from_slice(&mut self, src: &[u8]) {
            self.as_mut_slice().copy_from_slice(src);
        }
        fn to_be_bytes(self) -> T {
            ::core::panicking::panic("not yet implemented")
        }
        fn to_le_bytes(self) -> T {
            ::core::panicking::panic("not yet implemented")
        }
        fn to_ne_bytes(self) -> T {
            ::core::panicking::panic("not yet implemented")
        }
        fn from_be_bytes(_: T) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn from_le_bytes(_: T) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn from_ne_bytes(_: T) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn from_be(_: Self) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn from_le(_: Self) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn to_be(self) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn to_le(self) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn len() -> usize {
            std::mem::size_of::<Self>()
        }
    }
    pub trait Ehdr {
        fn shdr_table_range(&self) -> RangeInclusive<usize>;
        fn phdr_table_range(&self) -> RangeInclusive<usize>;
    }
    pub trait Section {
        fn name(&self) -> String;
    }
    pub trait Shdr {
        fn data_range(&self) -> RangeInclusive<usize>;
        fn entsize(&self) -> usize;
    }
    pub trait Elf {
        fn sections<T: Section>(&self) -> Vec<T>;
        fn programs(&self);
    }
}
pub struct EndWrapper<'a, T> {
    pub src: &'a T,
    pub endiness: Encode,
}
pub struct EndWrapperMut<'a, T> {
    pub src: &'a mut T,
    pub endiness: Encode,
}
pub struct Wrapper<'a, T> {
    pub src: &'a T,
}
pub struct WrapperMut<'a, T> {
    pub src: &'a mut T,
}
