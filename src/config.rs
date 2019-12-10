use clap::Clap;
use dirs::home_dir;

#[derive(Debug)]
pub enum LogLevel {
    Quiet,
    Info,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        match s {
            "quiet" => Ok(Self::Quiet),
            "info" => Ok(Self::Info),
            loglevel => Err(format!("I don't know the log level of {:?}", loglevel)),
        }
    }
}

#[derive(Clap, Debug)]
pub struct FnmConfig {
    /// https://nodejs.org/dist/ mirror
    #[clap(
        env = "FNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist/"
    )]
    pub node_dist_mirror: reqwest::Url,

    /// The root directory of fnm installations.
    #[clap(long = "fnm-dir", env = "FNM_DIR")]
    pub base_dir: Option<std::path::PathBuf>,

    /// Where the current node version link is stored
    #[clap(long = "multishell-path", env = "FNM_MULTISHELL_PATH")]
    pub multishell_path: Option<std::path::PathBuf>,

    /// The log level of fnm commands
    #[clap(long = "loglevel", env = "FNM_LOGLEVEL", default_value = "info")]
    pub loglevel: LogLevel,
}

impl FnmConfig {
    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        (self.base_dir.clone()).unwrap_or({
            let mut home_directory = home_dir().expect("Can't get home directory");
            home_directory.push(".fnm");
            home_directory
        })
    }

    pub fn installations_dir(&self) -> std::path::PathBuf {
        let mut base_dir = self.base_dir_with_default();
        base_dir.push("node-versions");
        base_dir
    }
}
