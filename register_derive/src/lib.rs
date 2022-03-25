extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Register)]
/// Assuming there exists an enum called `Address`, this adds a crate-public
/// function `addr() -> Address` which gets the element
/// from the enum with matching name
pub fn derive_register(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let addr_impl = quote! {
        impl #name {
            pub(crate) fn addr() -> Address {
                Address::#name
            }
        }
    };
    TokenStream::from(addr_impl)
}
