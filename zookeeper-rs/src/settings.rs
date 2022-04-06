use serde::Deserialize;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use config::{Config, ConfigError, Environment, File};

#[derive(Clone, Debug, Deserialize)]
pub enum RunEnvironment {
    Local,
    Development,
    Testing,
    Production,
}

impl FromStr for RunEnvironment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(RunEnvironment::Local),
            "dev" => Ok(RunEnvironment::Development),
            "test" => Ok(RunEnvironment::Testing),
            "prod" => Ok(RunEnvironment::Production),
            _ => Err("Invalid environment provided."),
        }
    }
}

impl fmt::Display for RunEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let filename = match self {
            RunEnvironment::Local => "Local.toml",
            RunEnvironment::Development => "Development.toml",
            RunEnvironment::Testing => "Testing.toml",
            RunEnvironment::Production => "Production.toml",
        };
        write!(f, "{}", filename)
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub tick_time: u32,
    pub init_limit: u32,
    pub sync_limit: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Settings {
    pub fn new(run_env: &RunEnvironment, config_dir: &PathBuf) -> Result<Self, ConfigError> {
        // let run_env = match run_mode {
        //     Some(x) => x,
        //     None => RunEnvironment::Development,
        // };
        // let config_dir = match config_path {
        //     Some(x) => x,
        //     None => {
        //         let config_dir = env::current_dir().expect("Failed to find current working directory!");
        //         config_dir.join("config")
        //     }
        // };

        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::from(config_dir.join("Default.toml")))?;

        // Add in the current environment file
        s.merge(File::from(config_dir.join(run_env.to_string())).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
