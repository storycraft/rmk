use std::{env, error::Error, fs, path::PathBuf};

use usb_keyboard_config::build_config;

/// Create keyboard configuration source by reading configuration toml
pub fn define_keyboard_config(path: &str, out_name: &str) -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed={path}");

    let out_dir = env::var("OUT_DIR").unwrap();

    // build keyboard config
    fs::write(
        PathBuf::from(out_dir).join(out_name),
        build_config(toml::Deserializer::new(&fs::read_to_string(path)?))?,
    )?;

    Ok(())
}
