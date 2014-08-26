#!/bin/sh
# A simple install script for development purposes
cp ./versions.toml ~/.rustv/versions.toml
cp ./target/rustv ./target/rustv-build /usr/local/bin
