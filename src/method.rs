use crate::protos::bigtable::*;
use protobuf::MessageFull;

// AIDEV-NOTE: protobuf 3.x requires MessageFull for JSON serialization via protobuf-json-mapping
// AIDEV-NOTE: UrlScope distinguishes table-level vs instance-level API methods
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UrlScope {
    /// URL format: /projects/{project}/instances/{instance}/tables/{table}:{method}
    Table,
    /// URL format: /projects/{project}/instances/{instance}:{method}
    Instance,
}

pub trait BigTable {
    type M: MessageFull;

    fn payload(&self) -> &Self::M;
    fn payload_mut(&mut self) -> &mut Self::M;
    fn set_payload(&mut self, payload: Self::M);
    fn url_method(&self) -> &str;
    fn is_post(&self) -> bool;
    /// Returns the URL scope for this method (default: Table)
    fn url_scope(&self) -> UrlScope {
        UrlScope::Table
    }
}

macro_rules! method {
    // Table-level method (default) - auto-generates URL suffix from name
    ($name: ident, $proto: ty, $post: expr) => {
        method!(@impl $name, $proto, $post, UrlScope::Table, {
            let mut x = stringify!($name).chars();
            let first = x.next().unwrap().to_lowercase().next().unwrap();
            let rest = x.as_str();
            format!(":{}{}", first, rest)
        });
    };
    // Method with explicit scope - auto-generates URL suffix from name
    ($name: ident, $proto: ty, $post: expr, $scope: expr) => {
        method!(@impl $name, $proto, $post, $scope, {
            let mut x = stringify!($name).chars();
            let first = x.next().unwrap().to_lowercase().next().unwrap();
            let rest = x.as_str();
            format!(":{}{}", first, rest)
        });
    };
    // Method with explicit scope and custom URL suffix
    ($name: ident, $proto: ty, $post: expr, $scope: expr, $url_suffix: expr) => {
        method!(@impl $name, $proto, $post, $scope, { String::from($url_suffix) });
    };
    // Internal implementation
    (@impl $name: ident, $proto: ty, $post: expr, $scope: expr, $url_method_expr: block) => {
        pub struct $name {
            pub payload: $proto,
            pub url_method: String,
            pub is_post: bool,
            pub scope: UrlScope,
        }

        impl $name {
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    payload: Default::default(),
                    url_method: $url_method_expr,
                    is_post: $post,
                    scope: $scope,
                }
            }
        }

        impl BigTable for $name {
            type M = $proto;

            fn payload(&self) -> &Self::M {
                &self.payload
            }

            fn set_payload(&mut self, payload: $proto) {
                self.payload = payload
            }

            fn payload_mut(&mut self) -> &mut Self::M {
                &mut self.payload
            }

            fn url_method(&self) -> &str {
                &self.url_method
            }

            fn is_post(&self) -> bool {
                self.is_post
            }

            fn url_scope(&self) -> UrlScope {
                self.scope
            }
        }
    };
}

/// ### `ReadRows`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, ReadRows};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: ReadRows::new()
///     };
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn read_rows_doctest() {}
method!(ReadRows, ReadRowsRequest, true);

/// ### `SampleRowKeys`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, SampleRowKeys};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: SampleRowKeys::new()
///     };
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn sample_row_keys_doctest() {}
method!(SampleRowKeys, SampleRowKeysRequest, false);

/// ### `MutateRow`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, MutateRow};
/// use bt::protos::data::{Mutation, mutation};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: MutateRow::new()
///     };
///
///     let row_key = encode_str("r1");
///
///     let mut m = Mutation::new();
///     m.mutation = Some(mutation::Mutation::DeleteFromRow(Default::default()));
///
///     req.method.payload_mut().row_key = row_key;
///     req.method.payload_mut().mutations.push(m);
///
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn mutate_row_doctest() {}
method!(MutateRow, MutateRowRequest, true);

/// ### `MutateRows`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, MutateRows};
/// use bt::protos::data::{Mutation, mutation};
/// use bt::protos::bigtable::mutate_rows_request;
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: MutateRows::new()
///     };
///
///     let row_key = encode_str("r1");
///
///     let mut m = Mutation::new();
///     m.mutation = Some(mutation::Mutation::DeleteFromRow(Default::default()));
///
///     let mut entry = mutate_rows_request::Entry::new();
///     entry.row_key = row_key;
///     entry.mutations.push(m);
///
///     req.method.payload_mut().entries.push(entry);
///
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn mutate_rows_doctest() {}
method!(MutateRows, MutateRowsRequest, true);

