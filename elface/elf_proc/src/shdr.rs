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
use quote::quote;
use syn::{DataStruct, DeriveInput};

pub(crate) fn shdr_proc(input: TokenStream2) -> TokenStream2 {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = ast.ident;
    let offset: proc_macro2::Ident;
    let entsize: proc_macro2::Ident;
    let size: proc_macro2::Ident;
    let name_idx: proc_macro2::Ident;

    if let syn::Data::Struct(data) = ast.data {
        offset = shdr_find_attr_ident(&data, "offset");
        entsize = shdr_find_attr_ident(&data, "entsize");
        size = shdr_find_attr_ident(&data, "size");
        name_idx = shdr_find_attr_ident(&data, "name_idx");
    } else {
        panic! {"Should be struct"}
    }
    quote! {
        impl elface::SecHeader for #name {
            fn data_range(&self)->core::ops::RangeInclusive<usize>{
                (self.#offset as usize) ..= self.#size as usize
            }
            fn entsize(&self)->usize{
                self.#entsize as usize
            }
            fn name_idx(&self)->usize{
                self.#name_idx as usize
            }
        }
    }
}

pub(crate) fn shdr_find_attr_ident(data: &DataStruct, ident: &str) -> proc_macro2::Ident {
    data.fields
        .iter()
        .find(|field| {
            field
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident(ident))
                .is_some()
        })
        .expect(&format!("need #[{}]", ident))
        .to_owned()
        .ident
        .expect(&format!("need {}", ident))
}
