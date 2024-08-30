# chromedriver-update

Automatically download Chromedriver when browser/driver versions do not match.

[![Documentation](https://docs.rs/chromedriver-update/badge.svg?style=flat-square)](https://docs.rs/chromedriver-update)
[![License](https://img.shields.io/crates/l/chromedriver-update.svg?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/d/chromedriver-update.svg?style=flat-square)](https://crates.io/crates/chromedriver-update)
[![Crates.io](https://img.shields.io/crates/v/chromedriver-update.svg?style=flat-square)](https://crates.io/crates/chromedriver-update)

## Getting start with cmd

### install

```bash
cargo install chromedriver-update
```

### params

- `--browser-path`: Chrome 浏览器路径
- `--driver-path`: Chromedriver 路径（如果文件不存在，将会创建）

### example

```bash
# mac
chromedriver-update \
    --browser-path="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
    --driver-path="/usr/local/bin/chromedriver"

# linux
chromedriver-update \
    --browser-path="/usr/bin/google-chrome" \
    --driver-path="/usr/bin/chromedriver"

# windows
chromedriver-update \
    --browser-path="C:\setup-chrome\chromium\120.0.6099.109\x64\chrome.exe" \
    --driver-path="C:\setup-chrome\chromedriver.exe"
```

## Getting start with code

> require rust >= v1.80

### add dependency

```bash
cargo add chromedriver-update
```

### examples

- /examples/default_args.rs
```rust
use chromedriver_update::ChromeDriver;

#[tokio::main]
async fn main() {
    let mut driver = ChromeDriver::new();
    driver.init().await.unwrap();

    println!("driver version {}", driver.version);
    println!("browser version {}", driver.browser_version);

    if driver.need_download() {
        driver.try_download().await.unwrap();
    }
}
```

- /examples/custom_args.rs
```rust
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
