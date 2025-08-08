use std::fs;
use std::path::PathBuf;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

fn main() {
    const B64: &str = include_str!("blockmappings.b64");
    let data: String = B64.lines().collect();
    let bytes = STANDARD
        .decode(data)
        .expect("invalid base64 block mapping");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(out_dir.join("blockmappings.bz2"), bytes)
        .expect("failed to write blockmappings.bz2");
}

