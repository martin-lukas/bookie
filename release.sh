#!/usr/bin/env bash
set -e

echo "==> Building release..."
cargo build --release

echo "==> Copying binary to project root..."
cp target/release/bookie .

echo "==> Release ready: ./bookie"

