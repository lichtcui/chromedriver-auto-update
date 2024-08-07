use chromedriver_auto_update::ChromeDriver;

static DRIVER_PATH: &str = "local/chromedriver";
static BROWSER_PATH: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
static CONNECT_TIMEOUT: u64 = 5000;
static TIMEOUT: u64 = 10000;

#[tokio::main]
async fn main() {
    let mut driver = ChromeDriver::new();
    driver
        .set_driver_path(DRIVER_PATH)
        .set_browser_path(BROWSER_PATH)
        .set_connect_timeout(CONNECT_TIMEOUT)
        .set_timeout(TIMEOUT)
        .init()
        .await;

    println!("version:");
    println!("driver  {}", driver.version);
    println!("browser {}", driver.browser_version);

    driver.try_download().await;
}
