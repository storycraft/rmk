use std::{
    env,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Context};
use cargo_metadata::{Artifact, Message};

#[derive(Debug)]
/// Utilities of keyboard firmware directory
pub struct Keyboard<'a> {
    path: &'a Path,
    name: &'a str,
}

impl<'a> Keyboard<'a> {
    pub const fn new(parent_dir: &'a Path, name: &'a str) -> Self {
        Self {
            path: parent_dir,
            name,
        }
    }

    /// Name of keyboard
    pub fn name(&self) -> &str {
        self.name
    }

    /// Path of keyboard source
    pub fn path(&self) -> PathBuf {
        self.path.join(self.name)
    }

    /// Spawn build process and returns executable artifact
    pub fn build(&self) -> anyhow::Result<Artifact> {
        let mut build_cmd = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
            .args([
                "build",
                "--release",
                "--message-format=json-render-diagnostics",
            ])
            .current_dir(self.path())
            .stdout(Stdio::piped())
            .spawn()
            .context("cannot spawn compile process")?;

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

        if !build_cmd
            .wait()
            .context("compile process ended unexpectedly")?
            .success()
        {
            bail!("compile process failed")
        }

        let Some(artifact) = artifact else {
            bail!("compile process did not produced any executable artifact")
        };

        Ok(artifact)
    }
}
