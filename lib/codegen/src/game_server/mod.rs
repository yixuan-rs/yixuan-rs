// Helper macros for game-server

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn impl_model_manager(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let Data::Struct(data) = input.data else {
        panic!("unexpected input type");
    };

    let model_field_names = data
        .fields
        .iter()
        .filter_map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("model"))
                .then_some(field.ident.as_ref().unwrap())
        })
        .collect::<Vec<_>>();

    let name = &input.ident;
    quote! {
        impl ModelManager for #name {
            fn is_any_model_modified(&self) -> bool {
                #(self.#model_field_names.is_any_field_changed() )||*
            }

            fn changes_acknowledged(&mut self) {
                #(self.#model_field_names.reset_changed_fields();)*
            }

            fn has_models_to_synchronize(&self) -> bool {
                #((self.#model_field_names.supports_player_sync() && self.#model_field_names.is_any_field_changed()))||*
            }

            fn for_each_model(&self, mut f: impl FnMut(&dyn Model)) {
                #(f(&self.#model_field_names);)*
            }

            fn for_each_model_mut(&mut self, mut f: impl FnMut(&mut dyn Model)) {
                #(f(&mut self.#model_field_names);)*
            }
        }
    }
    .into()
}

pub fn impl_model_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let Data::Struct(data) = input.data else {
        panic!("unexpected input type");
    };

    let fields = data
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();

    let name = &input.ident;
    quote! {
        impl Model for #name {
            fn is_any_field_changed(&self) -> bool {
                #(self.#fields.is_changed() )||*
            }

            fn reset_changed_fields(&mut self) {
                #(self.#fields.reset_changed_state();)*
            }
        }
    }
    .into()
}

pub fn impl_property_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let Data::Struct(data) = input.data else {
        panic!("unexpected input type");
    };

    let fields = data
        .fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();

    let name = &input.ident;
    quote! {
        impl Property for #name {
            fn is_changed(&self) -> bool {
                #(self.#fields.is_changed() )||*
            }

            fn reset_changed_state(&mut self) {
                #(self.#fields.reset_changed_state();)*
            }
        }
    }
    .into()
}
