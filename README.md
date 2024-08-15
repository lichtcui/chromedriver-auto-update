# chromedriver

Automatically download Chromedriver when browser/driver versions do not match.

# usage

### install

```bash
cargo install chromedriver-update
```

### run

##### --browser-path: chrome browser path
##### --driver-path: chrome driver path, create when file not exist

```bash
# mac
chromedriver-update \
    --browser-path="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
    --driver-path="/usr/local/bin/chromedriver"

# linux
chromedriver-update \
    --browser-path="/usr/bin/google-chrome" \
    --driver-path="/usr/bin/chromedriver"

# windows (only tested in github workflow)
chromedriver-update \
    --browser-path="C:\setup-chrome\chromium\120.0.6099.109\x64\chrome.exe" \
    --driver-path="C:\setup-chrome\chromedriver.exe"
```

# code usage

> require rust >= v1.80

### add package

```bash
cargo add chromedriver-update
```

### code examples

- /examples/default_args.rs
- /examples/custom_args.rs
