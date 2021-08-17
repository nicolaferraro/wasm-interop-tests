# wasm-interop-tests

WASM interoperability tests with different languages.

## Prerequisites

Linux:

```
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```
## Building

```
cd connector
cargo +nightly build --target wasm32-unknown-unknown --no-default-features -Z build-std -Z build-std-features=panic_immediate_abort --release
```
