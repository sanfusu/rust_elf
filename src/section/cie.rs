#[repr(C)]
pub struct Cie {
    pub len: u32,
    pub id: u32,
    pub ver: u8,
}

impl_borrow!(Cie);
