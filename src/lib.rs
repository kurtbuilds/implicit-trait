use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ImplItem, AttributeArgs};
use quote::{quote};

#[proc_macro_attribute]
pub fn implicit_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemImpl);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut vis = quote!();
    for arg in args.into_iter() {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                if path.is_ident("pub") {
                    vis = quote! { pub };
                } else {
                    panic!("Only 'pub' is allowed as an argument to implicit_trait");
                }
            }
            _ => panic!("Only 'pub' is allowed as an argument to implicit_trait"),
        }
    }
    let trait_name = &item.trait_.as_ref().unwrap().1;
    let declarations = item.items.iter().map(|item| {
        match item {
            ImplItem::Method(method) => {
                let sig = &method.sig;
                quote! {
                    #sig;
                }
            }
            _ => panic!("Only methods are allowed in an implicit trait"),
        }
    });
    (quote! {
        #vis trait #trait_name {
            #(#declarations)*
        }
        #item
    }).into()
}