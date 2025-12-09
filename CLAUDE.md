# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rust-bigtable is a Rust library for interfacing with Google Cloud Bigtable Data API. It uses protobuf messages converted to JSON for HTTP requests (not full gRPC) with JWT-based authentication via `goauth`.

## Build Commands

```bash
# Build the library (also generates protobuf code via build.rs)
cargo build

# Run tests (doc tests only - no separate test files)
cargo test

# Check compilation without building
cargo check
```

## Architecture

### Code Generation
- `build.rs` generates Rust code from `.proto` files in `protos/` into `src/protos/`
- Proto sources: `protos/google/bigtable/v2/bigtable.proto`, `data.proto`, and `protos/google/rpc/status.proto`

### Core Modules

- **`method.rs`**: Defines the `BigTable` trait and method structs via the `method!` macro. Each Bigtable API method (ReadRows, MutateRow, MutateRows, etc.) is a struct wrapping its protobuf request type.

- **`request.rs`**: `BTRequest<T: BigTable>` is the main request executor. It:
  1. Forms URLs from `Table` hierarchy (Project → Instance → Table)
  2. Converts protobuf payloads to JSON via `protobuf_json_temp`
  3. Executes HTTP requests via `curl`
  4. Returns responses as `serde_json::Value`

- **`support.rs`**: Defines `Project`, `Instance`, and `Table` structs that form the table path hierarchy.

- **`wraps.rs`**: Higher-level wrappers (`read_rows`, `write_rows`, `bulk_write_rows`) that abstract away protobuf message construction.

- **`utils.rs`**: Authentication helpers (`get_auth_token`) and encoding utilities (`encode_str` for base64).

- **`error.rs`**: Unified `BTErr` error type aggregating errors from goauth, curl, serde, protobuf, jwt, and utf8.

### Request Flow
1. Create a `Table` (with `Instance` and `Project`)
2. Get auth token via `get_auth_token(credentials_path, is_file_path)`
3. Build `BTRequest` with table and method (e.g., `ReadRows::new()`)
4. Configure payload via `method.payload_mut().set_*()`
5. Execute with `req.execute(&token)` → returns `serde_json::Value`

### Dependencies
- `goauth` / `smpl_jwt`: Google OAuth2 / JWT authentication
- `protobuf` / `protobuf-json-temp`: Protobuf message handling and JSON conversion
- `curl`: HTTP client
- `serde_json`: JSON serialization
