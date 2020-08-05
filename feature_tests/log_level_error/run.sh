#!/bin/bash

set -e

eval "$(fnm --log-level=error env)"
ALIAS="$(fnm install 8.11.3 && (fnm alias 123 abc 2>&1 || true))"

if [ "$ALIAS" == "" ]; then
  echo "Expected the output to contain errors"
  exit 1
fi
