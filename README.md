# chromedriver

Automatically download Chromedriver when the browser/driver versions do not match.

# usage

### build & run with arguments
```bash
cargo build --release
./target/release/chromedriver-auto-update --dirver-path="/usr/local/bin/chromedriver"
```

### no build && with arguments
```
cargo run -- --dirver-path="/usr/local/bin/chromedriver"
```

### arguments:
```txt
Options:
  -d, --dirver-path <DIRVER_PATH>
          [default: /usr/local/bin/chromedriver]
  -b, --browser-path <BROWSER_PATH>
          [default: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"]
  -c, --connect-timeout <CONNECT_TIMEOUT>
          [default: 5000]
  -t, --timeout <TIMEOUT>
          [default: 10000]
  -h, --help
          Print help
  -V, --version
          Print version
```

# code usage
### example with default config

```rust
use chromedriver_update::ChromeDriver;

let mut driver = ChromeDriver::new();
driver.init().await;

println!("driver version {}", driver.version);
println!("browser version {}", driver.browser_version);

driver.try_download().await;
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
  .await;

println!("driver version {}", driver.version);
println!("browser version {}", driver.browser_version);

driver.try_download().await;
```
