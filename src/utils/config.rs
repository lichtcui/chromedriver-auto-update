use serde::{Deserialize, Serialize};
use std::{fs, sync::LazyLock};

static CONFIG_FILE_PATH: &str = "local/config.toml";
pub static LOCAL_CONFIG: LazyLock<Config> = LazyLock::new(Config::load);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default = "Config::default_value")]
pub struct Config {
    pub driver: Driver,
    pub browser: Browser,
}

impl Config {
    pub fn load() -> Self {
        let data = fs::read_to_string(CONFIG_FILE_PATH).expect("unable to read config file");
        toml::from_str(&data).expect("config file is not valid")
    }

    fn default_value() -> Self {
        Self {
            driver: Driver::default_value(),
            browser: Browser::default_value(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default = "Driver::default_value")]
pub struct Driver {
    pub path: String,
    pub connect_timeout: u64,
    pub timeout: u64,
}

impl Driver {
    fn default_value() -> Self {
        Self {
            path: "/usr/local/bin/chromedriver".to_string(),
            connect_timeout: 5000,
            timeout: 10000,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default = "Browser::default_value")]
pub struct Browser {
    pub path: String,
}

impl Browser {
    fn default_value() -> Self {
        Self {
            path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string(),
        }
    }
}
