mod ident {
    use flassor::ByteOrder;
    pub struct Ident {
        pub mag: [u8; 4],
        pub file_class: u8,
        pub data_encode: u8,
        pub file_version: u8,
        pub osabi: u8,
        pub abi_version: u8,
    }
    impl Into<[u8; Ident::plain_size]> for Ident {
        fn into(self) -> [u8; Ident::plain_size] {
            let mut ret: [u8; Ident::plain_size] = [0; Ident::plain_size];
            ret.get_mut(
                <flat_accessor::fields::mag as flat_accessor::fields::IdentFields>::layout_range(),
            )
            .unwrap()
            .copy_from_slice(&self.mag.to_ne_bytes());
            ret . get_mut (< flat_accessor :: fields :: file_class as flat_accessor :: fields :: IdentFields > :: layout_range ()) . unwrap () . copy_from_slice (& self . file_class . to_ne_bytes ()) ;
            ret . get_mut (< flat_accessor :: fields :: data_encode as flat_accessor :: fields :: IdentFields > :: layout_range ()) . unwrap () . copy_from_slice (& self . data_encode . to_ne_bytes ()) ;
            ret . get_mut (< flat_accessor :: fields :: file_version as flat_accessor :: fields :: IdentFields > :: layout_range ()) . unwrap () . copy_from_slice (& self . file_version . to_ne_bytes ()) ;
            ret.get_mut(
                <flat_accessor::fields::osabi as flat_accessor::fields::IdentFields>::layout_range(
                ),
            )
            .unwrap()
            .copy_from_slice(&self.osabi.to_ne_bytes());
            ret . get_mut (< flat_accessor :: fields :: abi_version as flat_accessor :: fields :: IdentFields > :: layout_range ()) . unwrap () . copy_from_slice (& self . abi_version . to_ne_bytes ()) ;
            ret
        }
    }
    impl Ident {
        #[allow(non_upper_case_globals)]
        pub const plain_size: usize = 0
            + core::mem::size_of::<[u8; 4]>()
            + core::mem::size_of::<u8>()
            + core::mem::size_of::<u8>()
            + core::mem::size_of::<u8>()
            + core::mem::size_of::<u8>()
            + core::mem::size_of::<u8>();
        pub fn flat<'a, End: crate::flassor::Endianess<'a>>(
            raw: &'a [u8; Ident::plain_size],
        ) -> flat_accessor::IdentFlat<'a, End> {
            flat_accessor::IdentFlat::<'a, End>::from_raw(raw)
        }
        pub fn flat_mut<'a, End: crate::flassor::Endianess<'a>>(
            raw: &'a mut [u8; Ident::plain_size],
        ) -> flat_accessor::IdentFlatMut<'a, End> {
            flat_accessor::IdentFlatMut::<'a, End>::from_raw(raw)
        }
    }
    impl<'a> flassor::ByteOrder<'a> for Ident {
        type Bytes = [u8; Ident::plain_size];
        fn to_ne_bytes(self) -> [u8; Ident::plain_size] {
            self.into()
        }
        fn to_le_bytes(self) -> [u8; Ident::plain_size] {
            let ret = Ident {
                mag: <[u8; 4]>::to_le(self.mag),
                file_class: u8::to_le(self.file_class),
                data_encode: u8::to_le(self.data_encode),
                file_version: u8::to_le(self.file_version),
                osabi: u8::to_le(self.osabi),
                abi_version: u8::to_le(self.abi_version),
            };
            ret.into()
        }
        fn to_be_bytes(self) -> [u8; Ident::plain_size] {
            let ret = Ident {
                mag: <[u8; 4]>::to_be(self.mag),
                file_class: u8::to_be(self.file_class),
                data_encode: u8::to_be(self.data_encode),
                file_version: u8::to_be(self.file_version),
                osabi: u8::to_be(self.osabi),
                abi_version: u8::to_be(self.abi_version),
            };
            ret.into()
        }
        fn from_le_bytes(x: Self::Bytes) -> Self {
            let ret = Ident::flat::<flassor::Le>(&x);
            ret.to_meta()
        }
        fn from_be_bytes(x: Self::Bytes) -> Self {
            let ret = Ident::flat::<flassor::Be>(&x);
            ret.to_meta()
        }
        fn from_ne_bytes(x: Self::Bytes) -> Self {
            let ret = Ident::flat::<flassor::Ne>(&x);
            ret.to_meta()
        }
        fn from_be(x: Self) -> Self {
            Self {
                mag: <[u8; 4]>::from_be(x.mag),
                file_class: u8::from_be(x.file_class),
                data_encode: u8::from_be(x.data_encode),
                file_version: u8::from_be(x.file_version),
                osabi: u8::from_be(x.osabi),
                abi_version: u8::from_be(x.abi_version),
            }
        }
        fn from_le(x: Self) -> Self {
            Self {
                mag: <[u8; 4]>::from_le(x.mag),
                file_class: u8::from_le(x.file_class),
                data_encode: u8::from_le(x.data_encode),
                file_version: u8::from_le(x.file_version),
                osabi: u8::from_le(x.osabi),
                abi_version: u8::from_le(x.abi_version),
            }
        }
        fn to_be(self) -> Self {
            Self {
                mag: <[u8; 4]>::to_be(self.mag),
                file_class: u8::to_be(self.file_class),
                data_encode: u8::to_be(self.data_encode),
                file_version: u8::to_be(self.file_version),
                osabi: u8::to_be(self.osabi),
                abi_version: u8::to_be(self.abi_version),
            }
        }
        fn to_le(self) -> Self {
            Self {
                mag: <[u8; 4]>::to_le(self.mag),
                file_class: u8::to_le(self.file_class),
                data_encode: u8::to_le(self.data_encode),
                file_version: u8::to_le(self.file_version),
                osabi: u8::to_le(self.osabi),
                abi_version: u8::to_le(self.abi_version),
            }
        }
    }
    mod flat_accessor {
        use super::Ident;
        use core::{
            borrow::Borrow,
            convert::{AsRef, TryInto},
        };
        use flassor::{ByteOrder, Endianess};
        pub struct IdentFlat<'a, End: Endianess<'a>> {
            raw: &'a [u8; Ident::plain_size],
            phantom: core::marker::PhantomData<End>,
        }

        #[allow(unused_qualifications)]
        impl<'a, End: ::core::fmt::Debug + Endianess<'a>> ::core::fmt::Debug for IdentFlat<'a, End> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    IdentFlat {
                        raw: ref __self_0_0,
                        phantom: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "IdentFlat");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "raw",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "phantom",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<'a, End: Endianess<'a>> IdentFlat<'a, End> {
            /// raw_from means we didn't check the internal value.
            pub fn from_raw(raw: &'a [u8; Ident::plain_size]) -> Self {
                Self {
                    raw,
                    phantom: core::marker::PhantomData,
                }
            }
            pub fn raw(&self) -> &'a [u8; Ident::plain_size] {
                self.raw
            }
            pub fn get<T: fields::IdentFields + ByteOrder<'a>>(&self) -> T {
                End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
            }
            pub fn to_meta(&'a self) -> Ident {
                Ident {
                    mag: self.get::<fields::mag>().raw(),
                    file_class: self.get::<fields::file_class>().raw(),
                    data_encode: self.get::<fields::data_encode>().raw(),
                    file_version: self.get::<fields::file_version>().raw(),
                    osabi: self.get::<fields::osabi>().raw(),
                    abi_version: self.get::<fields::abi_version>().raw(),
                }
            }
        }
        impl<'a, End: Endianess<'a>> AsRef<[u8]> for IdentFlat<'a, End> {
            fn as_ref(&self) -> &[u8] {
                self.raw
            }
        }
        pub struct IdentFlatMut<'a, End: Endianess<'a>> {
            raw: &'a mut [u8; Ident::plain_size],
            phantom: core::marker::PhantomData<End>,
        }
        #[allow(unused_qualifications)]
        impl<'a, End: ::core::fmt::Debug + Endianess<'a>> ::core::fmt::Debug for IdentFlatMut<'a, End> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    IdentFlatMut {
                        raw: ref __self_0_0,
                        phantom: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "IdentFlatMut");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "raw",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "phantom",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<'a, End: Endianess<'a>> IdentFlatMut<'a, End> {
            pub fn new(raw: &'a mut [u8; Ident::plain_size]) -> Self {
                Self {
                    raw,
                    phantom: core::marker::PhantomData,
                }
            }
            /// from_raw means we didn't check the internal value.
            pub fn from_raw(raw: &'a mut [u8; Ident::plain_size]) -> Self {
                Self {
                    raw,
                    phantom: core::marker::PhantomData,
                }
            }
            pub fn raw_mut(&'a mut self) -> &'a mut [u8; Ident::plain_size] {
                self.raw
            }
            pub fn raw(&'a self) -> &'a [u8; Ident::plain_size] {
                self.raw
            }
            fn as_mut(&mut self) -> &mut [u8] {
                self.raw
            }
            pub fn get<T: fields::IdentFields + ByteOrder<'a>>(&'a self) -> T {
                End::from_bytes(self.raw.get(T::layout_range()).unwrap().try_into().unwrap())
            }
            pub fn set<T: fields::IdentFields + ByteOrder<'a>>(
                &'a mut self,
                value: T,
            ) -> &'a mut IdentFlatMut<'a, End> {
                self.raw[T::layout_range()].copy_from_slice(End::bytes_from(value).borrow());
                self
            }
            pub fn to_meta(&'a self) -> Ident {
                Ident {
                    mag: self.get::<fields::mag>().raw(),
                    file_class: self.get::<fields::file_class>().raw(),
                    data_encode: self.get::<fields::data_encode>().raw(),
                    file_version: self.get::<fields::file_version>().raw(),
                    osabi: self.get::<fields::osabi>().raw(),
                    abi_version: self.get::<fields::abi_version>().raw(),
                }
            }
        }
        impl<'a, End: Endianess<'a>> AsRef<[u8]> for IdentFlatMut<'a, End> {
            fn as_ref(&self) -> &[u8] {
                self.raw
            }
        }
        pub mod fields {

            use flassor::ByteOrder;
            pub trait IdentFields {
                fn layout_range() -> core::ops::Range<usize>;
            }
            #[allow(non_camel_case_types)]
            pub struct mag {
                value: [u8; 4],
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for mag {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        mag {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "mag");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for mag {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    0..core::mem::size_of::<[u8; 4]>()
                }
            }
            impl mag {
                pub fn new(value: [u8; 4]) -> mag {
                    mag { value }
                }
                pub fn raw(&self) -> [u8; 4] {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for mag {
                type Bytes = [u8; core::mem::size_of::<[u8; 4]>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<[u8; 4]>()] {
                    <[u8; 4]>::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<[u8; 4]>()] {
                    <[u8; 4]>::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<[u8; 4]>()] {
                    <[u8; 4]>::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: <[u8; 4]>::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: <[u8; 4]>::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: <[u8; 4]>::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: <[u8; 4]>::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: <[u8; 4]>::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: <[u8; 4]>::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: <[u8; 4]>::to_le(self.value),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            pub struct file_class {
                value: u8,
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for file_class {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        file_class {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "file_class");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for file_class {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    <mag>::layout_range().end
                        ..<mag>::layout_range().end + core::mem::size_of::<u8>()
                }
            }
            impl file_class {
                pub fn new(value: u8) -> file_class {
                    file_class { value }
                }
                pub fn raw(&self) -> u8 {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for file_class {
                type Bytes = [u8; core::mem::size_of::<u8>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: u8::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: u8::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: u8::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: u8::to_le(self.value),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            pub struct data_encode {
                value: u8,
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for data_encode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        data_encode {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "data_encode");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for data_encode {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    <file_class>::layout_range().end
                        ..<file_class>::layout_range().end + core::mem::size_of::<u8>()
                }
            }
            impl data_encode {
                pub fn new(value: u8) -> data_encode {
                    data_encode { value }
                }
                pub fn raw(&self) -> u8 {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for data_encode {
                type Bytes = [u8; core::mem::size_of::<u8>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: u8::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: u8::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: u8::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: u8::to_le(self.value),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            pub struct file_version {
                value: u8,
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for file_version {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        file_version {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "file_version");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for file_version {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    <data_encode>::layout_range().end
                        ..<data_encode>::layout_range().end + core::mem::size_of::<u8>()
                }
            }
            impl file_version {
                pub fn new(value: u8) -> file_version {
                    file_version { value }
                }
                pub fn raw(&self) -> u8 {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for file_version {
                type Bytes = [u8; core::mem::size_of::<u8>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: u8::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: u8::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: u8::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: u8::to_le(self.value),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            pub struct osabi {
                value: u8,
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for osabi {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        osabi {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "osabi");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for osabi {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    <file_version>::layout_range().end
                        ..<file_version>::layout_range().end + core::mem::size_of::<u8>()
                }
            }
            impl osabi {
                pub fn new(value: u8) -> osabi {
                    osabi { value }
                }
                pub fn raw(&self) -> u8 {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for osabi {
                type Bytes = [u8; core::mem::size_of::<u8>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: u8::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: u8::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: u8::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: u8::to_le(self.value),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            pub struct abi_version {
                value: u8,
            }

            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for abi_version {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        abi_version {
                            value: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "abi_version");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl IdentFields for abi_version {
                #[inline]
                fn layout_range() -> core::ops::Range<usize> {
                    <osabi>::layout_range().end
                        ..<osabi>::layout_range().end + core::mem::size_of::<u8>()
                }
            }
            impl abi_version {
                pub fn new(value: u8) -> abi_version {
                    abi_version { value }
                }
                pub fn raw(&self) -> u8 {
                    self.value
                }
            }
            impl<'a> ByteOrder<'a> for abi_version {
                type Bytes = [u8; core::mem::size_of::<u8>()];
                fn to_ne_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_ne_bytes(self.value)
                }
                fn to_le_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_le_bytes(self.value)
                }
                fn to_be_bytes(self) -> [u8; core::mem::size_of::<u8>()] {
                    u8::to_be_bytes(self.value)
                }
                fn from_le_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_le_bytes(x),
                    }
                }
                fn from_be_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_be_bytes(x),
                    }
                }
                fn from_ne_bytes(x: Self::Bytes) -> Self {
                    Self {
                        value: u8::from_ne_bytes(x),
                    }
                }
                fn from_be(x: Self) -> Self {
                    Self {
                        value: u8::from_be(x.value),
                    }
                }
                fn from_le(x: Self) -> Self {
                    Self {
                        value: u8::from_le(x.value),
                    }
                }
                fn to_be(self) -> Self {
                    Self {
                        value: u8::to_be(self.value),
                    }
                }
                fn to_le(self) -> Self {
                    Self {
                        value: u8::to_le(self.value),
                    }
                }
            }
        }
    }

    #[allow(unused_qualifications)]
    impl ::core::default::Default for Ident {
        #[inline]
        fn default() -> Ident {
            Ident {
                mag: ::core::default::Default::default(),
                file_class: ::core::default::Default::default(),
                data_encode: ::core::default::Default::default(),
                file_version: ::core::default::Default::default(),
                osabi: ::core::default::Default::default(),
                abi_version: ::core::default::Default::default(),
            }
        }
    }
}
