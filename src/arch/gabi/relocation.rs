pub mod rel {
    #[repr(C)]
    pub struct Entry<T: crate::IBasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
    }
}
pub mod rela {
    #[repr(C)]
    pub struct Entry<T: crate::IBasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
        pub addend: T::Sxword,
    }
}
