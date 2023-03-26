FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:edge
RUN apt-get update -y && apt-get install --assume-yes libsqlite3-dev