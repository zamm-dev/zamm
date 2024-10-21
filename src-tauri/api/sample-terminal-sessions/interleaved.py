#!/usr/bin/env python3

import sys

print("stdout")
sys.stdout.flush()
print("stderr", file=sys.stderr)
print("stdout")
