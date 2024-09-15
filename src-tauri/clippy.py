import subprocess
import sys


def cut_off_separate(output: str, separator: str) -> str:
    if separator not in output:
        return output
    separator_line = output.split(separator)[1]
    return "\n".join(separator_line.split("\n")[1:])


clippy_command = "cargo clippy --color never --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings"
clippy_process = subprocess.Popen(
    clippy_command, shell=True, stderr=subprocess.STDOUT, stdout=subprocess.PIPE
)
clippy_output = clippy_process.communicate()[0].decode()

zamm_output = cut_off_separate(
    clippy_output, "warning: `async-openai` (lib) generated "
)
zamm_output = cut_off_separate(zamm_output, "warning: `ollama-rs` (lib) generated ")

print(zamm_output)

if "warning" in zamm_output:
    sys.exit(1)
