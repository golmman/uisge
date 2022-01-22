#!/bin/bash

set -e
set -x

cargo test
cargo check
cargo clippy
cargo +nightly fmt --check
