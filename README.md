# chromedriver

Automatically download Chromedriver when browser/driver versions do not match.

# usage

### install & run

```bash
cargo install chromedriver-update

# mac
chromedriver-update --browser-path="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --driver-path="/usr/local/bin/chromedriver"

# linux
chromedriver-update --browser-path="/usr/bin/google-chrome" --driver-path="/usr/bin/chromedriver"

# windows (only tested in github workflow)
chromedriver-update --browser-path="C:\setup-chrome\chromium\120.0.6099.109\x64\chrome.exe" --driver-path="C:\setup-chrome\chromedriver.exe"
```

# code usage

> require rust >= v1.80

add package

```shell
cargo add chromedriver-update
```

### example with default config

```rust
use chromedriver_update::ChromeDriver;

let mut driver = ChromeDriver::new();
driver.init().await.unwrap();

println!("driver version {}", driver.version);
println!("browser version {}", driver.browser_version);

driver.try_download().await.unwrap();
```

### example with custom config

```rust
use chromedriver_update::ChromeDriver;

let mut driver = ChromeDriver::new();
driver
  .set_driver_path("/other/path")
  .set_browser_path("/other/path")
  .set_connect_timeout(1000)
  .set_timeout(2000)
  .init()
  .await
  .unwrap();

println!("driver version {}", driver.version);
println!("browser version {}", driver.browser_version);

driver.try_download().await.unwrap();
```
