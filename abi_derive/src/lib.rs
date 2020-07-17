extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Abi32Elf)]
pub fn abi32_elf_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl ElfBasicType for #name {
            type Addr = u32;
            type Half = u16;
            type Off = u32;
            type Word = u32;
            type Sword = i32;
            type Xword=u32;
            type Sxword=u32;
        }
    };
    gen.into()
}

#[proc_macro_derive(Abi64Elf)]
pub fn abi64_elf_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl ElfBasicType for #name {
            type Addr = u64;
            type Half = u16;
            type Off = u64;
            type Word = u32;
            type Sword = i32;
            type Xword = u64;
            type Sxword = u64;
        }
    };
    gen.into()
}
use syn::{parse_quote, Expr};

#[proc_macro_derive(EnumFromPrimitive)]
pub fn enum_from_primitive(token: TokenStream) -> TokenStream {
    #![allow(unused_variables)]
    let ast: syn::DeriveInput = syn::parse(token).unwrap();
    let name = ast.ident;
    let (variant, mut disc): (Vec<_>, Vec<_>) = match ast.data {
        syn::Data::Enum(ref v) => v
            .variants
            .iter()
            .map(|x| {
                match x.fields {
                    syn::Fields::Unit => {}
                    _ => panic!("#[derivepEnumFromPrimitive]"),
                }
                let discrim = x
                    .discriminant
                    .clone()
                    .map(|(_eq, expr): (syn::token::Eq, Expr)| expr);
                (x.ident.clone(), discrim)
            })
            .unzip(),
        _ => panic!("EnumFromPrimitive must be used with enum"),
    };
    let mut base: Expr = parse_quote!(0);
    for v in disc.iter_mut() {
        match v {
            Some(x) => {
                base = x.clone();
                base = parse_quote! {#base+1};
            }
            None => {
                *v = Some(parse_quote! {#base});
                base = parse_quote! {#base +1};
            }
        }
    }
    let enum_u64 = vec![name.clone(); variant.len()];
    let gen = quote! {
        impl TryFrom<u8> for #name {
        type Error = String;
            fn try_from(v: u8) -> Result<Self, Self::Error> {
                #(if v == #disc {return Ok(#enum_u64::#variant);})*
                return Err(String::from("Invalid discriminant for enum"));
            }
        }
        impl TryFrom<u32> for #name {
        type Error = String;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                #(if v == #disc {return Ok(#enum_u64::#variant);})*
                return Err(String::from("Invalid discriminant for enum"));
            }
        }
    };
    gen.into()
}
