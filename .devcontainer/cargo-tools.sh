#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

cd ~

# Tooling for webasembely deveopment.
rustup target add wasm32-unknown-unknown
wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
mv ./trunk /usr/local/bin/

# Minion robot needs Python.
apt update
apt -y install python3-dev