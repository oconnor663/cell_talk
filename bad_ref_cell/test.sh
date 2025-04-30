#! /usr/bin/env bash

set -xv

# =================== The UB test fails. ==========================
cargo +1.81.0 test --release

# =================== The should_panic test fails instead! ========
cargo +1.70.0 test --release

# =================== Tests pass! =================================
cargo +1.69.0 test --release
