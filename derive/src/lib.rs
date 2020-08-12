extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AsSlice)]
pub fn as_slice_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause{
            fn as_slice<'a>(&'a self)->&'a [u8]{
                unsafe {
                    std::slice::from_raw_parts(
                        self as *const Self as *const u8, std::mem::size_of::<Self>()
                    )
                }
            }
            fn as_mut_slice<'a>(&'a mut self)->&'a mut [u8]{
                unsafe {
                    std::slice::from_raw_parts_mut(
                        self as *mut Self as *mut u8, std::mem::size_of::<Self>()
                    )
                }
            }
        }
    };
    gen.into()
}
