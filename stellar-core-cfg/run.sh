#!/usr/bin/env bash

tmux \
    split-window  'stellar-core run --conf mainnet.cfg' \; \
    split-window "while ((http :11626/info | jq -r '.info.status') || true); do sleep 1; done" \;
