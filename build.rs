//! Sample schema builder.
use std::{env, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("flatc").args(&["-r", "-o", &out_dir, "schema/monster.fbs"])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=schema/monster.fbs");
}
