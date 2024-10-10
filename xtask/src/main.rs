use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{error::ErrorKind, CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "storyboard68 xtask")]
enum Options {
    #[command(about = "Cross-compile keyboard firmware into elf and hex")]
    Build,
    #[command(about = "Build and flash firmware into device using dfu-programmer")]
    Deploy,
}

fn main() {
    let res: Result<(), Box<dyn Error>> = match Options::parse() {
        Options::Build => build(),
        Options::Deploy => deploy(),
    };

    if let Err(err) = res {
        Options::command().error(ErrorKind::Io, err).exit();
    }
}

fn build() -> Result<(), Box<dyn Error>> {
    if !Command::new(cargo_cmd())
        .args(["build", "--release"])
        .current_dir(project_root().join("firmware"))
        .status()?
        .success()
    {
        return Err("build process terminated unexpectedly".into());
    }

    if !Command::new("avr-objcopy")
        .args(["-O", "ihex", "storyboard68.elf", "storyboard68.hex"])
        .current_dir(target_release())
        .status()?
        .success()
    {
        return Err("hex file generation failed".into());
    }

    Ok(())
}

fn deploy() -> Result<(), Box<dyn Error>> {
    build()?;

    let release = target_release();

    if !Command::new("dfu-programmer")
        .args(["atmega32u4", "erase", "--force"])
        .current_dir(&release)
        .status()?
        .success()
    {
        return Err("flash preparation process terminated unexpectedly.".into());
    }

    if !Command::new("dfu-programmer")
        .args(["atmega32u4", "flash", "--force", "storyboard68.hex"])
        .current_dir(&release)
        .status()?
        .success()
    {
        return Err("flash process terminated unexpectedly".into());
    }

    if !Command::new("dfu-programmer")
        .args(["atmega32u4", "reset"])
        .current_dir(&release)
        .status()?
        .success()
    {
        return Err("device reset failed".into());
    }

    Ok(())
}

fn cargo_cmd() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn target_release() -> PathBuf {
    project_root().join("target/atmega32u4/release")
}
