//! # chromedriver
//!
//! Automatically download Chromedriver when the browser/driver versions do not match.
//!
//! # use
//! ### example with default config (for mac)
//!
//! ```no_run
//! use chromedriver_update::ChromeDriver;
//!
//! let mut driver = ChromeDriver::new();
//! driver.init().await;
//!
//! println!("driver version {}", driver.version);
//! println!("browser version {}", driver.browser_version);
//!
//! driver.try_download().await;
//! ```
//!
//! ### example with custom config
//!
//! ```no_run
//! use chromedriver_update::ChromeDriver;
//!
//! let mut driver = ChromeDriver::new();
//!  driver
//!    .set_driver_path("/other/path")
//!    .set_browser_path("/other/path")
//!    .set_connect_timeout(1000)
//!    .set_timeout(2000)
//!    .init()
//!    .await;
//!
//! println!("driver version {}", driver.version);
//! println!("browser version {}", driver.browser_version);
//!
//! driver.try_download().await;
//! ```
use regex::Regex;
use std::{
    fs,
    io::{Cursor, Read},
    os::unix::fs::PermissionsExt,
    process::Output,
};
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

pub struct ChromeDriver {
    pub version: String,
    pub browser_version: String,
    path: String,
    browser_path: String,
    connect_timeout: u64,
    timeout: u64,
}

impl ChromeDriver {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            path: "/usr/local/bin/chromedriver".to_string(),
            browser_version: String::new(),
            browser_path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
                .to_string(),
            connect_timeout: 5000,
            timeout: 10000,
        }
    }

    /// change chromedriver path, default: `/usr/local/bin/chromedriver`
    pub fn set_driver_path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    /// change chrome browser path, default: `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`
    pub fn set_browser_path(&mut self, path: &str) -> &mut Self {
        self.browser_path = path.to_string();
        self
    }

    /// change connect_timeout(ms) for download request, default: `5000`
    pub fn set_connect_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connect_timeout = timeout;
        self
    }

    /// change timeout(ms) for download request, default: `10000`
    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// setup with driver/browser version
    pub async fn init(&mut self) {
        self.version = self.get_driver_version().await;
        self.browser_version = self.get_browser_version().await;
    }

    /// try download chromedriver when version not matched
    pub async fn try_download(&self) {
        if !self.version.eq(&self.browser_version) {
            self.download_driver().await;
        }
    }

    async fn download_driver(&self) {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_millis(self.connect_timeout))
            .timeout(std::time::Duration::from_millis(self.timeout))
            .build()
            .unwrap();

        let url = format!("https://storage.googleapis.com/chrome-for-testing-public/{}/mac-x64/chromedriver-mac-x64.zip", self.browser_version);
        let bytes = client
            .get(url)
            .send()
            .await
            .expect("request timeout, you may need vpn to update chromedriver")
            .bytes()
            .await
            .unwrap();

        let cursor = Cursor::new(bytes.as_ref());
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let file_name = file.name();

            if file_name.eq("chromedriver-mac-x64/chromedriver") {
                let mut output_file = File::create(&self.path).await.unwrap();
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                output_file.write_all(&buffer).await.unwrap();

                let permissions = fs::metadata(&self.path).unwrap().permissions();
                let mut new_permissions = permissions.clone();
                new_permissions.set_mode(0o755);
                fs::set_permissions(&self.path, new_permissions).unwrap();
            }
        }
    }

    async fn get_driver_version(&self) -> String {
        match Command::new(self.path.clone())
            .arg("--version")
            .output()
            .await
        {
            Ok(res) => get_version_from_output(res),
            Err(_) => String::new(),
        }
    }

    async fn get_browser_version(&self) -> String {
        let output = Command::new(self.browser_path.clone())
            .arg("--version")
            .output()
            .await
            .expect("cannot find chrome");
        get_version_from_output(output)
    }
}

fn get_version_from_output(output: Output) -> String {
    let text = String::from_utf8(output.stdout).unwrap();
    let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
    re.captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string()
}
