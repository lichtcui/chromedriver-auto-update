# chromedriver-auto-update

auto download chromedriver when browser/driver version not matched

## Usage

1. copy config file
```bash
cp ./local/config.toml.example ./local/config.toml
```

2. edit config file
```toml
[driver]
path = "local/chromedriver"          # local path
# path = "/usr/local/bin/chromedriver" # This is the path we use on Mac.

# Increase these settings if you are frequently experiencing timeouts, or consider using a VPN.
connect_timeout = 5000               # download url connect timeout
timeout = 10000                      # download total timeout
```

3. run
```bash
cargo run

# or

cargo build --release
./target/release/chromedriver-auto-update
```
