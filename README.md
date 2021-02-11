[![](https://travis-ci.org/durch/rust-bigtable.svg?branch=master)](https://travis-ci.org/durch/rust-bigtable)
[![](http://meritbadge.herokuapp.com/bigtable)](https://crates.io/crates/bigtable)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/rust-bigtable/blob/master/LICENSE.md)
[![Join the chat at https://gitter.im/durch/rust-bigtable](https://badges.gitter.im/durch/rust-bigtable.svg)](https://gitter.im/durch/rust-bigtable?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Mentioned in Awesome Bigtable](https://awesome.re/mentioned-badge-flat.svg)](https://github.com/zrosenbauer/awesome-bigtable)

## rust-bigtable [[docs](https://durch.github.io/rust-bigtable/)]

Rust library for working with [Google Bigtable](https://cloud.google.com/bigtable/docs/) [Data API](https://cloud.google.com/bigtable/docs/reference/data/rpc/google.bigtable.v2)

### Intro
Interface towards Cloud Bigtable, supports all [Data API](https://cloud.google.com/bigtable/docs/reference/data/rpc/google.bigtable.v2) methods. 

+ CheckAndMutateRow
+ MutateRow
+ MutateRows
+ ReadModifyWriteRow
+ ReadRows
+ SampleRowKeys

Includes support for [JWT auth](https://cloud.google.com/docs/authentication):

### How it works

Initial plans was to go full `grpc` over `http/2`, unfortunately Rust support is not there yet, so a middle way was taken :).

Requests objects are `protobuf` messages, generated using `proto` definitions available from [Google](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto). And all configuration is done through very nice interfaces generated in this way. These messages are then transparently converted to `json`, and sent to predefined `google.api.http` endpoints, also defined [here](https://github.com/googleapis/googleapis/blob/master/google/bigtable/v2/bigtable.proto). Responses are returned as [serde_json::Value](https://docs.serde.rs/serde_json/value/index.html).

In theory this should enable easy upgrade to full `grpc` over `http/2` as soon as it becomes viable, the only remaining work would be utilising proper return types, also available as `protobuf` messages.

### Configuration

You can provide the `json` service accounts credentials obtained from Google Cloud Console or the private key file in `pem` or (see [random_rsa_for_testing](https://github.com/durch/rust-bigtable/blob/master/random_rsa_for_tests) for proper format) format as well as Google Cloud service account with proper scopes (scopes are handled by [goauth](https://crates.io/crates/goauth), as part of authentication), 

### Usage

*In your Cargo.toml*

```
[dependencies]
bigtable = '0.3'
```

#### Higher level wrappers (`wraps`)

There and higher wrappers available for reading and writing rows, so there is not need to craft `protobufs` manually. Write can also be used to update, but not very robustly yet, coming soon :).

##### Read and Write

Read wrappers allows for simple limit on the number of rows, it uses the `ReadRows` underlying method.

There are two write strategies available, `bulk_write_rows` and `write_rows`. `bulk_write_rows` first collects all writes and fires only one request, underlying method is `MutateRows`, this results in a much higher write throughput. `write_rows` shoots one request per row to be written, underlying method is `ReadModifyWriteRow`. 

```rust

extern crate bigtable as bt;

use bt::utils::*;
use bt::wraps;

const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
const ISS: &'static str = "some-service-account@developer.gserviceaccount.com";
const PK: &'static str = "pk_for_the_acc_above.pem";

fn read_rows(limit: i64) -> Result<(serde_json::Value), BTErr> {

    let token = get_auth_token(TOKEN_URL, ISS, PK)?;
    let table = Default::default();

    wraps::read_rows(table, &token, Some(limit))

}

fn write_rows(n: usize, bulk: bool) -> Result<(), BTErr> {
    let mut rows: Vec<wraps::Row> = vec!(wraps::Row::default()); // put some real data here
    let token = get_auth_token(TOKEN_URL, ISS, PK)?;
    let table = Default::default(); // Again use a real table here
    if bulk {
        let _ = wraps::bulk_write_rows(&mut rows, &token, table);
    } else {
        let _ = wraps::write_rows(&mut rows, &token, table);
    }
    Ok(())
}
```
