# HTSlib bindings for Rust

This sub-crate provides the raw HTSlib bindings, which are used for the high level Rust API for reading and writing BAM files provided by rust-htslib.

See [rust-htslib](https://github.com/rust-bio/rust-htslib) for details.

## Updating prebuilt bindings

When updating the htslib submodule or modifying `wrapper.h`, the prebuilt binding files need to be regenerated.

### macOS

```sh
cargo build --features bindgen
cp target/debug/build/hts-sys-*/out/bindings.rs osx_prebuilt_bindings.rs
```

### Linux

```sh
docker run --rm --platform linux/amd64 \
  -v "$(pwd)":/workspace -w /workspace rust:latest bash -c '
  rustup component add rustfmt &&
  apt-get update -qq &&
  apt-get install -y -qq cmake zlib1g-dev libbz2-dev clang libc6-dev libssl-dev libcurl4-openssl-dev &&
  cargo build --features bindgen &&
  BINDINGS=$(find target/debug/build/hts-sys-*/out -name bindings.rs | head -1) &&
  rustfmt "$BINDINGS" &&
  cp "$BINDINGS" linux_prebuilt_bindings.rs
'
```
