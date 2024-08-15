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
