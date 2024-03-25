#!/usr/bin/env python3
import os
import subprocess
import sys

# Directory paths
relative_base_dir = os.environ.get("SCREENSHOTS_BASE_DIR", "screenshots/")
base_dir = os.path.abspath(relative_base_dir)
baseline_dir = f"{base_dir}/baseline/"
test_dir = f"{base_dir}/testing/actual/"
diff_dir = f"{base_dir}/testing/diff/"

# Check if test_dir exists
if not os.path.isdir(test_dir):
    print("No faulty screenshots at", test_dir)
    sys.exit()

# Traverse directory
for dirpath, dirnames, filenames in os.walk(test_dir):
    # Check each file
    for filename in filenames:
        # Create corresponding baseline and diff file paths
        base_file = os.path.join(
            baseline_dir, os.path.relpath(dirpath, test_dir), filename
        )
        diff_file = os.path.join(
            diff_dir,
            os.path.relpath(dirpath, test_dir),
            filename.rsplit(".", 1)[0] + "-diff.png",
        )

        # Make sure the output directory exists
        os.makedirs(os.path.dirname(diff_file), exist_ok=True)

        # Create and execute compare command
        compare_cmd = "compare {} {} {}".format(
            base_file, os.path.join(dirpath, filename), diff_file
        )
        subprocess.call(compare_cmd, shell=True)
