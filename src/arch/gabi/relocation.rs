pub mod rel {
    use derive::AsSlice;
    #[derive(AsSlice)]
    #[repr(C)]
    pub struct Entry<T: crate::IBasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
    }
}
pub mod rela {
    use derive::AsSlice;
    #[derive(AsSlice)]
    #[repr(C)]
    pub struct Entry<T: crate::IBasicType> {
        pub offset: T::Addr,
        pub info: T::Xword,
        pub addend: T::Sxword,
    }
}
