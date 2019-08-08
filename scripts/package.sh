#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

cargo package --manifest-path log4rs-cases/Cargo.toml

zcat target/package/log4rs-cases-*.crate | tar tf -
