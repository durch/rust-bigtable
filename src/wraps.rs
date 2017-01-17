extern crate serde_json;
extern crate protobuf;

use protobuf::RepeatedField;

use request::BTRequest;
use data::{ReadModifyWriteRule, Mutation, Mutation_SetCell};
use bigtable::MutateRowsRequest_Entry;
use utils::*;
use method::{BigTable, ReadRows, ReadModifyWriteRow, MutateRows};
use error::BTErr;
use support::Table;
use goauth::auth::Token;


pub fn get_row_prefix(prefix: Option<&str>) -> String {
    match prefix {
        Some(x) => String::from(x),
        None => String::from("r")
    }
}

/// ```
/// extern crate bigtable as bt;
///
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
/// const ISS: &'static str = "service_acc@developer.gserviceaccount.com";
/// const PK: &'static str = "random_rsa_for_tests";
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn write_rows() -> Result<(), BTErr> {
///
///     let rows = Vec::new(); // put some real data here
///     let token = get_auth_token(TOKEN_URL, ISS, PK)?;
///     let table = Default::default();
///
///     let _ = wraps::bulk_write_rows(rows, "cf1", "test", None, &token, table);
///
/// #    Ok(())
/// # }
/// }
/// ```
pub fn bulk_write_rows(rows: Vec<String>,
                       column_family: &str,
                       column_qualifier: &str,
                       row_prefix: Option<&str>,
                       token: &Token,
                       table: Table) -> Result<(), BTErr> {

    let mut rows = rows;

    let prefix = get_row_prefix(row_prefix);

    let mut req = BTRequest {
                      base: None,
                      table: table,
                      method: MutateRows::new()
                };

    let mut mutate_entries = Vec::new();

    for (row_cnt, blob) in rows.drain(..).enumerate() {
        let mut mutate_entry = MutateRowsRequest_Entry::new();
        let row_key = encode_str(&format!("{}{}", prefix, row_cnt));
        mutate_entry.set_row_key(row_key);

        let mut mutations: Vec<Mutation> = Vec::new();
        let mut mutation = Mutation::new();

        let mut set_cell = Mutation_SetCell::new();
        set_cell.set_column_qualifier(encode_str(column_qualifier));
        set_cell.set_family_name(String::from(column_family));
        set_cell.set_timestamp_micros(-1);
        set_cell.set_value(encode_str(&blob));

        mutation.set_set_cell(set_cell);
        mutations.push(mutation);
        mutate_entry.set_mutations(RepeatedField::from_vec(mutations));
        mutate_entries.push(mutate_entry);
    }

    req.method.payload_mut().set_entries(RepeatedField::from_vec(mutate_entries));

    let _ = req.execute(token)?;
    Ok(())
}

/// ```
/// extern crate bigtable as bt;
///
/// use bt::utils::*;
/// use bt::error::BTErr;
/// use bt::wraps;
///
/// const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
/// const ISS: &'static str = "service_acc@developer.gserviceaccount.com";
/// const PK: &'static str = "random_rsa_for_tests";
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn write_rows() -> Result<(), BTErr> {
///
///     let rows = Vec::new(); // put some real data here
///     let token = get_auth_token(TOKEN_URL, ISS, PK)?;
///     let table = Default::default();
///
///     let _ = wraps::write_rows(rows, "cf1", "test", None, &token, table);
///
/// #    Ok(())
/// # }
/// }
/// ```
pub fn write_rows(rows: Vec<String>,
                  column_familiy: &str,
                  column_qualifier: &str,
                  row_prefix: Option<&str>,
                  token: &Token,
                  table: Table) -> Result<usize, BTErr> {

    let mut rows = rows;
    let prefix = get_row_prefix(row_prefix);
    let mut total = 0;

    for (row_cnt, blob) in rows.drain(..).enumerate() {

        let row_key = &format!("{}{}", prefix, row_cnt);

        let mut req = BTRequest {
            base: None,
            table: table.clone(),
            method: ReadModifyWriteRow::new()
        };

        let mut rules: Vec<ReadModifyWriteRule> = Vec::new();

        let mut rule = ReadModifyWriteRule::new();
        rule.set_family_name(String::from(column_familiy));
        rule.set_column_qualifier(encode_str(column_qualifier));
        rule.set_append_value(encode_str(&blob));

        rules.push(rule);

        req.method.payload_mut().set_row_key(encode_str(row_key));
        req.method.payload_mut().set_rules(RepeatedField::from_vec(rules));

        let _ = req.execute(token)?;
        total = row_cnt;
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
/// const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
/// const ISS: &'static str = "service_acc@developer.gserviceaccount.com";
/// const PK: &'static str = "random_rsa_for_tests";
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn read_rows(limit: i64) -> Result<(), BTErr> {
///
///    let token = get_auth_token(TOKEN_URL, ISS, PK)?;
///    let table = Default::default();
///
///    let _ = wraps::read_rows(table, &token, Some(limit));
///
/// # Ok(())
/// # }
/// }
/// ```
pub fn read_rows(table: Table,
                 token: &Token,
                 rows_limit: Option<i64>) -> Result<serde_json::Value, BTErr> {

    let mut req = BTRequest {
        base: None,
        table: table.clone(),
        method: ReadRows::new()
    };

    if let Some(x) = rows_limit {
        req.method.payload_mut().set_rows_limit(x);
    }

    let response = req.execute(token)?;
    Ok(response)
}