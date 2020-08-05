#!/bin/bash

set -e

eval "$(fnm env)"

fnm install 6.11.3
fnm install 8.11.3

fnm alias 8.11.3 oldie
fnm alias 6.11.3 older
fnm default 6.11.3 

VERSIONS_INSTALLED=$(fnm ls)

echo "$VERSIONS_INSTALLED" | grep 8.11.3 | grep oldie
echo "$VERSIONS_INSTALLED" | grep 6.11.3 | grep older
echo "$VERSIONS_INSTALLED" | grep 6.11.3 | grep default

fnm use oldie
EXPECT_OLDIE="$(fnm current)"

if [ "$EXPECT_OLDIE" != "v8.11.3" ]; then
  echo "Expected $EXPECT_OLDIE to be v8.11.3"
  exit 1
fi

fnm use older
EXPECT_OLDER="$(fnm current)"

if [ "$EXPECT_OLDER" != "v6.11.3" ]; then
  echo "Expected $EXPECT_OLDER to be v6.11.3"
  exit 1
fi
