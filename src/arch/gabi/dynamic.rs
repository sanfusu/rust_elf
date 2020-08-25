pub union Du<T: crate::IBasicType> {
    pub val: T::Xword,
    pub ptr: T::Addr,
}

use derive::AsSlice;
#[derive(AsSlice)]
#[repr(C)]
pub struct Dyn<T: crate::IBasicType> {
    pub tag: T::Sxword,
    pub un: Du<T>,
}
pub(crate) mod d_tag {}
