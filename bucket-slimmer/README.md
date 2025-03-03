# Bucket WASM Extractor

## Overview

The Bucket WASM Extractor is a tool for extracting WebAssembly (WASM) code from Stellar bucket files. It processes the bucket files and extracts the contract code, saving it as individual WASM files in the current directory.

Note that in a history archive buckets contain some duplicate entries with older buckets holding old copies of data that's been updated or deleted. Therefore to accurately understand a ledger one must process all buckets in order. However, this utility does not do that because contract code is stored in buckets uniquely based on its sha256 hash, and so it cannot be updated.

## Install

```
cargo install --git https://github.com/leighmcculloch/stellar/bucket-wasm-extractor --branch main
```

## Usage

Collect bucket files either from Stellar History Archives, or by running stellar-core:

```
stellar-core run
```

After the node has caught up, run the `bucket-wasm-extractor` command with the path to the bucket files to process:

```sh
$ bucket-wasm-extractor bucket-1.xdr bucket-2.xdr ...
```

This command will extract any Wasm contracts in the files and write them to the current directory as .wasm files.
