use std::error::Error;

use create_rmk_build::define_keyboard_config;

fn main() -> Result<(), Box<dyn Error>> {
    define_keyboard_config("keyboard.toml", "config.rs")?;

    Ok(())
}
