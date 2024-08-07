# chromedriver

Automatically download Chromedriver when the browser/driver versions do not match.

# usage
```no_run
cargo run
```

# code usage
### example with default config

```no_run
use chromedriver_auto_update::ChromeDriver;

let mut driver = ChromeDriver::new();
driver.init().await;

println!("driver version {}", driver.version);
println!("browser version {}", driver.browser_version);

driver.try_download().await;
```

### example with custom config

```no_run
use chromedriver_auto_update::ChromeDriver;

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
