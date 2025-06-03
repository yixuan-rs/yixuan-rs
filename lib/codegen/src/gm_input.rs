use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Field, Fields, GenericArgument, PathArguments, Type, TypePath,
    parse_macro_input,
};

pub fn impl_gm_input(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let item_name = &item.ident;

    let Data::Enum(data) = &item.data else {
        panic!("#[derive(GMInput)] only supports enums")
    };

    let internal_enum_name = quote::format_ident!("{item_name}Type");
    let mut internal_enum_variants = TokenStream::new();
    let mut internal_enum_mapping = TokenStream::new();
    let mut parser_match_arms = TokenStream::new();

    for var in data.variants.iter() {
        let name = &var.ident;
        let name_lowercase_str = name.to_string().to_lowercase();

        internal_enum_variants.extend(quote! {
            #name,
        });

        internal_enum_mapping.extend(quote! {
            (#name_lowercase_str, #internal_enum_name::#name),
        });

        let mut assignments = TokenStream::new();

        if let Fields::Named(fields) = &var.fields {
            for field in fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();

                let field_type = get_field_type(field);
                let field_sub_type = get_field_sub_type(field);
                let assignment = match field_type.as_str() {
                    "String" => {
                        quote! { #field_name: data.next().ok_or(MissingArgument(#field_name_str))?.to_string(), }
                    }
                    "u32" | "i32" | "u16" | "i16" | "bool" => {
                        quote! { #field_name: data.next().ok_or(MissingArgument(#field_name_str))?.parse().map_err(|_| InvalidArgumentFormat(#field_name_str, #field_type))?, }
                    }
                    "Vec"
                        if matches!(
                            field_sub_type.as_str(),
                            "u32" | "i32" | "u16" | "i16" | "bool"
                        ) =>
                    {
                        quote! { #field_name: {
                            let mut list = Vec::new();
                            while let Some(value) = data.next() {
                                if list.is_empty() {
                                    if value == "[]" {
                                        break;
                                    }
                                    else if let Some(value) = value.strip_prefix('[') {
                                        list.push(value.parse().map_err(|_| InvalidArgumentFormat(#field_name_str, #field_type))?);
                                    }
                                    else {
                                        return Err(InvalidArgumentFormat(#field_name_str, #field_type));
                                    }
                                }
                                else {
                                    if let Some(value) = value.strip_suffix(']') {
                                        list.push(value.parse().map_err(|_| InvalidArgumentFormat(#field_name_str, #field_type))?);
                                        break;
                                    }
                                    else {
                                        list.push(value.parse().map_err(|_| InvalidArgumentFormat(#field_name_str, #field_type))?);
                                    }
                                }
                            }

                            list
                        }
                        }
                    }
                    unsupported => {
                        todo!("#[derive(GMInput)] doesn't support field type: {unsupported}")
                    }
                };

                assignments.extend(assignment);
            }

            parser_match_arms.extend(quote! {
                #internal_enum_name::#name => Ok(Self::#name {
                    #assignments
                }),
            });
        } else if var.fields.is_empty() {
            parser_match_arms.extend(quote! { #internal_enum_name::#name => Ok(Self::#name), });
        } else {
            panic!("#[derive(GMInput)] doesn't support unnamed fields");
        }
    }

    let internal_enum = quote! {
        enum #internal_enum_name {
            #internal_enum_variants
        }
    };

    quote! {
        #internal_enum

        impl GMInput for #item_name {
            fn from_str(input: &str) -> Result<Self, GMInputParseError> {
                use GMInputParseError::*;

                static CMD_TYPE_MAP: ::std::sync::LazyLock<::std::collections::HashMap<&'static str, #internal_enum_name>> =
                    ::std::sync::LazyLock::new(|| ::std::collections::HashMap::from([#internal_enum_mapping]));

                let mut data = input.split(' ');

                let cmd_name = data.next().unwrap().to_string();
                let cmd_type = CMD_TYPE_MAP.get(cmd_name.to_lowercase().as_str()).ok_or(UnknownCommand(cmd_name))?;

                match cmd_type {
                    #parser_match_arms
                }
            }
        }
    }
    .into()
}

#[must_use]
fn get_type_name(path: &syn::Path) -> String {
    path.segments.last().unwrap().ident.to_string()
}

#[must_use]
fn get_field_type(field: &Field) -> String {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => get_type_name(path),
        _ => panic!("Unsupported field type"),
    }
}

#[must_use]
fn get_field_sub_type(field: &Field) -> String {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            match &last_segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                        args.args.last()
                    {
                        get_type_name(path)
                    } else {
                        get_type_name(path)
                    }
                }
                _ => get_type_name(path),
            }
        }
        _ => panic!("Unsupported field type"),
    }
}
