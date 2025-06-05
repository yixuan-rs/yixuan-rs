use proc_macro::TokenStream;

mod config;
mod game_server;
mod gm_input;
mod packet_handling;

#[proc_macro_attribute]
pub fn handlers(attr: TokenStream, input: TokenStream) -> TokenStream {
    packet_handling::impl_handlers(attr, input)
}

#[proc_macro_attribute]
pub fn required_state(attr: TokenStream, input: TokenStream) -> TokenStream {
    packet_handling::impl_required_state(attr, input)
}

#[proc_macro_derive(ModelManager, attributes(model))]
pub fn derive_model_manager(item: TokenStream) -> TokenStream {
    game_server::impl_model_manager(item)
}

#[proc_macro_derive(Model, attributes(ignore_property))]
pub fn derive_model(item: TokenStream) -> TokenStream {
    game_server::impl_model_trait(item)
}

#[proc_macro_derive(Property)]
pub fn derive_property(item: TokenStream) -> TokenStream {
    game_server::impl_property_trait(item)
}

#[proc_macro_attribute]
pub fn action(_: TokenStream, item: TokenStream) -> TokenStream {
    config::impl_action(item)
}

#[proc_macro_derive(GMInput)]
pub fn derive_gm_input(item: TokenStream) -> TokenStream {
    gm_input::impl_gm_input(item)
}
