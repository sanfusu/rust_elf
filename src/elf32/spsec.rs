use super::{
    gabi41::spsec::{HashShdrGeneral, SpSecShdrGeneral},
    Elf,
};
use std::default::Default;

pub type SpSecShdr = SpSecShdrGeneral<Elf>;
pub type HashShdr = HashShdrGeneral<Elf>;

impl Default for HashShdr {
    fn default() -> Self {
        Self::new()
    }
}
