# Webcrust

Build (Stable)

- rustup target add --toolchain nightly x86_64-unknown-linux-musl
- cargo build --release --target=x86_64-unknown-linux-musl

Build (Nightly)

- rustup target add --toolchain nightly x86_64-unknown-linux-musl
- cargo +nightly build -Z unstable-options --release --out-dir=crust --target
  x86_64-unknown-linux-musl
