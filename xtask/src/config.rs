use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
/// xtask firmware build configuration
pub struct Config {
    pub build: BuildConfig,
    pub deploy: DeployConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default)]
    /// Commands running before build process
    pub pre_commands: Vec<String>,

    #[serde(default)]
    /// Commands running after build process
    pub post_commands: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeployConfig {
    #[serde(default)]
    /// Deploy commands. Needs to be specified to run deploy.
    pub cmds: Vec<String>,
}
