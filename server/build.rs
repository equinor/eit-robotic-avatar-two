use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=static");
    let _ = fs::create_dir("static");
}
