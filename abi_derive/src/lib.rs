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
    let enum_name_ident = ast.ident;
    let (variant, mut discriminant): (Vec<_>, Vec<_>) = match ast.data {
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
    for v in discriminant.iter_mut() {
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
    let enum_name_ident_str =
        syn::LitStr::new(&enum_name_ident.to_string(), enum_name_ident.span());

    let enum_path = vec![enum_name_ident.clone(); variant.len()];
    let gen = quote! {
        impl TryFrom<u8> for #enum_name_ident {
            type Error = String;
            fn try_from(v: u8) -> Result<Self, Self::Error> {
                #(if v == #discriminant {return Ok(#enum_path::#variant);})*
                return Err(format!("Invalid discriminant for enum {}, the value is {}", #enum_name_ident_str, v));
            }
        }
        impl TryFrom<u32> for #enum_name_ident {
            type Error = String;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                #(if v == #discriminant {return Ok(#enum_path::#variant);})*
                return Err(format!("Invalid discriminant for enum {}, the value is {}", #enum_name_ident_str, v));
            }
        }
    };
    // println!("{}", gen);
    gen.into()
}
