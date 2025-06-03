use std::{io, path::Path};

use syn::{Attribute, Item, parse_quote};

fn main() {
    const PB_HEAD: &str = "cs_proto/head.proto";
    const PB_ACTION: &str = "cs_proto/action.proto";

    println!("cargo:rerun-if-changed=cs_proto");

    if Path::new("cs_proto").exists() {
        prost_build::Config::new()
            .out_dir("out/")
            .compile_protos(&[PB_HEAD, PB_ACTION], &["."])
            .unwrap();

        build_cs_protocol();
        build_server_only_protocol();
    }
}

fn build_server_only_protocol() {
    const PB_SERVER_ONLY: &str = "cs_proto/server_only.proto";
    const OUT_SERVER_ONLY: &str = "out/server_only.rs";

    prost_build::Config::new()
        .out_dir("out/")
        .message_attribute(".", "#[derive(::proto_derive::NetCmd)]")
        .compile_protos(&[PB_SERVER_ONLY], &["."])
        .unwrap();

    make_additional_attributes(Path::new(OUT_SERVER_ONLY)).unwrap();
}

fn build_cs_protocol() {
    const PB_CURRENT: &str = "cs_proto/cs_current.proto";
    const PB_INTERNAL: &str = "cs_proto/cs_internal.proto";

    const OUT_CURRENT: &str = "out/cs_current.rs";
    const OUT_INTERNAL: &str = "out/cs_internal.rs";
    const OUT_FINAL: &str = "out/cs_output.rs";

    prost_build::Config::new()
        .out_dir("out/")
        .message_attribute(".", "#[derive(::proto_derive::NetCmd)]")
        .compile_protos(&[PB_INTERNAL], &["."])
        .unwrap();

    prost_build::Config::new()
        .out_dir("out/")
        .message_attribute(".", "#[derive(::proto_derive::NetCmd)]")
        .compile_protos(&[PB_CURRENT], &["."])
        .unwrap();

    make_additional_attributes(Path::new(OUT_CURRENT)).unwrap();

    update_attributes(
        Path::new(OUT_INTERNAL),
        Path::new(OUT_CURRENT),
        Path::new(OUT_FINAL),
    )
    .unwrap();

    std::fs::remove_file(OUT_INTERNAL).unwrap();
    std::fs::remove_file(OUT_CURRENT).unwrap();
}

fn make_additional_attributes(path: &Path) -> io::Result<()> {
    use std::io::{BufRead, BufReader};

    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    let mut cmd_id_attr = None;
    let mut xor_value = None;
    for line in reader.lines() {
        let line = line?;
        if line.contains("xor const: ") {
            if !line.contains("xor const: 0") {
                xor_value = Some(
                    line.split("xor const: ")
                        .nth(1)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                );
            }
        } else if line.contains("CmdID: ") {
            let cmd_id = line
                .split("CmdID: ")
                .nth(1)
                .unwrap()
                .parse::<u16>()
                .unwrap();

            cmd_id_attr = Some(format!("#[cmd_id({cmd_id})]"));
        } else if line.contains("#[prost(") && xor_value.map(|v| v != 0).unwrap_or(false) {
            output.push(line.replace(
                ")]",
                &format!(", xor = \"{}\")]", xor_value.take().unwrap()),
            ));
        } else {
            output.push(line);
            if let Some(attr) = cmd_id_attr.take() {
                output.push(attr);
            }
        }
    }

    std::fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn update_attributes(internal: &Path, current: &Path, output: &Path) -> io::Result<()> {
    let mut internal = syn::parse_file(&std::fs::read_to_string(internal)?).unwrap();
    let current = syn::parse_file(&std::fs::read_to_string(current)?).unwrap();

    let message_derive_attribute: Attribute = parse_quote! {
        #[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
    };

    let net_response_attribute: Attribute = parse_quote!(#[derive(::proto_derive::NetResponse)]);

    let mut missing_counter = 0;
    internal.items.iter_mut().for_each(|old_item| {
        if let Item::Struct(old_item) = old_item {
            old_item.attrs = vec![message_derive_attribute.clone()];

            old_item
                .fields
                .iter_mut()
                .for_each(|field| field.attrs.clear());

            if old_item.fields.iter().any(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| ident == "retcode")
                    .unwrap_or(false)
            }) {
                old_item.attrs.push(net_response_attribute.clone());
            }

            if let Some(Item::Struct(new_item)) = current.items.iter().find(|new_item| {
                if let Item::Struct(new_item) = new_item {
                    new_item.ident == old_item.ident
                } else {
                    false
                }
            }) {
                new_item.attrs.iter().for_each(|attr| {
                    if attr.path().is_ident("cmd_id") {
                        old_item.attrs.push(attr.clone());
                    }
                });

                old_item.fields.iter_mut().for_each(|old_field| {
                    if let Some(new_field) = new_item
                        .fields
                        .iter()
                        .find(|new_field| new_field.ident == old_field.ident)
                    {
                        old_field.attrs = new_field.attrs.clone();
                    } else {
                        missing_counter += 1;
                        println!(
                            "cargo:warning=cs_current.proto is missing [{}] \"{}::{}\"",
                            missing_counter,
                            old_item.ident,
                            old_field.ident.as_ref().unwrap(),
                        );
                    }
                })
            } else {
                missing_counter += 1;
                println!(
                    "cargo:warning=cs_current.proto is missing [{}] \"{}\"",
                    missing_counter, old_item.ident,
                );
            }
        }
    });

    std::fs::write(output, prettyplease::unparse(&internal))
}
