use std::sync::LazyLock;

pub static CHROME_DRIVER_PATH: LazyLock<String> = LazyLock::new(|| {
    let prefix = match std::env::consts::OS {
        "macos" => "/usr/local/bin",
        "linux" => "/usr/bin",
        "windows" => "",
        _ => "",
    };

    if !prefix.eq("") {
        format!("{}/{}", prefix, DRIVERNAME).to_string()
    } else {
        "".to_string()
    }
});

pub static CHROME_BROWSER_PATH: LazyLock<String> = LazyLock::new(|| {
    match std::env::consts::OS {
        "macos" => "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "linux" => "/usr/bin/google-chrome",
        "windows" => "",
        _ => "",
    }
    .to_string()
});

pub static CONNECT_TIMEOUT: u64 = 5000;
pub static TIMEOUT: u64 = 10000;

static OS: LazyLock<String> = LazyLock::new(|| {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "x86_64") => "mac-x64",
        ("macos", "aarch64") => "mac-arm64",
        ("windows", "x86") => "win32",
        ("windows", "x86_64") => "win64",
        ("linux", "x86_64") => "linux64",
        _ => "",
    }
    .to_string()
});

static DRIVERNAME: &str = "chromedriver";

// chromedriver-mac-x64
static FILENAME: LazyLock<String> = LazyLock::new(|| format!("{}-{}", DRIVERNAME, OS.as_str()));

// chromedriver-mac-x64/chromedriver
pub static DRIVER_FILE: LazyLock<String> =
    LazyLock::new(|| format!("{}/{}", FILENAME.as_str(), DRIVERNAME));

// mac-x64/chromedriver-mac-x64.zip
pub static ZIP_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/{}.zip", OS.as_str(), FILENAME.as_str()));
