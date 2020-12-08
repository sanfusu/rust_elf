use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MetaData)]
pub fn elf_metadata(input: TokenStream) -> TokenStream {
    _elf_metadata(input.into()).into()
}

fn _elf_metadata(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let name = ast.ident;

    quote! {
        impl crate::interface::MetaData for #name{}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
