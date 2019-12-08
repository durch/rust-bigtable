use bigtable::MutateRowsRequest_Entry;
use data::{Mutation, Mutation_SetCell, ReadModifyWriteRule};
use error::BTErr;
use goauth::auth::Token;
use method::{BigTable, MutateRows, ReadModifyWriteRow, ReadRows, SampleRowKeys};
use protobuf::RepeatedField;
use request::BTRequest;
use serde_json;
use support::Table;
use utils::*;

pub fn get_row_prefix(prefix: Option<&str>) -> String {
    match prefix {
        Some(x) => String::from(x),
        None => String::from("r"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub row_key: String,
    pub family: String,
    pub qualifier: String,
    pub value: String,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            row_key: String::from("dummy_row_key"),
            family: String::from("dummy_column_family"),
            qualifier: String::from("dummy_column_qualifier"),
            value: String::from("dummy_value"),
        }
    }
}

/// ```
/// extern crate bigtable as bt;
///
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn write_rows() -> Result<(), BTErr> {
///
///     let mut rows: Vec<wraps::Row> = vec!(wraps::Row::default()); // put some real data here
///     let token = get_auth_token("dummy_credentials_file_for_tests.json", true)?;
///     let table = Default::default();
///
///     let _ = wraps::bulk_write_rows(&mut rows, &token, table);
///
/// #    Ok(())
/// # }
/// }
/// ```
pub fn bulk_write_rows(rows: &mut Vec<Row>, token: &Token, table: Table) -> Result<String, BTErr> {
    let mut req = BTRequest {
        base: None,
        table,
        method: MutateRows::new(),
    };

    let mut mutate_entries = Vec::new();

    for row in rows.drain(..) {
        let mut mutate_entry = MutateRowsRequest_Entry::new();
        mutate_entry.set_row_key(encode_str(&row.row_key));

        let mut mutations: Vec<Mutation> = Vec::new();
        let mut mutation = Mutation::new();

        let set_cell = make_setcell_mutation(&row.qualifier, &row.family, encode_str(&row.value));

        mutation.set_set_cell(set_cell);
        mutations.push(mutation);
        mutate_entry.set_mutations(RepeatedField::from_vec(mutations));
        mutate_entries.push(mutate_entry);
    }

    req.method
        .payload_mut()
        .set_entries(RepeatedField::from_vec(mutate_entries));

    let response = req.execute(token)?;
    Ok(serde_json::to_string(&response)?)
}

/// ```
/// extern crate bigtable as bt;
///
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn write_rows() -> Result<(), BTErr> {
///
///     let mut rows: Vec<wraps::Row> = vec!(wraps::Row::default()); // put some real data here
///     let token = get_auth_token("dummy_credentials_file_for_tests.json", true)?;
///     let table = Default::default();
///
///     let _ = wraps::write_rows(&mut rows, &token, &table);
///
/// #    Ok(())
/// # }
/// }
/// ```
pub fn write_rows(rows: &mut Vec<Row>, token: &Token, table: &Table) -> Result<usize, BTErr> {
    let mut total = 0;

    for row in rows.drain(..) {
        let mut req = BTRequest {
            base: None,
            table: table.clone(),
            method: ReadModifyWriteRow::new(),
        };

        let mut rules: Vec<ReadModifyWriteRule> = Vec::new();

        let rule = make_readmodifywrite_rule(&row.qualifier, &row.family, encode_str(&row.value));

        rules.push(rule);

        req.method
            .payload_mut()
            .set_row_key(encode_str(&row.row_key));
        req.method
            .payload_mut()
            .set_rules(RepeatedField::from_vec(rules));

        let _ = req.execute(token)?;
        total += 1;
    }
    Ok(total)
}

/// ```
/// extern crate bigtable as bt;
///
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn read_rows(limit: i64) -> Result<(), BTErr> {
///
///    let token = get_auth_token("dummy_credentials_file_for_tests.json", true)?;
///    let table = Default::default();
///
///    let _ = wraps::read_rows(&table, &token, Some(limit));
///
/// # Ok(())
/// # }
/// }
/// ```
pub fn read_rows(
    table: &Table,
    token: &Token,
    rows_limit: Option<i64>,
) -> Result<serde_json::Value, BTErr> {
    let mut req = BTRequest {
        base: None,
        table: table.clone(),
        method: ReadRows::new(),
    };

    if let Some(x) = rows_limit {
        req.method.payload_mut().set_rows_limit(x);
    }

    let response = req.execute(token)?;
    Ok(response)
}

fn make_setcell_mutation(
    column_qualifier: &str,
    column_family: &str,
    blob: Vec<u8>,
) -> Mutation_SetCell {
    let mut set_cell = Mutation_SetCell::new();
    set_cell.set_column_qualifier(encode_str(column_qualifier));
    set_cell.set_family_name(String::from(column_family));
    set_cell.set_timestamp_micros(-1);
    set_cell.set_value(blob);
    set_cell
}

fn make_readmodifywrite_rule(
    column_qualifier: &str,
    column_familiy: &str,
    blob: Vec<u8>,
) -> ReadModifyWriteRule {
    let mut rule = ReadModifyWriteRule::new();
    rule.set_family_name(String::from(column_familiy));
    rule.set_column_qualifier(encode_str(column_qualifier));
    rule.set_append_value(blob);
    rule
}

fn sample_row_keys(token: &Token) -> Result<String, BTErr> {
    let req = BTRequest {
        base: None,
        table: Default::default(),
        method: SampleRowKeys::new(),
    };
    let response = req.execute(token)?;
    Ok(serde_json::to_string(&response)?)
}
