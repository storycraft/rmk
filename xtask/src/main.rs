use std::{
    env,
    error::Error,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use camino::Utf8PathBuf;
use cargo_metadata::{Artifact, Message};
use clap::{error::ErrorKind, CommandFactory, Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "storyboard68 xtask")]
struct KeyboardCommand {
    #[arg(name = "options", help = "firmware build options")]
    options: BuildOptions,
    #[arg(help = "keyboard target")]
    keyboard: String,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum BuildOptions {
    #[value(
        name = "build",
        help = "cross-compile keyboard firmware into elf and hex"
    )]
    Build,
    #[value(
        name = "deploy",
        help = "build and flash firmware into device using dfu-programmer"
    )]
    Deploy,
}

fn main() {
    let cmd = KeyboardCommand::parse();

    let res: Result<(), Box<dyn Error>> = match cmd.options {
        BuildOptions::Build => build(&cmd.keyboard).map(|_| ()),
        BuildOptions::Deploy => deploy(&cmd.keyboard),
    };

    if let Err(err) = res {
        KeyboardCommand::command().error(ErrorKind::Io, err).exit();
    }
}

fn build(keyboard: &str) -> Result<Option<Utf8PathBuf>, Box<dyn Error>> {
    let mut build_cmd = Command::new(cargo_cmd())
        .args(["build", "--release"])
        .current_dir(
            project_root()
                .join("firmware")
                .join("keyboard")
                .join(keyboard),
        )
        .stdout(Stdio::piped())
        .spawn()?;

    let reader = BufReader::new(build_cmd.stdout.take().unwrap());

    let mut exec_path = None::<Utf8PathBuf>;

    for res in cargo_metadata::Message::parse_stream(reader) {
        if let Message::CompilerArtifact(Artifact {
            executable: Some(executable),
            ..
        }) = res?
        {
            exec_path = Some(executable);
        }
    }

    if !build_cmd.wait()?.success() {
        return Err("build process terminated unexpectedly".into());
    }

    if let Some(ref exec) = exec_path {
        let hex_file_name = format!("{}.hex", exec.file_name().unwrap_or("firmware"));
        let output_dir = exec.ancestors().nth(1).unwrap();

        if !Command::new("avr-objcopy")
            .args([
                "-O",
                "ihex",
                exec.as_str(),
                output_dir.join(hex_file_name).as_str(),
            ])
            .current_dir(target_release())
            .status()?
            .success()
        {
            return Err("hex file generation failed".into());
        }
    }

    Ok(exec_path)
}

fn deploy(keyboard: &str) -> Result<(), Box<dyn Error>> {
    let executables = build(keyboard)?;

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
        .args([
            "atmega32u4",
            "flash",
            "--force",
            executables
                .ok_or("cannot find any built artifact")?
                .as_str(),
        ])
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
    project_root().join("firmware/target/atmega32u4/release")
}
