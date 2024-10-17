mod config;
mod keyboard;
mod template;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use clap::{Parser, ValueEnum};
use config::{BuildConfig, Config};
use handlebars::Handlebars;
use keyboard::Keyboard;
use serde::Serialize;
use subprocess::Exec;
use template::{Executable, PathContext};
use tracing::{event, instrument, level_filters::LevelFilter, Level};
use tracing_subscriber::{fmt::format, layer::SubscriberExt, util::SubscriberInitExt};

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
    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(tracing_subscriber::fmt::layer().event_format(format().compact().without_time()))
        .try_init()
        .unwrap();

    if let Err(err) = run(KeyboardCommand::parse()) {
        event!(Level::ERROR, "{:?}", err);
    }
}

fn run(cmd: KeyboardCommand) -> anyhow::Result<()> {
    let keyboard = keyboard_dir();
    let keyboard = Keyboard::new(&keyboard, &cmd.keyboard);

    event!(Level::INFO, "keyboard: {}", keyboard.name());
    event!(
        Level::INFO,
        "firmware_directory: {}",
        keyboard.path().display()
    );

    let config = {
        let path = keyboard.path().join("xtask.toml");

        if fs::exists(&path)? {
            toml::from_str(&fs::read_to_string(&path).context("cannot read xtask.toml")?)
                .context("configuration file is invalid")?
        } else {
            Config::default()
        }
    };

    match cmd.options {
        BuildOptions::Build => {
            build(&keyboard, &config.build)?;
        }

        BuildOptions::Deploy => {
            deploy(&keyboard, &config)?;
        }
    };

    Ok(())
}

#[instrument(level = Level::INFO, skip_all)]
fn deploy(keyboard: &Keyboard<'_>, config: &Config) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct DeployContext {
        pub path: PathContext,
        pub exec: Executable,
    }

    if config.deploy.cmds.is_empty() {
        bail!("deploy commands are not configured")
    }

    let exec = build(keyboard, &config.build)?;

    event!(Level::INFO, "running deploy process");
    run_cmds(
        &config.deploy.cmds,
        DeployContext {
            path: PathContext::new(),
            exec,
        },
    )
    .context("deploy failed")?;

    Ok(())
}

#[instrument(level = Level::INFO, skip_all)]
fn build(keyboard: &Keyboard, config: &BuildConfig) -> anyhow::Result<Executable> {
    #[derive(Serialize)]
    struct PostBuildContext {
        pub path: PathContext,
        pub exec: Executable,
    }

    event!(Level::INFO, "running pre-build commands");
    run_cmds(&config.pre_cmds, ()).context("pre-build commands failed")?;

    event!(Level::INFO, "running build process");
    let artifact = keyboard.build().context("firmware build failed")?;

    let exec = Executable::new_from_path(artifact.executable.as_ref().unwrap().clone()).unwrap();

    let cx = PostBuildContext {
        path: PathContext::new(),
        exec,
    };

    event!(Level::INFO, "running post-build commands");
    run_cmds(&config.post_cmds, &cx).context("post-build commands failed")?;

    event!(Level::INFO, "target: {}", artifact.target.name);
    event!(Level::INFO, "artifacts: {:?}", artifact.filenames);
    event!(
        Level::INFO,
        "executable: {}",
        artifact.executable.as_ref().unwrap()
    );

    Ok(cx.exec)
}

fn run_cmds(cmds: &[String], context: impl Serialize) -> anyhow::Result<()> {
    let reg = Handlebars::new();

    for cmd in cmds {
        let proc = Exec::shell(reg.render_template(cmd, &context)?);

        if !proc
            .join()
            .with_context(|| format!("command: {} ended unexpectedly", cmd))?
            .success()
        {
            bail!("command: {} ended with error", cmd);
        }
    }

    Ok(())
}

pub fn keyboard_dir() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .join("firmware")
        .join("keyboard")
}
