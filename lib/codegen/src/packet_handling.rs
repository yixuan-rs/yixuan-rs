use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, Ident, ImplItem, ItemFn, ItemImpl, ReturnType, parse_macro_input};

pub fn impl_handlers(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);
    let impl_name = &impl_block.self_ty;

    let mut output_module = proc_macro2::TokenStream::new();
    let mut registrations = proc_macro2::TokenStream::new();

    for item in impl_block.items.iter() {
        let ImplItem::Fn(func) = item else {
            output_module.extend(item.to_token_stream());
            continue;
        };

        let sig = &func.sig;
        let name = &sig.ident;

        assert_eq!(sig.inputs.len(), 2);

        let argument = match sig.inputs.get(1).as_ref().unwrap() {
            FnArg::Typed(argument) => argument,
            FnArg::Receiver(_) => unreachable!(),
        };

        output_module.extend(quote! {
            #[::tracing::instrument(skip_all)]
            #func
        });

        let request_type = argument.ty.as_ref();
        let has_return_message = ReturnType::Default != sig.output;

        if has_return_message {
            registrations.extend(quote! {
                handler_map.insert(#request_type::CMD_ID, |context, cmd_id, body| {
                    let Ok(req) = #request_type::decode(body.as_ref()) else {
                        ::tracing::error!("failed to decode {}", stringify!(#request_type));
                        return;
                    };

                    let rsp = Self::#name(context, req);
                    context.response = Some((rsp.get_cmd_id(), rsp.encode_to_vec()))
                });
            });
        } else {
            registrations.extend(quote! {
                handler_map.insert(#request_type::CMD_ID, |context, cmd_id, body| {
                    let Ok(req) = #request_type::decode(body.as_ref()) else {
                        ::tracing::error!("failed to decode {}", stringify!(#request_type));
                        return;
                    };

                    Self::#name(context, req);
                });
            });
        }
    }

    quote! {
        impl #impl_name {
            pub fn register_handlers(handler_map: &mut ::std::collections::HashMap<u16, super::PacketHandler>) {
                use vivian_proto::*;

                #registrations
            }

            #output_module
        }
    }
    .into()
}

pub fn impl_required_state(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Ident);
    let func = parse_macro_input!(input as ItemFn);

    let vis = &func.vis;
    let sig = &func.sig;
    let func_name = &sig.ident;
    let block = &func.block;

    let FnArg::Typed(context_arg) = func.sig.inputs.get(0).unwrap() else {
        panic!("First argument should be of type &mut NetContext<'_>");
    };

    let context_arg_pat = &context_arg.pat;

    if let ReturnType::Type(_, return_message) = &func.sig.output {
        quote! {
            #vis #sig {
                if !#context_arg_pat.player.ensure_state(crate::player::LoadingState::#attr) {
                    ::tracing::error!("{}: ensure_state({:?}) failed", stringify!(#func_name), stringify!(#attr));

                    #[allow(clippy::needless_update)]
                    return #return_message {
                        retcode: 1,
                        ..Default::default()
                    };
                }

                #block
            }
        }.into()
    } else {
        quote! {
            #vis #sig {
                if #context_arg_pat.player.ensure_state(crate::player::PlayerState::#attr) {
                    ::tracing::error!("{}: ensure_state({:?}) failed", stringify!(#func_name), stringify!(#attr));
                    return;
                }

                #block
            }
        }.into()
    }
}
