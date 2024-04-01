#!/usr/bin/env bash

# https://stackoverflow.com/questions/31389483/find-and-delete-file-or-folder-older-than-x-days

if [ "$(uname)" == "Darwin" ]; then
    ulimit -n 1000000
    # sudo mdutil -a -i off
    tmutil thinlocalsnapshots / # if not enough: tmutil thinlocalsnapshots / 500000000000 4
fi

cargo run -r
