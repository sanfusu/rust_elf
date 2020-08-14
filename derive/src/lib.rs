extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AsSlice)]
pub fn as_slice_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut gen = quote! {
        impl #impl_generics crate::AsBytes for #name #ty_generics #where_clause {
            #[doc="类型转换"]
            #[inline]
            fn as_bytes<'a>(&'a self)->&'a [u8] {
                self.as_ref()
            }

            #[doc="类型转换"]
            #[inline]
            fn as_bytes_mut<'a>(&'a mut self)->&'a mut [u8] {
                self.as_mut()
            }
        }
    };

    let a: syn::GenericParam = syn::parse_quote! { AS_MUT_REF_TY };
    let mut impl_g = ast.generics.clone();
    &impl_g.params.push(a);
    let (impl_generics, _, where_clause) = impl_g.split_for_impl();
    gen.extend(quote! {
        impl #impl_generics std::convert::AsMut<[AS_MUT_REF_TY]> for #name #ty_generics #where_clause{
            #[doc="类型转换"]
            #[inline]
            fn as_mut(&mut self) -> &mut [AS_MUT_REF_TY] {
                unsafe {
                    std::slice::from_raw_parts_mut(
                        self as *mut Self as *mut AS_MUT_REF_TY,
                        std::mem::size_of::<Self>() / std::mem::size_of::<AS_MUT_REF_TY>(),
                    )
                }
            }
        }
        impl #impl_generics std::convert::AsRef<[AS_MUT_REF_TY]> for #name #ty_generics #where_clause{
            #[doc="类型转换"]
            #[inline]
            fn as_ref(&self) -> &[AS_MUT_REF_TY] {
                unsafe {
                    std::slice::from_raw_parts(
                        self as *const  Self as *const AS_MUT_REF_TY,
                        std::mem::size_of::<Self>() / std::mem::size_of::<AS_MUT_REF_TY>(),
                    )
                }
            }
        }
    });
    gen.into()
}
