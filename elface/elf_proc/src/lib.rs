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

use proc_macro::TokenStream;

pub(crate) mod ehdr;
pub(crate) mod metadata;
pub(crate) mod shdr;

#[proc_macro_derive(MetaData)]
pub fn metadata_proc(input: TokenStream) -> TokenStream {
    metadata::metadata_proc(input.into()).into()
}

#[proc_macro_derive(Ehdr, attributes(shoff, shentsize, phoff, shnum, phnum, phentsize))]
pub fn ehdr_proc(input: TokenStream) -> TokenStream {
    ehdr::ehdr_proc(input.into()).into()
}

#[proc_macro_derive(Shdr, attributes(offset, entsize, size, name_idx))]
pub fn shdr_proc(input: TokenStream) -> TokenStream {
    shdr::shdr_proc(input.into()).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
