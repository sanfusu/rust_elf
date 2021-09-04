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
#![no_std]
#![allow(dead_code)]

macro_rules! impl_borrow {
    ($($Type:ty),+) => {
        $(impl core::borrow::Borrow<[u8; core::mem::size_of::<$Type>()]> for $Type {
            fn borrow(&self) -> &[u8; core::mem::size_of::<$Type>()] {
                use core::{convert::TryInto, ptr::slice_from_raw_parts};
                unsafe {
                    &*slice_from_raw_parts(
                        self as *const $Type as *const u8,
                        core::mem::size_of::<$Type>(),
                    )
                }
                .try_into()
                .unwrap() // 不会 panic，因为长度一致。
            }
        })+
    };
}

pub mod header;
pub mod program;
pub mod section;

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64Sword = u32;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;
