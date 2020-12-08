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

pub(crate) fn ehdr_proc(input: TokenStream2) -> TokenStream2 {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = ast.ident;
    let shoff: proc_macro2::Ident;
    let shnum: proc_macro2::Ident;
    let shentsize: proc_macro2::Ident;
    let phoff: proc_macro2::Ident;
    let phnum: proc_macro2::Ident;
    let phentsize: proc_macro2::Ident;
    if let syn::Data::Struct(data) = ast.data {
        shoff = ehdr_find_attr_ident(&data, "shoff");
        shnum = ehdr_find_attr_ident(&data, "shnum");
        shentsize = ehdr_find_attr_ident(&data, "shentsize");
        phoff = ehdr_find_attr_ident(&data, "phoff");
        phnum = ehdr_find_attr_ident(&data, "phnum");
        phentsize = ehdr_find_attr_ident(&data, "phentsize");
    } else {
        panic! {"Should be struct"}
    }
    quote! {
        impl crate::interface::Ehdr for #name {
            fn shdr_table_range(&self)->std::ops::Range<usize>{
                (self.#shoff as usize) .. (self.#shnum as usize) * (self.#shentsize as usize) + (self.#shoff as usize)
            }
            fn phdr_table_range(&self)->std::ops::Range<usize> {
                (self.#phoff as usize) .. (self.#phnum as usize) * (self.#phentsize as usize) + (self.#phoff as usize)

            }
        }
    }
}

pub(crate) fn ehdr_find_attr_ident(data: &DataStruct, ident: &str) -> proc_macro2::Ident {
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
