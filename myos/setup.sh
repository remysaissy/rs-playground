#!/bin/sh


brew install qemu
rustup toolchain install nightly
rustup component add rust-src
cargo install bootimage
rustup component add llvm-tools-preview
rustup update
