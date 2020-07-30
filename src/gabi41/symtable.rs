macro_rules! SET_TYPE_VALUE {
    ($info:expr, $ty:expr) => {
        ($info & 0xf) | $ty
    };
}
macro_rules! GET_BIND_VALUE {
    ($info:expr) => {
        $info >> 4
    };
}
macro_rules! GET_TYPE_VALUE {
    ($info:expr) => {
        $info & 0xf
    };
}
macro_rules! SET_BIND_VALUE {
    ($info:expr, $bind:expr) => {
        ($bind << 4) | GET_TYPE_VALUE!($info)
    };
}

pub mod st_bind {

    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
    #[derive(Ordinalize, Debug)]
    #[repr(u8)]
    #[non_exhaustive]
    pub enum Normal {
        LOCAL = 0,
        GLOABL,
        WEAK,
    }
    #[non_exhaustive]
    pub enum Bind {
        Bind(Normal),
        ProcessorSpecific(u8),
        Unknown(u8),
    }
    // use super::GET_BIND_VALUE;
    pub fn get_bind(info_v: u8) -> Bind {
        let info = GET_BIND_VALUE!(info_v);
        Normal::from_ordinal(info).map_or_else(
            || {
                if info >= LOPROC as u8 && info <= HIPROC as u8 {
                    Bind::ProcessorSpecific(info)
                } else {
                    Bind::Unknown(info)
                }
            },
            |v| Bind::Bind(v),
        )
    }
    pub fn set_bind(info: u8, bind: Bind) -> u8 {
        match bind {
            Bind::Bind(v) => SET_BIND_VALUE!(info, v as u8),
            Bind::ProcessorSpecific(v) | Bind::Unknown(v) => SET_BIND_VALUE!(info, v),
        }
    }
}

pub mod st_type {
    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
    #[derive(Ordinalize, Debug)]
    #[repr(u8)]
    #[non_exhaustive]
    pub enum Normal {
        NOTYPE = 0,
        OBJECT = 1,
        FUNC = 2,
        SECTION = 3,
        FILE = 4,
    }
    #[non_exhaustive]
    pub enum Type {
        Type(Normal),
        ProcessorSpecific(u8),
        Unknown(u8),
    }
    pub fn set_type(info: u8, ty: Type) -> u8 {
        match ty {
            Type::Type(v) => SET_TYPE_VALUE!(info, v as u8),
            Type::ProcessorSpecific(v) | Type::Unknown(v) => SET_TYPE_VALUE!(info, v),
        }
    }
    pub fn get_type(info_v: u8) -> Type {
        let ty = GET_TYPE_VALUE!(info_v);
        Normal::from_ordinal(ty).map_or_else(
            || {
                if ty >= LOPROC as u8 && ty <= HIPROC as u8 {
                    Type::ProcessorSpecific(ty)
                } else {
                    Type::Unknown(ty)
                }
            },
            |v| Type::Type(v),
        )
    }
}
#[macro_export]
macro_rules! sym_info_accessor {
    ($Sym:ty) => {
        impl $Sym {
            pub fn get_bind(&self) -> $crate::gabi41::symtable::st_bind::Bind {
                $crate::gabi41::symtable::st_bind::get_bind(self.info)
            }
            pub fn get_type(&self) -> $crate::gabi41::symtable::st_type::Type {
                $crate::gabi41::symtable::st_type::get_type(self.info)
            }
            pub fn set_bind(&mut self, bind: $crate::gabi41::symtable::st_bind::Bind) -> &Self {
                self.info = $crate::gabi41::symtable::st_bind::set_bind(self.info, bind);
                self
            }
            pub fn set_type(&mut self, ty: $crate::gabi41::symtable::st_type::Type) -> &Self {
                self.info = $crate::gabi41::symtable::st_type::set_type(self.info, ty);
                self
            }
            pub fn set_info(
                &mut self,
                b: $crate::gabi41::symtable::st_bind::Bind,
                t: $crate::gabi41::symtable::st_type::Type,
            ) {
                self.set_bind(b);
                self.set_type(t);
            }
            pub fn get_info(
                &self,
            ) -> (
                $crate::gabi41::symtable::st_bind::Bind,
                $crate::gabi41::symtable::st_type::Type,
            ) {
                (self.get_bind(), self.get_type())
            }
        }
    };
}
