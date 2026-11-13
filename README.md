# HTSlib bindings for Rust

This sub-crate provides the raw HTSlib bindings, which are used for the high level Rust API for reading and writing BAM files provided by rust-htslib.

See [rust-htslib](https://github.com/rust-bio/rust-htslib) for details.

## Cloning
When cloning this repo and updating the binaries, the `htslib` submodule needs to be cloned recursively:

```
git clone --recurse-submodules ...
```

## Updating prebuilt bindings

When updating the htslib submodule or modifying `wrapper.h`, the prebuilt binding files need to be regenerated.

The generated bindings do not depend on cargo features, so `--no-default-features` is used to skip building curl and OpenSSL. A dedicated target directory guarantees exactly one `bindings.rs` is found, and the `find` works with both the older `build/hts-sys-<hash>/out` and newer `build/hts-sys/<hash>/out` cargo layouts.

### macOS

```sh
CARGO_TARGET_DIR=target/bindgen-macos cargo build --no-default-features --features bindgen
cp "$(find target/bindgen-macos -path '*hts-sys*' -name bindings.rs)" osx_prebuilt_bindings.rs
```

### Linux

Generate these on Linux (or in Docker as below), never by copying the macOS bindings: the libc declarations and `va_list` differ between platforms.

```sh
docker run --rm --platform linux/amd64 \
  -v "$(pwd)":/workspace -w /workspace -e CARGO_TARGET_DIR=/tmp/target rust:latest bash -c '
  set -euo pipefail
  rustup component add rustfmt
  apt-get update -qq
  apt-get install -y -qq cmake clang libc6-dev
  cargo build --no-default-features --features bindgen
  mapfile -t files < <(find /tmp/target -path "*hts-sys*" -name bindings.rs)
  [ "${#files[@]}" -eq 1 ] || { echo "expected one bindings.rs, found ${#files[@]}"; exit 1; }
  rustfmt --edition 2021 "${files[0]}"
  cp "${files[0]}" linux_prebuilt_bindings.rs
'
```
