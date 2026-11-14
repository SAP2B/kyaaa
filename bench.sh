#!/usr/bin/env bash

set -e

for file in benches/*.rs; do
    bench=$(basename "$file" .rs)
    cargo bench --bench "$bench"
done
