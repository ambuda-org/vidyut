#!/usr/bin/env python3

"""Fetches training data from GitHub.

We could do this in a shell script, but I find this more readable.
"""

import subprocess
from pathlib import Path


training_data = Path("dcs-data")
if not training_data.exists():
    print(f"Training data folder '{training_data}' does not exist -- fetching.")
    subprocess.check_call(
        f"git clone --depth 1 https://github.com/OliverHellwig/sanskrit.git {training_data}",
        shell=True,
    )
else:
    print(f"Training data folder '{training_data}' exists -- skipping fetch.")
