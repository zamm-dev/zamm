#!/usr/bin/bash

clippy_output=$(cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings 2>&1)
zamm_output=$(echo "$clippy_output" | awk '/Checking zamm /{flag=1; next} flag')
echo "$zamm_output"

if [[ $zamm_output == *"warning"* ]]; then
  exit 1;
fi
