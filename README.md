[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/rust-bigtable/blob/master/LICENSE.md)
[![Mentioned in Awesome Bigtable](https://awesome.re/mentioned-badge-flat.svg)](https://github.com/zrosenbauer/awesome-bigtable)

## rust-bigtable [[docs](https://durch.github.io/rust-bigtable/)]

Rust library for working with [Google Bigtable](https://cloud.google.com/bigtable/docs/) [Data API](https://cloud.google.com/bigtable/docs/reference/data/rpc/google.bigtable.v2)

### Supported API Methods

Full coverage of the Bigtable Data API v2:

| Method | Description |
|--------|-------------|
| `ReadRows` | Streams back the contents of all requested rows |
| `SampleRowKeys` | Returns a sample of row keys in the table |
| `MutateRow` | Mutates a row atomically |
| `MutateRows` | Mutates multiple rows in a batch |
| `CheckAndMutateRow` | Mutates a row atomically based on output of a predicate filter |
| `ReadModifyWriteRow` | Modifies a row atomically on the server |
| `PingAndWarm` | Warms up connection channels to the service |
| `GenerateInitialChangeStreamPartitions` | Generates initial change stream partitions |
| `ReadChangeStream` | Reads changes from a table's change stream |
| `PrepareQuery` | Prepares a GoogleSQL query for execution |
| `ExecuteQuery` | Executes a GoogleSQL query against a Bigtable table |

### How It Works

Requests are `protobuf` messages generated from [Google's proto definitions](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto). These messages are converted to JSON and sent to the predefined REST endpoints. Responses are returned as `serde_json::Value`.

Authentication is handled via [goauth](https://crates.io/crates/goauth) with JWT tokens.

### Installation

```toml
[dependencies]
bigtable = "0.6"
```

### Configuration

Provide service account credentials from Google Cloud Console as a JSON key file:

```rust
use bigtable::utils::get_auth_token;

let token = get_auth_token("service-account-key.json", true)?;
```

### Usage

#### High-Level Wrappers

Simple wrappers for common operations:

```rust
use bigtable::utils::get_auth_token;
use bigtable::wraps;
use bigtable::support::Table;

// Read rows with limit
let token = get_auth_token("credentials.json", true)?;
let table = Table::default();
let rows = wraps::read_rows(&table, &token, Some(100))?;

// Bulk write rows (uses MutateRows - higher throughput)
let mut rows = vec![wraps::Row::default()];
wraps::bulk_write_rows(&mut rows, &token, table.clone())?;

// Write rows one at a time (uses ReadModifyWriteRow)
let mut rows = vec![wraps::Row::default()];
wraps::write_rows(&mut rows, &token, &table)?;
```

#### Direct API Access

For full control, use the request builder directly:

```rust
use bigtable::request::BTRequest;
use bigtable::method::{BigTable, ReadRows, MutateRow};
use bigtable::utils::{get_auth_token, encode_str};
use bigtable::protos::data::{Mutation, mutation};

let token = get_auth_token("credentials.json", true)?;

// ReadRows
let mut req = BTRequest {
    base: None,
    table: Default::default(),
    method: ReadRows::new(),
};
req.method.payload_mut().rows_limit = 10;
let response = req.execute(&token)?;

// MutateRow with SetCell
let mut req = BTRequest {
    base: None,
    table: Default::default(),
    method: MutateRow::new(),
};

let mut set_cell = mutation::SetCell::new();
set_cell.family_name = String::from("cf1");
set_cell.column_qualifier = encode_str("col1");
set_cell.timestamp_micros = -1;
set_cell.value = encode_str("value1");

let mut m = Mutation::new();
m.mutation = Some(mutation::Mutation::SetCell(set_cell));

req.method.payload_mut().row_key = encode_str("row1");
req.method.payload_mut().mutations.push(m);

let response = req.execute(&token)?;
```

### Testing

Integration tests run against a live Bigtable instance:

```bash
# Run integration tests (requires credentials)
cargo test --test integration_tests -- --ignored --test-threads=1

# Run doc tests
cargo test
```

### Dependencies

- `protobuf` / `protobuf-json-mapping` - Protocol buffer handling and JSON conversion
- `goauth` / `smpl_jwt` - Google OAuth2 / JWT authentication
- `curl` - HTTP client
- `serde_json` - JSON serialization

### License

MIT
