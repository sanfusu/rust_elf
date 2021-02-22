// Copyright (C) 2020 sanfusu@foxmail.com
//
// This file is part of rust_elf.
//
// rust_elf is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rust_elf is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rust_elf.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::DeriveInput;

pub(crate) fn metadata_proc(input: TokenStream2) -> TokenStream2 {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = ast.ident;
    let mut fields: Vec<syn::Ident> = Vec::new();
    let mut fields_other: Vec<syn::Ident> = Vec::new();
    let mut fields_type: Vec<TokenStream2> = Vec::new();
    if let syn::Data::Struct(data) = ast.data {
        data.fields.iter().for_each(|x| match x.to_owned().ty {
            syn::Type::Array(arr) => match *arr.elem {
                syn::Type::Path(ty) => {
                    if ty.to_token_stream().to_string() == "u8" {
                        fields_other.push(x.to_owned().ident.expect("need named field"));
                    }
                }
                _ => {
                    panic!("Currently can only support u8 array")
                }
            },
            syn::Type::Group(ty) => {
                fields.push(x.to_owned().ident.expect("need named field"));
                fields_type.push(ty.to_token_stream());
            }
            syn::Type::Path(ty) => {
                fields.push(x.to_owned().ident.expect("need named field"));
                fields_type.push(ty.to_token_stream());
            }
            _ => {
                panic!("Unsupport type")
            }
        })
    } else {
        panic!("can only be applied into struct")
    }

    quote! {
        impl elface::MetaData<[u8;core::mem::size_of::<#name>()]> for #name{
            fn to_le(self) -> Self {
                Self{
                    #(#fields: self.#fields.to_le(),)*
                    #(#fields_other: self.#fields_other,)*
                }
            }
            fn to_be(self) -> Self {
                Self{
                    #(#fields: self.#fields.to_be(),)*
                    #(#fields_other: self.#fields_other,)*
                }
            }
            fn from_le(value: Self) -> Self {
                Self{
                    #(#fields: #fields_type::from_le(value.#fields),)*
                    #(#fields_other: value.#fields_other,)*
                }
            }
            fn from_be(value: Self) -> Self {
                Self{
                    #(#fields: #fields_type::from_be(value.#fields),)*
                    #(#fields_other: value.#fields_other,)*
                }
            }
            fn to_be_bytes(self) -> [u8;core::mem::size_of::<#name>()] {
                let tmp = self.to_be();
                let mut ret: [u8;core::mem::size_of::<#name>()] = [0;core::mem::size_of::<#name>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_le_bytes(self) -> [u8;core::mem::size_of::<#name>()] {
                let tmp = self.to_le();
                let mut ret: [u8;core::mem::size_of::<#name>()] = [0;core::mem::size_of::<#name>()];
                (&mut ret[..]).copy_from_slice(tmp.as_slice());
                ret
            }
            fn to_ne_bytes(self) -> [u8;core::mem::size_of::<#name>()] {
                let mut ret: [u8;core::mem::size_of::<#name>()] = [0;core::mem::size_of::<#name>()];
                (&mut ret[..]).copy_from_slice(self.as_slice());
                ret
            }
            unsafe fn from_be_bytes(src: [u8;core::mem::size_of::<#name>()]) -> Self {
                let mut tmp : Self = Self {
                    #(#fields: Default::default(),)*
                    #(#fields_other: Default::default(),)*
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_be(tmp)
            }

            unsafe fn from_le_bytes(src: [u8;core::mem::size_of::<#name>()]) -> Self {
                let mut tmp : Self = Self {
                    #(#fields: Default::default(),)*
                    #(#fields_other: Default::default(),)*
                };
                tmp.read_from_slice(src.as_ref());
                Self::from_le(tmp)
            }
            fn from_ne_bytes(src: [u8;core::mem::size_of::<#name>()]) -> Self {
                let mut tmp : Self = Self {
                    #(#fields: Default::default(),)*
                    #(#fields_other: Default::default(),)*
                };
                tmp.read_from_slice(src.as_ref());
                tmp
            }
        }
    }
}
