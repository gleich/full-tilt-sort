# Contributing Guide

Thank you so much for showing an interest in contributing to full-tilt-sort! I am excited to see what you have to add :)

## 🦀 Rust Toolchain

full-tilt-sort uses the nightly distribution of the rust toolchain. You can get on the nightly edition of rustfmt, the only toolchain component used by full-tilt-sort, by running the following command:

```bash
rustup component add rustfmt --toolchain nightly
```

## 🏗️ Build System

full-tilt-sort uses [cargo-make](https://github.com/sagiegurari/cargo-make) for all build scripts. Please run the following command to install cargo-make:

```bash
cargo install --force cargo-make
```

Once you have cargo-make installed you can run `cargo make TASK_NAME` to run a certain task. All tasks are prefixed with `tasks.` and are stored in the [Makefile.toml](Makefile.toml) file. So if you wanted to build the development binary for example you would run the following command:

```bash
cargo make build-rust-dev
```

## 🧪 Linters

full-tilt-sort only uses two linters: [hadolint](https://github.com/hadolint/hadolint) and [rustfmt](https://github.com/rust-lang/rustfmt). hadolint will lint the dockerfiles stored in [docker/](docker/) and rustfmt will lint the source code in [src/](src/). Please install hadolint using your system's package manager and rustfmt with the toolchain command provided in the [🦀 Rust Toolchain section](#-rust-toolchain).

## 🔄 File Syncing

full-tilt-sort uses a GitHub action called [gh_fsync](https://github.com/Matt-Gleich/gh_fsync) to automatically sync certain files from other repositories. The configuration for gh_fsync is stored in [fsync.yml](fsync.yml). This all means that if you update a file in this repository before removing it from the configuration then gh_fsync will reset it back. If you are looking to change a file for full-tilt-sort that is listed in the configuration please remove it from the configuration or update the source file.

---

Thats all, good luck on your contribution and please make an issue if anything doesn't make sense.
