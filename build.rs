use std::{env, fs::copy, path::Path};

use duct::cmd;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR to be set");
    let git_dir = Path::new(&out_dir).join("git");

    // drop error when cloning twice
    let _ = cmd!(
        "git",
        "clone",
        "https://github.com/c4dt/angular-components.git",
        git_dir.clone(),
    )
    .run();

    copy(
        git_dir.join("projects/lib/src/c4dt.css"),
        Path::new(&out_dir).join("c4dt.css"),
    )
    .expect("copy file");
}
