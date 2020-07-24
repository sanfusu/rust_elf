use super::{
    elf32::{ehdr::Ehdr as Ehdr32, shdr::Shdr as Shdr32, symtable::Sym as Sym32},
    elf64::{ehdr::Ehdr as Ehdr64, shdr::Shdr as Shdr64, symtable::Sym as Sym64},
};
pub enum Shdr {
    Header32(Shdr32),
    Header64(Shdr64),
}

pub enum Ehdr {
    Header64(Ehdr64),
    Header32(Ehdr32),
}

impl Default for Ehdr {
    fn default() -> Self {
        Ehdr::Header64(Ehdr64 {
            ..Default::default()
        })
    }
}

pub enum Sym {
    Symbol32(Sym32),
    Symbol64(Sym64),
}

#[cfg(test)]
mod test {
    #![allow(unused_variables)]
    use super::{Ehdr, Ehdr64};
    #[test]
    fn default_value_test() -> Result<(), String> {
        let a = Ehdr::Header64(Ehdr64 {
            ..Default::default()
        });
        let b = match a {
            Ehdr::Header64(v) => v,
            _ => Ehdr64 {
                ..Default::default()
            },
        };
        Ok(())
    }
}
