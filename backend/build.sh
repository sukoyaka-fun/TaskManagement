#!/bin/bash
export RUSTFLAGS="-Z macro-backtrace"
cargo build --release --offline
