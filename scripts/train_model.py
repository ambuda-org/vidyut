#!/usr/bin/env python3

"""A simple training script.

Most of the heavy lifting is done in Rust. We use this script just to fetch the
data directory from GitHub.
"""

import subprocess
from pathlib import Path


training_data = Path("dcs-data")
if not training_data.exists():
    print(f"Training data folder '{training_data}' does not exist -- fetching.")
    subprocess.check_call(
        f"git clone git@github.com:OliverHellwig/sanskrit.git {training_data}",
        shell=True,
    )
else:
    print(f"Training data folder '{training_data}' exists -- skipping fetch.")

subprocess.check_call("cargo run --release --bin train", shell=True)
