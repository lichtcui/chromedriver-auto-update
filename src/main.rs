use chromedriver_auto_update::ChromeDriver;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/usr/local/bin/chromedriver")]
    dirver_path: String,
    #[arg(
        short,
        long,
        default_value = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
    )]
    browser_path: String,

    #[arg(short, long, default_value_t = 5000)]
    connect_timeout: u64,
    #[arg(short, long, default_value_t = 10000)]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut driver = ChromeDriver::new();
    driver
        .set_driver_path(&args.dirver_path)
        .set_browser_path(&args.browser_path)
        .set_connect_timeout(args.connect_timeout)
        .set_timeout(args.timeout)
        .init()
        .await;

    println!("version:");
    println!("driver  {}", driver.version);
    println!("browser {}", driver.browser_version);

    driver.try_download().await;
}
