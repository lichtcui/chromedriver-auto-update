FROM rust:latest

RUN apt update
RUN wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN dpkg -i google-chrome-stable_current_amd64.deb || true && apt --fix-broken install -y

WORKDIR /build
COPY Cargo.* .
COPY src src
RUN cargo build --release

## install/update chromedriver
# RUN ./target/release/chromedriver-update
