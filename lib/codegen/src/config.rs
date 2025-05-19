use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn impl_action(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let name = &item.ident;
    let vis = &item.vis;
    let attrs = &item.attrs;

    let Fields::Named(fields) = &item.fields else {
        panic!("unnamed fields are not supported");
    };

    let fields = &fields.named;

    quote! {
        #[derive(::serde::Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        #(#attrs)*
        #vis struct #name {
            #[serde(rename = "ID")]
            pub id: u32,
            #[serde(default)]
            pub predicates: Vec<ConfigPredicate>,
            #fields
        }
    }
    .into()
}
