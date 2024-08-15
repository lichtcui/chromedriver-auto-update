use std::sync::LazyLock;

/// Chrome driver path.
/// Default:
/// - macos:    "/usr/local/bin"
/// - linux:    "/usr/bin"
/// - windows:  ""
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

/// Chrome browser path.
/// Default:
/// - macos:    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
/// - linux:    "/usr/bin/google-chrome"
/// - windows:  ""
pub static CHROME_BROWSER_PATH: LazyLock<String> = LazyLock::new(|| {
    match std::env::consts::OS {
        "macos" => "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "linux" => "/usr/bin/google-chrome",
        "windows" => "",
        _ => "",
    }
    .to_string()
});

/// Request connect timeout (ms). Default: 5000
pub static CONNECT_TIMEOUT: u64 = 5000;

/// Request timeout (ms). Default: 10000
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

/// Driver path inside downloaded zip file
/// - unix:      chromedriver-mac-x64/chromedriver
/// - windows:   chromedriver-win64/chromedriver.exe
pub static DRIVER_FILE: LazyLock<String> = LazyLock::new(|| {
    #[cfg(unix)]
    {
        return format!("{}/{}", FILENAME.as_str(), DRIVERNAME);
    }

    #[cfg(windows)]
    {
        return format!("{}/{}.exe", FILENAME.as_str(), DRIVERNAME);
    }
});

/// Zip file path, composed of the operating system type and the file name
/// for example: mac-x64/chromedriver-mac-x64.zip
pub static ZIP_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/{}.zip", OS.as_str(), FILENAME.as_str()));
