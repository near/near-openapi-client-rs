#!/bin/bash
set -e

# Generate types and client crates from OpenAPI spec
# Uses Rust codegen binary instead of Python scripts

if [ "$1" = "--no-fetch" ]; then
    cargo run -p near-openapi-codegen -- --no-fetch
else
    cargo run -p near-openapi-codegen
fi
