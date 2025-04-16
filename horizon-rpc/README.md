# Horizon-RPC

A JSON-RPC server that uses the Stellar Horizon API as a data source.

## Overview

This application implements a JSON-RPC API as described in `json-rpc-open-api.json` and forwards requests to the Stellar Horizon API. It serves as a bridge between JSON-RPC clients and the Horizon REST API.

## Features

- JSON-RPC 2.0 compliant server
- Implements methods from the Stellar RPC API specification
- Uses the public Horizon API for Stellar data
- Configurable Horizon endpoint

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)

### Installation

1. Clone the repository
2. Build the project:

```bash
cargo build --release
```

### Usage

Run the server with default settings:

```bash
cargo run --release
```

This will start the JSON-RPC server on `127.0.0.1:8545` using the Stellar testnet Horizon API (`https://horizon-testnet.stellar.org`).

### Command Line Options

- `-b, --bind-address`: The address to bind the JSON-RPC server to (default: `127.0.0.1:8545`)
- `-h, --horizon-url`: The Horizon API server URL (default: `https://horizon-testnet.stellar.org`)

Example with custom settings:

```bash
cargo run --release -- --bind-address 0.0.0.0:8080 --horizon-url https://horizon.stellar.org
```

## Implemented Methods

- `getHealth`: Returns the health status of the server and the connected Horizon instance
- `getNetwork`: Returns information about the Stellar network
- `getLatestLedger`: Returns information about the latest ledger
- `getLedgers`: Returns a list of ledgers based on optional pagination parameters
- `getLedgerEntries`: Placeholder for retrieving ledger entries
- `getEvents`: Placeholder for retrieving events

## Notes

- Some methods (like `getLedgerEntries` and `getEvents`) are currently placeholder implementations
- The application uses the public Horizon API which may have rate limits
- This is a bridge implementation and does not have full JSON-RPC API feature parity yet

## License

Licensed under Apache 2.0