#!/usr/bin/env bash
if command -v cargo build >/dev/null 2>&1; then
    echo "cargo build found. Building..."
    cargo build

    echo "Done. Installing it to /usr/local/bin/..."
    sudo mv target/debug/plz /usr/local/bin/
else
    echo "cargo build command not found"
    exit 1
fi