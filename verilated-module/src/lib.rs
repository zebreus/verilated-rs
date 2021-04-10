#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn module(_: TokenStream, input: TokenStream) -> TokenStream {
    // Code is generated by build.rs, so eat all the tokens...

    let expanded = match syn::parse::<ItemStruct>(input) {
        Ok(item) => {
            let src = format!("/{}.rs", item.ident);
            quote! {
                include!(concat!(env!("OUT_DIR"), #src));
            }
        }
        Err(..) => quote!{},
    };

    expanded.into()
}