use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("../../../scripts/zshrc.sh");
    fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
    fs::copy("scripts/zshrc.sh", &dest_path).unwrap();
    println!("cargo:rerun-if-changed=scripts/zshrc.sh");
}
