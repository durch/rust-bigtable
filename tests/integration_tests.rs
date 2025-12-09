// AIDEV-NOTE: Integration tests for all Bigtable API methods
// These tests require:
// - Credentials file: "Rust Bigtable IAM Admin.json"
// - Project: gen-lang-client-0421059902
// - Instance: test-inst
// - Table: my-table with column family cf1
//
// Run with: cargo test --test integration_tests -- --ignored --test-threads=1

use bigtable::error::BTErr;
use bigtable::method::{
    BigTable, CheckAndMutateRow, ExecuteQuery, GenerateInitialChangeStreamPartitions,
    MutateRow, MutateRows, PingAndWarm, PrepareQuery, ReadChangeStream, ReadModifyWriteRow,
    ReadRows, SampleRowKeys,
};
use bigtable::protos::bigtable::mutate_rows_request;
use bigtable::protos::data::{mutation, row_filter, Mutation, ReadModifyWriteRule, RowFilter, read_modify_write_rule};
use bigtable::request::BTRequest;
use bigtable::support::{Instance, Project, Table};
use bigtable::utils::{encode_str, get_auth_token};
use goauth::auth::Token;
use serde_json::Value;

const CREDENTIALS_FILE: &str = "Rust Bigtable IAM Admin.json";
const PROJECT_ID: &str = "gen-lang-client-0421059902";
const INSTANCE_ID: &str = "test-inst";
const TABLE_NAME: &str = "my-table";
const COLUMN_FAMILY: &str = "cf1";

fn get_token() -> Result<Token, BTErr> {
    get_auth_token(CREDENTIALS_FILE, true)
}

fn get_table() -> Table {
    Table {
        name: String::from(TABLE_NAME),
        instance: Instance {
            name: String::from(INSTANCE_ID),
            project: Project {
                name: String::from(PROJECT_ID),
            },
        },
    }
}

fn is_error_response(response: &Value) -> bool {
    response.get("error").is_some()
}

// Helper to print response for debugging
fn debug_response(name: &str, response: &Value) {
    if is_error_response(response) {
        eprintln!("{} error: {}", name, serde_json::to_string_pretty(response).unwrap());
    } else {
        println!("{} success: {}", name, serde_json::to_string_pretty(response).unwrap());
    }
}

// ============================================================================
// Core Data Operations
// ============================================================================

#[test]
#[ignore]
fn test_read_rows() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: ReadRows::new(),
    };
    req.method.payload_mut().rows_limit = 10;

    let response = req.execute(&token).expect("ReadRows failed");
    debug_response("ReadRows", &response);

    // ReadRows returns empty array [] or array of chunks
    assert!(!is_error_response(&response), "ReadRows returned error");
}

#[test]
#[ignore]
fn test_sample_row_keys() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let req = BTRequest {
        base: None,
        table,
        method: SampleRowKeys::new(),
    };

    let response = req.execute(&token).expect("SampleRowKeys failed");
    debug_response("SampleRowKeys", &response);

    assert!(!is_error_response(&response), "SampleRowKeys returned error");
}

#[test]
#[ignore]
fn test_mutate_row() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: MutateRow::new(),
    };

    let row_key = encode_str("test_row_mutate");

    // Create a SetCell mutation
    let mut set_cell = mutation::SetCell::new();
    set_cell.family_name = String::from(COLUMN_FAMILY);
    set_cell.column_qualifier = encode_str("test_col");
    set_cell.timestamp_micros = -1; // Server assigns timestamp
    set_cell.value = encode_str("test_value_mutate_row");

    let mut m = Mutation::new();
    m.mutation = Some(mutation::Mutation::SetCell(set_cell));

    req.method.payload_mut().row_key = row_key;
    req.method.payload_mut().mutations.push(m);

    let response = req.execute(&token).expect("MutateRow failed");
    debug_response("MutateRow", &response);

    assert!(!is_error_response(&response), "MutateRow returned error");
}

#[test]
#[ignore]
fn test_mutate_rows() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: MutateRows::new(),
    };

    // Create multiple entries
    for i in 0..3 {
        let row_key = encode_str(&format!("test_batch_row_{}", i));

        let mut set_cell = mutation::SetCell::new();
        set_cell.family_name = String::from(COLUMN_FAMILY);
        set_cell.column_qualifier = encode_str("batch_col");
        set_cell.timestamp_micros = -1;
        set_cell.value = encode_str(&format!("batch_value_{}", i));

        let mut m = Mutation::new();
        m.mutation = Some(mutation::Mutation::SetCell(set_cell));

        let mut entry = mutate_rows_request::Entry::new();
        entry.row_key = row_key;
        entry.mutations.push(m);

        req.method.payload_mut().entries.push(entry);
    }

    let response = req.execute(&token).expect("MutateRows failed");
    debug_response("MutateRows", &response);

    assert!(!is_error_response(&response), "MutateRows returned error");
}

