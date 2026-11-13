#!/usr/bin/env bash
set -e

echo "===================================================="
echo "Running Kani Verification..."
echo "===================================================="

cargo kani --package math_proof --solver cadical -Z unstable-options --export-json mproof.json

echo -e "\nVerification completed successfully!"
