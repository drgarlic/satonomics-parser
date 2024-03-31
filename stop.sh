#!/usr/bin/env bash

if [ "$(uname)" == "Darwin" ]; then
    # sudo mdutil -a -i on
fi

bitcoin-cli -datadir=/Users/k/Developer/bitcoin stop
