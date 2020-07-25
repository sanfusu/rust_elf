use super::{
    gabi41::spsec::{HashShdrGen, SpSecShdrGen},
    Elf,
};


pub type SpSecShdr = SpSecShdrGen<Elf>;
pub type HashShdr = HashShdrGen<Elf>;
