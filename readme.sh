#!/usr/bin/env bash

pushd econf
cargo install cargo-readme
cargo readme > ../README.md
popd
