#!/usr/bin/env bash

set -e

for file in examples/*.rs; do
    ex=$(basename "$file" .rs)
    
    echo -e "\n\033[1;36m┌────────────────────────────────────────────────────────┐\033[0m"
    echo -e "\033[1;36m│\033[0m \033[1;32mEXECUTING EXAMPLE:\033[0m \033[1;37m$ex\033[0m"
    echo -e "\033[1;36m└────────────────────────────────────────────────────────┘\033[0m\n"
    
    cargo run --example "$ex" --release --target x86_64-unknown-none
done
