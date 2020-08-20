pub(crate) mod p_type {
    #[macro_export]
    macro_rules! define_p_type_basic_const {
        ($elf:ty) => {
            pub const NULL: $elf = 0;
            pub const LOAD: $elf = 1;
            pub const DYNAMIC: $elf = 2;
            pub const INTERP: $elf = 3;
            pub const NOTE: $elf = 4;
            pub const SHLIB: $elf = 5;
            pub const PHDR: $elf = 6;
            pub const LOPROC: $elf = 0x7000_0000;
            pub const HIPROC: $elf = 0x7fff_ffff;
        };
    }
}

pub(crate) mod p_flags {
    #[macro_export]
    macro_rules! define_p_flags_basic_const {
        ($elf:ty) => {
            pub const X: $elf = 0x1;
            pub const W: $elf = 0x2;
            pub const R: $elf = 0x4;
            pub const MASKPROC: $elf = 0xf000_0000;
        };
    }
}
