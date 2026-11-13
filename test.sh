#!/usr/bin/env bash

set -e

for file in tests/*.rs; do
    t=$(basename "$file" .rs)
    cargo test --test "$t"
done
