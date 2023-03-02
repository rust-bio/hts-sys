# Changelog

## 0.41.0 (2023-03-02)


### ⚠ BREAKING CHANGES

* dummy major version bump to move away from previous versions that were following htslib versions.
* bump to new major version (for technical reasons).
* dummy breaking change to increase hts-sys major version.
* Improve bcf Record filter interface and improve docs ([#306](https://github.com/rust-bio/hts-sys/issues/306))

### Features

* add musl target support ([#111](https://github.com/rust-bio/hts-sys/issues/111)) ([1f9736e](https://github.com/rust-bio/hts-sys/commit/1f9736ef108762e919802b3d78e22e47533c122c))
* Add wrapper of BGZF writer ([#349](https://github.com/rust-bio/hts-sys/issues/349)) ([965ed88](https://github.com/rust-bio/hts-sys/commit/965ed886a0c24ee3070e48cc192c0772ac5cbaf4))
* Improve bcf Record filter interface and improve docs ([#306](https://github.com/rust-bio/hts-sys/issues/306)) ([f45e91d](https://github.com/rust-bio/hts-sys/commit/f45e91dfdc64ecb662d676f2996ed4f14c079995))
* Revised calculation of leading- and trailing-softclips ([#375](https://github.com/rust-bio/hts-sys/issues/375)) ([b61dd2c](https://github.com/rust-bio/hts-sys/commit/b61dd2cfb2b74c0180f2d76bbd4ed4eb14fa09b3))


### Bug Fixes

* add ID to automatic release handling ([1244393](https://github.com/rust-bio/hts-sys/commit/124439300e1e3e01e1d847f7549747d560c01989))
* bump hts-sys version to 2.0.1 ([336c8b8](https://github.com/rust-bio/hts-sys/commit/336c8b8a1779422afea1065e37bcc44f54abac42))
* bump to new major version (for technical reasons). ([9c6db30](https://github.com/rust-bio/hts-sys/commit/9c6db3060818692070db1411d63e113dc7effd64))
* change the type to c_char so it can be compiled for aarch64 ([#337](https://github.com/rust-bio/hts-sys/issues/337)) ([a21aff2](https://github.com/rust-bio/hts-sys/commit/a21aff289bc03c7549afc7a958084ed57e8c93f2))
* Configuration when cross-compiling. Even when cross-compiling, build.rs runs on the build host. Hence within build.rs `#[cfg(target_os)]` always reflects the host, not the target. Use $CARGO_CFG_TARGET_OS instead to query target properties. ([#329](https://github.com/rust-bio/hts-sys/issues/329)) ([d5198e6](https://github.com/rust-bio/hts-sys/commit/d5198e6c777fdbbfdd9c73a820f1be983a458ce2))
* do not use ret for limiting integer and float arrays coming from yinfo tag ([47126ab](https://github.com/rust-bio/hts-sys/commit/47126ab69e26616df1d401ee11423e150e118e0c))
* dummy breaking change to increase hts-sys major version. ([93415cb](https://github.com/rust-bio/hts-sys/commit/93415cbb82e4f11d257a2b2cedba2664f86a034d))
* dummy changes ([3af5ede](https://github.com/rust-bio/hts-sys/commit/3af5ede13a6b44ce5d1e7f0eb90836a692e711ec))
* dummy fix for triggering release ([e92e6b1](https://github.com/rust-bio/hts-sys/commit/e92e6b10b0a7e5db50b12e2fbe2c42b467eb369e))
* dummy major version bump to move away from previous versions that were following htslib versions. ([aaa70a8](https://github.com/rust-bio/hts-sys/commit/aaa70a85ef9a908d3b101f23879189e84a15d23f))
* dummy release ([74d1565](https://github.com/rust-bio/hts-sys/commit/74d1565329fc862f1172c0925c7b66ceb8bcf988))
* dummy release ([af2f84e](https://github.com/rust-bio/hts-sys/commit/af2f84eb0411507f8866b3cc05e9a6ba9d81d172))
* dummy release ([b97915f](https://github.com/rust-bio/hts-sys/commit/b97915f2c70da4c914f2e69861bf78eec5979baf))
* enum name usage in doc example ([#311](https://github.com/rust-bio/hts-sys/issues/311)) ([6e9ba49](https://github.com/rust-bio/hts-sys/commit/6e9ba4928b60c3105490a8179d074c705ea06fd7))
* handle subcrate with release-please ([0a4605f](https://github.com/rust-bio/hts-sys/commit/0a4605f165cb2edf4428d8fb39f7e4787585f4e1))
* Header::to_hashmap skips `@CO` tags, add `comments()`  method ([#363](https://github.com/rust-bio/hts-sys/issues/363)) ([c24a7f6](https://github.com/rust-bio/hts-sys/commit/c24a7f69fbe5d2db4a6f1fbd6eda3922fe7f1c18))
* improved documentation ([cb0b66c](https://github.com/rust-bio/hts-sys/commit/cb0b66c4a92d4f03debe38dfb2a014b154c7dd96))
* perform checkout before running release please ([cbc6a0a](https://github.com/rust-bio/hts-sys/commit/cbc6a0ad37c5623d14f2ed0bcbb4c5289d012fcb))
* set path in release-please config ([d8f7c6e](https://github.com/rust-bio/hts-sys/commit/d8f7c6e8f31accb7576e150fa1439e177f7816cb))
* trigger dummy release ([7c5a7de](https://github.com/rust-bio/hts-sys/commit/7c5a7de33e2a92052126e5f44389d421974d1e02))
* update changelog ([deef08f](https://github.com/rust-bio/hts-sys/commit/deef08feb0b5ba2d8abf98f2cc6d327236da8aef))
* update to latest release-please ([b130634](https://github.com/rust-bio/hts-sys/commit/b130634b3d096e620dcfe59acae2200df3e4d847))


### Performance Improvements

* update htslib and corresponding bindings to 1.16 ([#366](https://github.com/rust-bio/hts-sys/issues/366)) ([f597ce0](https://github.com/rust-bio/hts-sys/commit/f597ce0451e3f3c393166a7291486bbc2bde4c39))


### Miscellaneous Chores

* release 0.41.0 ([115d5e3](https://github.com/rust-bio/hts-sys/commit/115d5e3a2c7ebc0dd6e5b67378843309dcf72cf0))

## [2.0.3](https://github.com/rust-bio/rust-htslib/compare/hts-sys-v2.0.2...hts-sys-v2.0.3) (2022-10-13)


### Performance Improvements

* update htslib and corresponding bindings to 1.16 ([#366](https://github.com/rust-bio/rust-htslib/issues/366)) ([f597ce0](https://github.com/rust-bio/rust-htslib/commit/f597ce0451e3f3c393166a7291486bbc2bde4c39))

### [2.0.2](https://www.github.com/rust-bio/rust-htslib/compare/hts-sys-v2.0.1...hts-sys-v2.0.2) (2021-07-17)


### Bug Fixes

* Configuration when cross-compiling. Even when cross-compiling, build.rs runs on the build host. Hence within build.rs `#[cfg(target_os)]` always reflects the host, not the target. Use $CARGO_CFG_TARGET_OS instead to query target properties. ([#329](https://www.github.com/rust-bio/rust-htslib/issues/329)) ([d5198e6](https://www.github.com/rust-bio/rust-htslib/commit/d5198e6c777fdbbfdd9c73a820f1be983a458ce2))

### [2.0.1](https://www.github.com/rust-bio/rust-htslib/compare/hts-sys-v2.0.0...hts-sys-v2.0.1) (2021-07-06)


### Bug Fixes

* dummy release ([74d1565](https://www.github.com/rust-bio/rust-htslib/commit/74d1565329fc862f1172c0925c7b66ceb8bcf988))
* dummy release ([af2f84e](https://www.github.com/rust-bio/rust-htslib/commit/af2f84eb0411507f8866b3cc05e9a6ba9d81d172))

## [2.0.0](https://www.github.com/rust-bio/rust-htslib/compare/hts-sys-v1.0.0...hts-sys-v2.0.0) (2021-07-06)


### ⚠ BREAKING CHANGES

* dummy major version bump to move away from previous versions that were following htslib versions.

### Bug Fixes

* dummy major version bump to move away from previous versions that were following htslib versions. ([aaa70a8](https://www.github.com/rust-bio/rust-htslib/commit/aaa70a85ef9a908d3b101f23879189e84a15d23f))

## [1.0.0](https://www.github.com/rust-bio/rust-htslib/compare/hts-sys-v0.1.0...hts-sys-v1.0.0) (2021-07-06)


### ⚠ BREAKING CHANGES

* bump to new major version (for technical reasons).
* dummy breaking change to increase hts-sys major version.

### Bug Fixes

* bump to new major version (for technical reasons). ([9c6db30](https://www.github.com/rust-bio/rust-htslib/commit/9c6db3060818692070db1411d63e113dc7effd64))
* dummy breaking change to increase hts-sys major version. ([93415cb](https://www.github.com/rust-bio/rust-htslib/commit/93415cbb82e4f11d257a2b2cedba2664f86a034d))
* dummy changes ([3af5ede](https://www.github.com/rust-bio/rust-htslib/commit/3af5ede13a6b44ce5d1e7f0eb90836a692e711ec))
* update changelog ([deef08f](https://www.github.com/rust-bio/rust-htslib/commit/deef08feb0b5ba2d8abf98f2cc6d327236da8aef))

## [2.0.0](https://www.github.com/rust-bio/rust-htslib/compare/hts-sys-v1.11.1-fix1...hts-sys-v2.0.0) -  (2021-07-06)


### Bug Fixes

* dummy release ([b97915f](https://www.github.com/rust-bio/rust-htslib/commit/b97915f2c70da4c914f2e69861bf78eec5979baf))
* trigger dummy release ([7c5a7de](https://www.github.com/rust-bio/rust-htslib/commit/7c5a7de33e2a92052126e5f44389d421974d1e02))


## 0.1.0 - 2021-07-06

### Bug Fixes

* dummy release
