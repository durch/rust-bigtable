// AIDEV-NOTE: Updated for protobuf 3.x - RepeatedField replaced with Vec,
// nested types now use module-based naming (e.g., mutate_rows_request::Entry)
use crate::protos::bigtable::mutate_rows_request;
use crate::protos::data::{mutation, Mutation, ReadModifyWriteRule, read_modify_write_rule};
use crate::error::BTErr;
use goauth::auth::Token;
use crate::method::{BigTable, MutateRows, ReadModifyWriteRow, ReadRows, SampleRowKeys};
use crate::request::BTRequest;
use serde_json;
use crate::support::Table;
use crate::utils::*;

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

/// ```ignore
/// use bigtable as bt;
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn write_rows() -> Result<(), BTErr> {
///     let mut rows: Vec<wraps::Row> = vec!(wraps::Row::default());
///     let token = get_auth_token("credentials.json", true)?;
///     let table = Default::default();
///     let _ = wraps::bulk_write_rows(&mut rows, &token, table);
///     Ok(())
/// }
/// ```
pub fn bulk_write_rows(rows: &mut Vec<Row>, token: &Token, table: Table) -> Result<String, BTErr> {
    let mut req = BTRequest {
        base: None,
        table,
        method: MutateRows::new(),
    };

    for row in rows.drain(..) {
        let mut mutate_entry = mutate_rows_request::Entry::new();
        mutate_entry.row_key = encode_str(&row.row_key);

        let mut mutation = Mutation::new();
        let set_cell = make_setcell_mutation(&row.qualifier, &row.family, encode_str(&row.value));
        mutation.mutation = Some(mutation::Mutation::SetCell(set_cell));

        mutate_entry.mutations.push(mutation);
        req.method.payload_mut().entries.push(mutate_entry);
    }

    let response = req.execute(token)?;
    Ok(serde_json::to_string(&response)?)
}

/// ```ignore
/// use bigtable as bt;
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn write_rows() -> Result<(), BTErr> {
///     let mut rows: Vec<wraps::Row> = vec!(wraps::Row::default());
///     let token = get_auth_token("credentials.json", true)?;
///     let table = Default::default();
///     let _ = wraps::write_rows(&mut rows, &token, &table);
///     Ok(())
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

        let rule = make_readmodifywrite_rule(&row.qualifier, &row.family, encode_str(&row.value));

        req.method.payload_mut().row_key = encode_str(&row.row_key);
        req.method.payload_mut().rules.push(rule);

        let _ = req.execute(token)?;
        total += 1;
    }
    Ok(total)
}

/// ```ignore
/// use bigtable as bt;
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// fn read_rows(limit: i64) -> Result<(), BTErr> {
///    let token = get_auth_token("credentials.json", true)?;
///    let table = Default::default();
///    let _ = wraps::read_rows(&table, &token, Some(limit));
///    Ok(())
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
        req.method.payload_mut().rows_limit = x;
    }

    let response = req.execute(token)?;
    Ok(response)
}

fn make_setcell_mutation(
    column_qualifier: &str,
    column_family: &str,
    blob: Vec<u8>,
) -> mutation::SetCell {
    let mut set_cell = mutation::SetCell::new();
    set_cell.column_qualifier = encode_str(column_qualifier);
    set_cell.family_name = String::from(column_family);
    set_cell.timestamp_micros = -1;
    set_cell.value = blob;
    set_cell
}

fn make_readmodifywrite_rule(
    column_qualifier: &str,
    column_family: &str,
    blob: Vec<u8>,
) -> ReadModifyWriteRule {
    let mut rule = ReadModifyWriteRule::new();
    rule.family_name = String::from(column_family);
    rule.column_qualifier = encode_str(column_qualifier);
    rule.rule = Some(read_modify_write_rule::Rule::AppendValue(blob));
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
