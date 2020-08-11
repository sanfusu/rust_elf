pub mod rel {
    pub struct Entry<T: crate::BasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
    }
}
pub mod rela {
    pub struct Entry<T: crate::BasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
        pub addend: T::Sxword,
    }
}
