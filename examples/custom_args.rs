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
