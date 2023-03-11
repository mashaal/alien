FROM rust:latest

# USER app
WORKDIR /app
EXPOSE 8080

COPY . .

# # Switch Rust to the nightly bui
# RUN rustup toolchain install nightly

# # RUN cargo +nightly install --path .

# RUN mkdir crust

# # Install the cargo-ament-build plugin
# # RUN cargo +nightly install -Z sparse-registry --debug cargo-ament-build 

# RUN cargo +nightly build -Z unstable-options --out-dir=crust --target x86_64-unknown-linux-musl

RUN useradd -ms /bin/bash app
USER app

CMD ["./target/x86_64-unknown-linux-musl/debug/webcrust"]