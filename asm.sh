#!/usr/bin/env bash
set -e

for file in examples/*_proof*.rs; do
    if [ -f "$file" ]; then
        filename=$(basename -- "$file")
        example_name="${filename%.rs}"

        echo "===================================================="
        echo "Showing Assembly for: $example_name"
        echo "===================================================="

        cargo asm -p kyaaa --example "$example_name" --release --target x86_64-unknown-none kmain
        
        echo -e "\n"
    fi
done
