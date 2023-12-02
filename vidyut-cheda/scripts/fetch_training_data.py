#!/usr/bin/env python3

"""
Fetches training data from GitHub.

We could do this in a shell script, but I find this more readable.
"""

import subprocess
from pathlib import Path


training_dir = Path("dcs-data")

if training_dir.exists():
    print(f"Training data folder '{training_dir}' exists -- skipping fetch.")
else:
    print(f"Training data folder '{training_dir}' does not exist -- fetching.")

    repo_link = "https://github.com/OliverHellwig/sanskrit.git"
    subprocess.check_call(f"git clone --depth 1 {repo_link} {training_dir}",
                          shell=True)
