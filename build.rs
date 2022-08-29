use std::{fs, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=gen_consts.py");

    let c = Command::new("./gen_consts.py").output().unwrap();
    fs::write("src/consts.rs", c.stdout);
}
