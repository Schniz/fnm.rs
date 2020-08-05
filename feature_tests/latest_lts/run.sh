#!/bin/bash

set -e

eval "$(fnm env --multi)"

fnm install
fnm use

ALL_VERSIONS="$(fnm ls)"

echo "$ALL_VERSIONS"

echo "$ALL_VERSIONS" | grep lts-latest > /dev/null
