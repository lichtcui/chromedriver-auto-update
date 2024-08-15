use chromedriver_update::{
    constant::{CHROME_BROWSER_PATH, CHROME_DRIVER_PATH, CONNECT_TIMEOUT, TIMEOUT},
    ChromeDriver,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = CHROME_DRIVER_PATH.as_str())]
    driver_path: String,
    #[arg(
        short,
        long,
        default_value = CHROME_BROWSER_PATH.as_str()
    )]
    browser_path: String,

    #[arg(short, long, default_value_t = CONNECT_TIMEOUT)]
    connect_timeout: u64,
    #[arg(short, long, default_value_t = TIMEOUT)]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    println!(
        "os: {}, arch: {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let args = Args::parse();

    let mut driver = ChromeDriver::new();
    driver
        .set_driver_path(&args.driver_path)
        .set_browser_path(&args.browser_path)
        .set_connect_timeout(args.connect_timeout)
        .set_timeout(args.timeout)
        .init()
        .await
        .unwrap();

    println!("driver version {}", driver.version);
    println!("browser version {}", driver.browser_version);

    if driver.need_download() {
        driver.try_download().await.unwrap();
    }

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
