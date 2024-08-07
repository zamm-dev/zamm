import subprocess
import sys

SEPARATOR = "warning: `ollama-rs` (lib) generated "

clippy_command = "cargo clippy --color never --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings"
clippy_process = subprocess.Popen(
    clippy_command, shell=True, stderr=subprocess.STDOUT, stdout=subprocess.PIPE
)
clippy_output = clippy_process.communicate()[0].decode()

if SEPARATOR not in clippy_output:
    zamm_output = clippy_output
else:
    separator_line = clippy_output.split(SEPARATOR)[1]
    zamm_output = "\n".join(separator_line.split("\n")[1:])

print(zamm_output)

if "warning" in zamm_output:
    sys.exit(1)
