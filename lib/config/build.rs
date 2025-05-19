use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=fbs");

    let inputs = std::fs::read_dir("fbs")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();

    let _ = flatc_rust::run(flatc_rust::Args {
        inputs: &inputs.iter().map(|buf| buf.as_path()).collect::<Vec<_>>(),
        out_dir: Path::new("gen_flatbuffers"),
        ..Default::default()
    });
}
