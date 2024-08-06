use regex::Regex;
use std::{
    fs,
    io::{Cursor, Read},
    os::unix::fs::PermissionsExt,
    process::Output,
};
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use url::Url;

mod utils;
use utils::config::LOCAL_CONFIG;

#[tokio::main]
async fn main() {
    version_verify().await;
}

async fn version_verify() {
    let browser_version = get_chrome_version().await;
    println!("browser version: {}", &browser_version);

    let driver_version = get_driver_version().await;
    println!("driver version: {}", &driver_version);

    if driver_version.eq(&browser_version) {
        println!("no need to update");
        return;
    }

    println!("updating driver... ");
    download_driver(browser_version).await;
}

async fn get_chrome_version() -> String {
    let output = Command::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
        .arg("--version")
        .output()
        .await
        .expect("cannot find chrome");

    get_version_from_output(output)
}

async fn get_driver_version() -> String {
    if let Ok(res) = Command::new(&LOCAL_CONFIG.driver.path)
        .arg("--version")
        .output()
        .await
    {
        return get_version_from_output(res);
    }

    return "".to_string();
}

async fn download_driver(version: String) {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(std::time::Duration::from_millis(
            LOCAL_CONFIG.driver.connect_timeout,
        ))
        .timeout(std::time::Duration::from_millis(
            LOCAL_CONFIG.driver.timeout,
        ))
        .build()
        .unwrap();

    let url_text = format!("https://storage.googleapis.com/chrome-for-testing-public/{}/mac-x64/chromedriver-mac-x64.zip", version);
    let url = Url::parse(&url_text).unwrap();

    let bytes = client
        .get(url)
        .send()
        .await
        .expect("request timeout, you may need vpn to update chromedriver")
        .bytes()
        .await
        .unwrap();

    let cursor = Cursor::new(bytes.as_ref());
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_name = file.name();

        if file_name.eq("chromedriver-mac-x64/chromedriver") {
            let mut output_file = File::create(&LOCAL_CONFIG.driver.path).await.unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            output_file.write_all(&buffer).await.unwrap();

            let permissions = fs::metadata(&LOCAL_CONFIG.driver.path)
                .unwrap()
                .permissions();
            let mut new_permissions = permissions.clone();
            new_permissions.set_mode(0o755);
            fs::set_permissions(&LOCAL_CONFIG.driver.path, new_permissions).unwrap();
        }
    }
}

fn get_version_from_output(output: Output) -> String {
    let text = String::from_utf8(output.stdout).unwrap();
    let re = Regex::new(r"\d+\.\d+\.\d+\.\d+").unwrap();
    re.captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string()
}
