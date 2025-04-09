#!/usr/bin/env bash

stellar-core run --conf testnet.cfg --metadata-output-stream fd:1 \
    | stellar xdr decode --input stream-framed --type LedgerCloseMeta
