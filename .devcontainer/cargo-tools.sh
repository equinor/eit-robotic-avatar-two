#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

cd ~
wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
mv ./trunk /usr/local/bin/