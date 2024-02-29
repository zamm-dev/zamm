import subprocess
import sys

clippy_command = "cargo clippy --color never --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings"
clippy_process = subprocess.Popen(
    clippy_command, shell=True, stderr=subprocess.STDOUT, stdout=subprocess.PIPE
)
clippy_output = clippy_process.communicate()[0].decode()

checking_zamm_output = clippy_output.split("Checking zamm")[1]
zamm_output = "\n".join(checking_zamm_output.split("\n")[1:])

print(zamm_output)

if "warning" in zamm_output:
    sys.exit(1)
