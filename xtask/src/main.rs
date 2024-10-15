use std::{
    env,
    error::Error,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

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

fn build(keyboard: &str) -> Result<Artifact, Box<dyn Error>> {
    let mut build_cmd = Command::new(cargo_cmd())
        .args([
            "build",
            "--release",
            "--message-format=json-render-diagnostics",
        ])
        .current_dir(
            project_root()
                .join("firmware")
                .join("keyboard")
                .join(keyboard),
        )
        .stdout(Stdio::piped())
        .spawn()?;

    let reader = BufReader::new(build_cmd.stdout.take().unwrap());

    let mut artifact = None::<Artifact>;
    for res in cargo_metadata::Message::parse_stream(reader) {
        if let Message::CompilerArtifact(
            exec_artifact @ Artifact {
                executable: Some(_),
                ..
            },
        ) = res?
        {
            artifact = Some(exec_artifact);
        }
    }

    if !build_cmd.wait()?.success() {
        return Err("build process terminated unexpectedly".into());
    }

    let Some(artifact) = artifact else {
        return Err("compile process did not produced any artifact".into());
    };

    let exec = artifact.executable.as_ref().unwrap();
    let hex_file_name = format!("{}.hex", exec.file_stem().unwrap_or("firmware"));
    let output_dir = exec.ancestors().nth(1).unwrap();

    if !Command::new("avr-objcopy")
        .args([
            "-O",
            "ihex",
            exec.as_str(),
            output_dir.join(hex_file_name).as_str(),
        ])
        .current_dir(output_dir)
        .status()?
        .success()
    {
        return Err("hex file generation failed".into());
    }

    Ok(artifact)
}

fn deploy(keyboard: &str) -> Result<(), Box<dyn Error>> {
    let artifact = build(keyboard)?;

    if !Command::new("dfu-programmer")
        .args(["atmega32u4", "erase", "--force"])
        .status()?
        .success()
    {
        return Err("flash preparation process terminated unexpectedly".into());
    }

    if !Command::new("dfu-programmer")
        .args([
            "atmega32u4",
            "flash",
            "--force",
            artifact.executable.as_ref().unwrap().as_str(),
        ])
        .status()?
        .success()
    {
        return Err("flash process terminated unexpectedly".into());
    }

    if !Command::new("dfu-programmer")
        .args(["atmega32u4", "reset"])
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
