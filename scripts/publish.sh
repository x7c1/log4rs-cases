#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

cargo publish --manifest-path log4rs-cases/Cargo.toml
