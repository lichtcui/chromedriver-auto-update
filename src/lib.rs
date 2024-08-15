/*!
Automatically download Chromedriver when the browser/driver versions do not match.

### Use default values
```no_run
use chromedriver_update::ChromeDriver;

#[tokio::main]
async fn main() {
    let mut driver = ChromeDriver::new();
    driver.init().await.unwrap();
    if driver.need_download() {
        driver.try_download().await.unwrap();
    }
}
```

### Use custom values

```no_run
use chromedriver_update::ChromeDriver;

#[tokio::main]
async fn main() {
    let mut driver = ChromeDriver::new();
    driver
        .set_driver_path("/usr/local/bin/chromedriver")
        .set_browser_path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
        .set_connect_timeout(2000)
        .set_timeout(5000)
        .init()
        .await
        .unwrap();

    println!("driver version {}", driver.version);
    println!("browser version {}", driver.browser_version);

    if !driver.need_download() {
        println!("no need to update driver");
        return;
    }

    println!("updating driver ...");

    match driver.try_download().await {
        Ok(_) => println!("Download driver successful"),
        Err(err) => eprintln!("Download driver failed, {}", err),
    }
}
```
*/
use regex::Regex;
use std::{
    io::{Cursor, Read},
    process::Output,
};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

pub mod constant;
use constant::{
    CHROME_BROWSER_PATH, CHROME_DRIVER_PATH, CONNECT_TIMEOUT, DRIVER_FILE, TIMEOUT, ZIP_PATH,
};

pub struct ChromeDriver {
    /// Chrome driver version
    pub version: String,
    /// Chrome browser version
    pub browser_version: String,
    path: String,
    browser_path: String,
    connect_timeout: u64,
    timeout: u64,
}

impl ChromeDriver {
    /// Create driver
    pub fn new() -> Self {
        Self {
            version: String::new(),
            path: CHROME_DRIVER_PATH.to_string(),
            browser_version: String::new(),
            browser_path: CHROME_BROWSER_PATH.to_string(),
            connect_timeout: CONNECT_TIMEOUT,
            timeout: TIMEOUT,
        }
    }

    /// Update chromedriver path. Default:
    /// - mac:    `/usr/local/bin/chromedriver`
    /// - linux:  `/usr/bin/chromedriver`
    /// - windows:  ``
    pub fn set_driver_path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    /// Update chrome browser path. Default:
    /// - mac:    `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`
    /// - linux:  `/usr/bin/google-chrome`
    /// - windows:  ``
    pub fn set_browser_path(&mut self, path: &str) -> &mut Self {
        self.browser_path = path.to_string();
        self
    }

    /// Update connect_timeout (ms) for download requests. Default: 5000.
    pub fn set_connect_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connect_timeout = timeout;
        self
    }

    /// Update timeout (ms) for download requests. Default: 5000.
    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Setup driver & browser version
    pub async fn init(&mut self) -> DriverResult<()> {
        self.version = self.get_driver_version().await;
        self.browser_version = self.get_browser_version().await?;

        Ok(())
    }

    /// Compare driver & browser version
    pub fn need_download(&self) -> bool {
        !self.version.eq(&self.browser_version)
    }

    /// Download Chromedriver
    pub async fn try_download(&self) -> DriverResult<()> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_millis(self.connect_timeout))
            .timeout(std::time::Duration::from_millis(self.timeout))
            .build()
            .map_err(|_| DriverError::RequestInvalid)?;

        let url = format!(
            "https://storage.googleapis.com/chrome-for-testing-public/{}/{}",
            self.browser_version,
            ZIP_PATH.as_str()
        );
        let bytes = client
            .get(url)
            .send()
            .await
            .map_err(|_| DriverError::RequestTimeout)?
            .bytes()
            .await
            .map_err(|_| DriverError::RequestInvalid)?;

        let cursor = Cursor::new(bytes.as_ref());
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|_| DriverError::ResourceInvalid(ZIP_PATH.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            if file.name().eq(DRIVER_FILE.as_str()) {
                let mut output_file = File::create(&self.path)
                    .await
                    .map_err(|_| DriverError::ResourceInvalid(self.path.clone()))?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|_| DriverError::ResourceInvalid(self.path.clone()))?;
                output_file
                    .write_all(&buffer)
                    .await
                    .map_err(|_| DriverError::ResourceInvalid(self.path.clone()))?;

                #[cfg(unix)]
                {
                    use std::{fs, os::unix::fs::PermissionsExt};
                    let mut permissions = fs::metadata(&self.path).unwrap().permissions();
                    permissions.set_mode(0o755);
                    fs::set_permissions(&self.path, permissions).unwrap();
                }

                return Ok(());
            }
        }

        Err(DriverError::ResourceNotFound(DRIVER_FILE.to_string()))
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

    async fn get_browser_version(&self) -> DriverResult<String> {
        let path = self.browser_path.clone();

        #[cfg(unix)]
        {
            let output = Command::new(&path)
                .arg("--version")
                .output()
                .await
                .map_err(|_| DriverError::BrowserNotFound(path))?;
            Ok(get_version_from_output(output))
        }

        #[cfg(windows)]
        {
            use std::process::Stdio;
            let cmd = format!(
                r#"(Get-Item (Get-Command '{}').Source).VersionInfo.ProductVersion"#,
                &path
            );

            let output = Command::new("powershell")
                .arg("-Command")
                .arg(&cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
                .map_err(|_| DriverError::BrowserNotFound(path.clone()))?;

            if !output.status.success() {
                // let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                Err(DriverError::BrowserNotFound(path))
            } else {
                Ok(get_version_from_output(output))
            }
        }
    }
}

fn get_version_from_output(output: Output) -> String {
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
    re.captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string()
}

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("browser not found `{0}`")]
    BrowserNotFound(String),
    #[error("resource not found `{0}`")]
    ResourceNotFound(String),
    #[error("resource invalid `{0}`")]
    ResourceInvalid(String),
    #[error("download request timeout, please increase connect_timeout/timeout or use vpn")]
    RequestTimeout,
    #[error("failed to send request")]
    RequestInvalid,
}

type DriverResult<T> = Result<T, DriverError>;
