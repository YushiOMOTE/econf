#!/usr/bin/env bash

pushd econf
cargo install cargo-readme --version 3.2.0
cargo readme > ../README.md
popd
