use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{parse2, parse_quote, Data, DeriveInput, Expr, Type};

#[proc_macro_derive(Layout)]
pub fn layout(input: TokenStream) -> TokenStream {
    struct_layout(input.into()).into()
}

fn struct_layout(input: TokenStream2) -> TokenStream2 {
    let ast: DeriveInput = parse2(input).unwrap();
    let ident = ast.ident;
    let layout_struct = format_ident!("{}Layout", ident);
    let mut layout_ident: Vec<Ident> = Vec::new();
    let mut layout_offset_after: Vec<Expr> = Vec::new();
    let mut layout_offset_prev: Vec<Expr> = Vec::new();
    layout_offset_prev.push(parse_quote! {0});

    if let Data::Struct(data) = ast.data {
        data.fields.iter().for_each(|x| {
            let ty: Type = x.ty.clone();
            let current_size: Expr = parse_quote! {std::mem::size_of::<#ty>()};
            let current_offset: Expr;

            match layout_offset_after.last() {
                Some(lo) => {
                    current_offset = parse_quote! { #lo + #current_size};
                }
                None => {
                    current_offset = parse_quote! {#current_size};
                }
            }
            layout_offset_after.push(current_offset);
            layout_ident.push(x.clone().ident.expect("Should be all named field"));
        })
    } else {
        panic!("We need struct");
    }

    layout_offset_after
        .iter()
        .for_each(|x| layout_offset_prev.push(x.clone()));
    layout_offset_prev.pop();
    let usage_msg = format!(
        "用于获取 {} 每个字段的内存布局，如 `{}::{}();`",
        ident.to_string(),
        layout_struct.to_string(),
        layout_ident[0].to_string()
    );
    quote! {
        #[doc=#usage_msg]
        pub struct #layout_struct;
        impl #layout_struct {
            #(pub const fn #layout_ident()->std::ops::Range<usize> { #layout_offset_prev..#layout_offset_after})*
        }
    }
}