/// ### `CheckAndMutateRow`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, CheckAndMutateRow};
/// use bt::protos::data::{RowFilter, Mutation, mutation, row_filter};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: CheckAndMutateRow::new()
///     };
///
///     let row_key = encode_str("r1");
///
///     let mut predicate_filter = RowFilter::new();
///     predicate_filter.filter = Some(row_filter::Filter::PassAllFilter(true));
///
///     let mut m = Mutation::new();
///     m.mutation = Some(mutation::Mutation::DeleteFromRow(Default::default()));
///
///     req.method.payload_mut().row_key = row_key;
///     req.method.payload_mut().predicate_filter = Some(predicate_filter).into();
///     req.method.payload_mut().true_mutations.push(m);
///
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn check_and_mutate_row_doctest() {}
method!(CheckAndMutateRow, CheckAndMutateRowRequest, true);

/// ### `ReadModifyWriteRow`
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::protos::data::ReadModifyWriteRule;
/// use bt::utils::*;
/// use bt::method::{BigTable, ReadModifyWriteRow};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: ReadModifyWriteRow::new()
///     };
///
///     let token = get_auth_token("credentials.json", true)?;
///
///     let mut rule = ReadModifyWriteRule::new();
///     rule.family_name = String::from("cf1");
///     rule.column_qualifier = encode_str("r1");
///     rule.set_append_value(encode_str("test_value"));
///
///     req.method.payload_mut().row_key = encode_str("r1");
///     req.method.payload_mut().rules.push(rule);
///
///     let response = req.execute(&token)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
///     Ok(())
/// }
/// ```
fn read_modify_write_doctest() {}
method!(ReadModifyWriteRow, ReadModifyWriteRowRequest, true);

// AIDEV-NOTE: New methods added in later Bigtable API versions

/// ### `PingAndWarm`
///
/// Warms up connection channels to the service. Recommended to be called
/// periodically to maintain connection readiness.
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, PingAndWarm};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: PingAndWarm::new()
///     };
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     Ok(())
/// }
/// ```
fn ping_and_warm_doctest() {}
// AIDEV-NOTE: PingAndWarm uses instance-level URL with custom suffix ":ping"
method!(PingAndWarm, PingAndWarmRequest, true, UrlScope::Instance, ":ping");

/// ### `GenerateInitialChangeStreamPartitions`
///
/// Generates initial change stream partitions. Primarily for Apache Beam BigtableIO.
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, GenerateInitialChangeStreamPartitions};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: GenerateInitialChangeStreamPartitions::new()
///     };
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     Ok(())
/// }
/// ```
fn generate_initial_change_stream_partitions_doctest() {}
method!(GenerateInitialChangeStreamPartitions, GenerateInitialChangeStreamPartitionsRequest, true);

/// ### `ReadChangeStream`
///
/// Reads changes from a table's change stream. Primarily for Apache Beam BigtableIO CDC.
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, ReadChangeStream};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: ReadChangeStream::new()
///     };
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     Ok(())
/// }
/// ```
fn read_change_stream_doctest() {}
method!(ReadChangeStream, ReadChangeStreamRequest, true);

/// ### `PrepareQuery`
///
/// Prepares a GoogleSQL query for execution. Returns a prepared query that can
/// be used with ExecuteQuery.
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, PrepareQuery};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: PrepareQuery::new()
///     };
///     req.method.payload_mut().query = String::from("SELECT * FROM my_table");
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     Ok(())
/// }
/// ```
fn prepare_query_doctest() {}
// AIDEV-NOTE: PrepareQuery uses instance-level URL
method!(PrepareQuery, PrepareQueryRequest, true, UrlScope::Instance);

/// ### `ExecuteQuery`
///
/// Executes a GoogleSQL query against a Bigtable table.
///
/// ```ignore
/// use bigtable as bt;
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, ExecuteQuery};
/// use bt::error::BTErr;
///
/// fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///         base: None,
///         table: Default::default(),
///         method: ExecuteQuery::new()
///     };
///     req.method.payload_mut().query = String::from("SELECT * FROM my_table");
///     let response = req.execute(&get_auth_token("credentials.json", true)?)?;
///     Ok(())
/// }
/// ```
fn execute_query_doctest() {}
// AIDEV-NOTE: ExecuteQuery uses instance-level URL
method!(ExecuteQuery, ExecuteQueryRequest, true, UrlScope::Instance);
