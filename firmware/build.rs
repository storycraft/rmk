use std::{env, fs, path::PathBuf};

use usb_keyboard_config::build_config;

fn main() {
    println!("cargo::rerun-if-changed=keyboard.toml");

    let out_dir = env::var("OUT_DIR").unwrap();

    // build keyboard config
    fs::write(
        PathBuf::from(out_dir).join("config.rs"),
        build_config(toml::Deserializer::new(include_str!("keyboard.toml")))
            .expect("cannot parse keyboard.toml"),
    )
    .expect("cannot write config.rs");
}
