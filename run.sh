#!/usr/bin/env bash

# https://stackoverflow.com/questions/31389483/find-and-delete-file-or-folder-older-than-x-days

# Trap ctrl-c and call ctrl_c()
# trap ctrl_c INT

# function ctrl_c() {
    # echo "** Trapped CTRL-C"

    # For Mac OS users
    # if [ "$(uname)" == "Darwin" ]; then
    #     if mdutil -s / | grep "disabled"; then
    #         echo "Re-enabling spotlight indexing..."
    #         sudo mdutil -a -i on
    #     fi
    # fi
# }

# For Mac OS users
if [ "$(uname)" == "Darwin" ]; then
    echo "Increasing limit of opened files..."
    ulimit -n 1000000

    if mdutil -s / | grep "enabled"; then
        echo "Disabling spotlight indexing..."
        sudo mdutil -a -i off &>/dev/null
    fi

    echo "Cleaning local TimeMachine snapshots..."
    # If not enough: tmutil thinlocalsnapshots / 500000000000 4
    tmutil thinlocalsnapshots / &>/dev/null
fi

cargo run -r
