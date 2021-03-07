use crate::Word;

define_transparent_meta_data! {
    pub ShType, Word {
        Wellknown: {
            pub SHT_NULL = 0
            pub SHT_PROGBITS = 1
            pub SHT_SYMTAB = 2
            pub SHT_STRTAB = 3
            pub SHT_RELA = 4
            pub SHT_HASH = 5
            pub SHT_DYNAMIC = 6
            pub SHT_NOTE = 7
            pub SHT_NOBITS = 8
            pub SHT_REL = 9
            pub SHT_SHLIB = 10
            pub SHT_DYNSYM = 11
            pub SHT_LOOS = 0x60000000
            pub SHT_HIOS = 0x6FFFFFFF
            pub SHT_LOPROC = 0x70000000
            pub SHT_HIPROC = 0x7FFFFFFF
        }
        ValidRange : {
            SHT_RESERVED_RANGE = SHT_NULL..=SHT_DYNSYM
            SHT_OS_RANGE = SHT_LOOS..=SHT_HIOS
            SHT_PROC_RANGE = SHT_LOPROC..=SHT_HIPROC
        }
        Default: SHT_NULL
    }
}
