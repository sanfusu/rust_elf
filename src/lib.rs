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
#[macro_use]
extern crate elface;
#[macro_use]
extern crate layout;

use core::ops::Index;
use elf32::ehdr::ident::encode::Encode;

macro_rules! define_transparent_meta_data {
    ($StructVis:vis $Struct:ident, $Vt:ty {
        $(Wellknown: {
            $($WellknownVis:vis $Wellknown:ident = $value:literal)*
        })?
        ValidRange: {
            $($ValidRangeVis:vis $valid_range:ident = $range:expr)+
        }
        $(Default: $defaultValue:expr)?
    }) => {
        #[repr(transparent)]
        #[derive(MetaData, PartialOrd, PartialEq)]
        $StructVis struct $Struct {
            value: $Vt
        }

        impl $Struct {
            const fn new(value: $Vt) -> Self {
                Self { value }
            }
        }
        $($($WellknownVis const $Wellknown:$Struct = $Struct::new($value);)*)?
        $($ValidRangeVis const $valid_range:core::ops::RangeInclusive<$Struct> = $range;)+
        impl core::convert::TryFrom<$Vt> for $Struct {
            type Error = &'static str;
            fn try_from(value:$Vt)->Result<Self,Self::Error>{
                let ret = $Struct::new(value);

                if $($valid_range.contains(&ret))||+ {
                    Ok(ret)
                } else {
                    Err("Out of range")
                }
            }
        }
        $(
            impl core::default::Default for $Struct {
                fn default() -> Self {
                    $defaultValue
                }
            }
        )?
    };
}

pub mod elf32;
pub mod elf64;

pub struct EndWrapper<'a, T> {
    pub src: &'a T,
    pub endiness: Encode,
}

pub struct EndWrapperMut<'a, T> {
    pub src: &'a mut T,
    pub endiness: Encode,
}

pub struct Wrapper<'a, T> {
    pub src: &'a T,
}
pub struct WrapperMut<'a, T> {
    pub src: &'a mut T,
}

/// 直接通过索引来获取字符串表中的数值
/// # Example
/// ```
/// use elf::StrTab;
///
/// let tmp = ['a' as u8,'b' as u8,'c' as u8,'\0' as u8];
/// let str_tab = StrTab::new(&tmp);
/// assert_eq!(&str_tab[0], "abc");
/// assert_eq!(&str_tab[1], "bc");
/// assert_eq!(&str_tab[3], "");
///
/// let tmp_witout_null = ['a' as u8,'b' as u8,'c' as u8];
/// let str_tab_without_null = StrTab::new(&tmp_witout_null);
/// assert_eq!(&str_tab_without_null[0], "abc");
/// assert_eq!(&str_tab_without_null[1], "bc");
/// assert_eq!(&str_tab_without_null[3], "");
///
/// let tmp_split_null = ['a' as u8,'\0' as u8,'c' as u8];
/// let str_tab_split_null = StrTab::new(&tmp_split_null);
/// assert_eq!(&str_tab_split_null[0], "a");
/// assert_eq!(&str_tab_split_null[1], "");
/// assert_eq!(&str_tab_split_null[3], "");
/// ```
pub struct StrTab<'a> {
    src: &'a [u8],
}

impl<'a> StrTab<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self { src }
    }
}

impl Index<usize> for StrTab<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        let src = &self.src[index..];
        let end = src.iter().position(|&x| x == 0).unwrap_or(src.len());
        unsafe { core::str::from_utf8_unchecked(&src[..end]) }
    }
}
