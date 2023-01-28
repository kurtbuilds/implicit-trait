use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ImplItem};
use quote::quote;

#[proc_macro_attribute]
pub fn implicit_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemImpl);
    let trait_name = &item.trait_.as_ref().unwrap().1;
    let declarations = item.items.iter().map(|item| {
        match item {
            ImplItem::Method(method) => {
                let name = &method.sig.ident;
                let args = &method.sig.inputs;
                let ret = &method.sig.output;
                quote! {
                    fn #name(#args) #ret;
                }
            }
            _ => panic!("Only methods are allowed in an implicit trait"),
        }
    });
    (quote! {
        pub trait #trait_name {
            #(#declarations)*
        }
        #item
    }).into()
}