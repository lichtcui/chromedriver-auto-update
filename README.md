# chromedriver
> mac & linux only
Automatically download Chromedriver when the browser/driver versions do not match.

# usage

### install & run

```bash
cargo install chromedriver-update

# use default values
chromedriver-update

# or use custom values
chromedriver-update --driver-path="/driver/path" --browser-path="/browser/path"
```

### arguments:

use `chromedriver-update --help` to check details (different default value for different os)

#### --driver-path
|os|default_value|
|-|-|
|mac|/usr/local/bin/chromedriver|
|linux|/usr/bin/chromedriver|
|windows||

#### --browser-path
|os|default_value|
|-|-|
|mac|/Applications/Google Chrome.app/Contents/MacOS/Google Chrome|
|linux|/usr/bin/google-chrome|
|windows||

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
