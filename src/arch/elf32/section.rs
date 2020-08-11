pub use crate::arch::gabi::section::*;
use crate::arch::{elf32::Elf, gabi};

pub type Header = gabi::section::Header<Elf>;
