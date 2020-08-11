use super::Elf;
pub use crate::arch::gabi::dynamic::{self, *};
pub type Dyn = dynamic::Dyn<Elf>;