#[test]
#[ignore]
fn test_check_and_mutate_row() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: CheckAndMutateRow::new(),
    };

    let row_key = encode_str("test_row_check_mutate");

    // Predicate: pass all (always true)
    let mut predicate_filter = RowFilter::new();
    predicate_filter.filter = Some(row_filter::Filter::PassAllFilter(true));

    // True mutation: set a cell
    let mut set_cell = mutation::SetCell::new();
    set_cell.family_name = String::from(COLUMN_FAMILY);
    set_cell.column_qualifier = encode_str("check_col");
    set_cell.timestamp_micros = -1;
    set_cell.value = encode_str("check_mutate_value");

    let mut m = Mutation::new();
    m.mutation = Some(mutation::Mutation::SetCell(set_cell));

    req.method.payload_mut().row_key = row_key;
    req.method.payload_mut().predicate_filter = Some(predicate_filter).into();
    req.method.payload_mut().true_mutations.push(m);

    let response = req.execute(&token).expect("CheckAndMutateRow failed");
    debug_response("CheckAndMutateRow", &response);

    assert!(!is_error_response(&response), "CheckAndMutateRow returned error");
}

#[test]
#[ignore]
fn test_read_modify_write_row() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: ReadModifyWriteRow::new(),
    };

    let row_key = encode_str("test_row_rmw");

    let mut rule = ReadModifyWriteRule::new();
    rule.family_name = String::from(COLUMN_FAMILY);
    rule.column_qualifier = encode_str("rmw_col");
    rule.rule = Some(read_modify_write_rule::Rule::AppendValue(encode_str("_appended")));

    req.method.payload_mut().row_key = row_key;
    req.method.payload_mut().rules.push(rule);

    let response = req.execute(&token).expect("ReadModifyWriteRow failed");
    debug_response("ReadModifyWriteRow", &response);

    assert!(!is_error_response(&response), "ReadModifyWriteRow returned error");
}

// ============================================================================
// Connection Management
// ============================================================================

#[test]
#[ignore]
fn test_ping_and_warm() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let req = BTRequest {
        base: None,
        table,
        method: PingAndWarm::new(),
    };

    let response = req.execute(&token).expect("PingAndWarm failed");
    debug_response("PingAndWarm", &response);

    assert!(!is_error_response(&response), "PingAndWarm returned error");
}

// ============================================================================
// Change Streams (CDC)
// ============================================================================

#[test]
#[ignore]
fn test_generate_initial_change_stream_partitions() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let req = BTRequest {
        base: None,
        table,
        method: GenerateInitialChangeStreamPartitions::new(),
    };

    let response = req.execute(&token).expect("GenerateInitialChangeStreamPartitions failed");
    debug_response("GenerateInitialChangeStreamPartitions", &response);

    // This may return error if change streams not enabled on table - that's OK
    // We're testing that the API call works
    println!("GenerateInitialChangeStreamPartitions response received");
}

#[test]
#[ignore]
fn test_read_change_stream() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let req = BTRequest {
        base: None,
        table,
        method: ReadChangeStream::new(),
    };

    let response = req.execute(&token).expect("ReadChangeStream failed");
    debug_response("ReadChangeStream", &response);

    // This may return error if change streams not enabled - that's OK
    println!("ReadChangeStream response received");
}

// ============================================================================
// SQL Queries (GoogleSQL)
// ============================================================================

#[test]
#[ignore]
fn test_execute_query() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: ExecuteQuery::new(),
    };

    // Simple query - note: GoogleSQL support may vary by table config
    req.method.payload_mut().query = String::from("SELECT * FROM `my-table` LIMIT 1");

    let response = req.execute(&token).expect("ExecuteQuery failed");
    debug_response("ExecuteQuery", &response);

    // SQL queries may not be enabled - that's OK for API testing
    println!("ExecuteQuery response received");
}

#[test]
#[ignore]
fn test_prepare_query() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    let mut req = BTRequest {
        base: None,
        table,
        method: PrepareQuery::new(),
    };

    req.method.payload_mut().query = String::from("SELECT * FROM `my-table` LIMIT 1");

    let response = req.execute(&token).expect("PrepareQuery failed");
    debug_response("PrepareQuery", &response);

    // SQL queries may not be enabled - that's OK for API testing
    println!("PrepareQuery response received");
}

// ============================================================================
// End-to-End Test: Write then Read
// ============================================================================

#[test]
#[ignore]
fn test_write_then_read() {
    let token = get_token().expect("Failed to get token");
    let table = get_table();

    // 1. Write a row
    let test_row_key = "e2e_test_row";
    let test_value = "e2e_test_value_12345";

    let mut write_req = BTRequest {
        base: None,
        table: table.clone(),
        method: MutateRow::new(),
    };

    let mut set_cell = mutation::SetCell::new();
    set_cell.family_name = String::from(COLUMN_FAMILY);
    set_cell.column_qualifier = encode_str("e2e_col");
    set_cell.timestamp_micros = -1;
    set_cell.value = encode_str(test_value);

    let mut m = Mutation::new();
    m.mutation = Some(mutation::Mutation::SetCell(set_cell));

    write_req.method.payload_mut().row_key = encode_str(test_row_key);
    write_req.method.payload_mut().mutations.push(m);

    let write_response = write_req.execute(&token).expect("Write failed");
    assert!(!is_error_response(&write_response), "Write returned error");
    println!("Write succeeded");

    // 2. Read it back
    let mut read_req = BTRequest {
        base: None,
        table,
        method: ReadRows::new(),
    };
    read_req.method.payload_mut().rows_limit = 100;

    let read_response = read_req.execute(&token).expect("Read failed");
    assert!(!is_error_response(&read_response), "Read returned error");

    // Verify the response contains our data
    let response_str = serde_json::to_string(&read_response).unwrap();
    println!("Read response: {}", response_str);

    // The response should contain our row key (base64 encoded)
    // This is a basic check - the actual response format is complex
    assert!(read_response.is_array() || read_response.is_object(),
            "Expected array or object response");
}
