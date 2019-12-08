use bigtable::*;
use protobuf::Message;

pub trait BigTable {
    type M: Message;

    fn payload(&self) -> &Self::M;
    fn payload_mut(&mut self) -> &mut Self::M;
    fn set_payload(&mut self, payload: Self::M);
    fn url_method(&self) -> &str;
    fn is_post(&self) -> bool;
}

macro_rules! method {
    ($name: ident, $proto: ty, $post: expr) => {
        pub struct $name {
            pub payload: $proto,
            pub url_method: String,
            pub is_post: bool,
        }

        impl $name {
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                // URL suffix from ident
                let mut x = stringify!($name).chars();
                let first = x.next().unwrap().to_lowercase().next().unwrap();
                let rest = x.as_str();

                $name {
                    payload: Default::default(),
                    url_method: format!(":{}{}", first, rest),
                    is_post: $post,
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
        }
    };
}

/// ### `ReadRows`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate bigtable as bt;
/// extern crate serde_json;
///
/// use bt::request::BTRequest;
/// use bt::data::ReadModifyWriteRule;
/// use bt::utils::*;
/// use bt::method::{BigTable, ReadRows};
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: ReadRows::new()
///                    };
///     let response = req.execute(&get_auth_token("dummy_credentials_file_for_tests.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn read_rows_doctest() {}
method!(ReadRows, ReadRowsRequest, true);

/// ### `SampleRowKeys`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate bigtable as bt;
/// extern crate serde_json;
///
/// use bt::request::BTRequest;
/// use bt::data::ReadModifyWriteRule;
/// use bt::utils::*;
/// use bt::method::{BigTable, SampleRowKeys};
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: SampleRowKeys::new()
///                    };
///     let response = req.execute(&get_auth_token("dummy_credentials_file_for_tests.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn sample_row_keys_doctest() {}
method!(SampleRowKeys, SampleRowKeysRequest, false);

/// ### `MutateRow`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate bigtable as bt;
/// extern crate serde_json;
/// extern crate protobuf;
///
/// use protobuf::RepeatedField;
///
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, MutateRow};
/// use bt::data::{Mutation, Mutation_DeleteFromRow};
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: MutateRow::new()
///                    };
///
///     let row_key = encode_str("r1");
///
///     let mut mutations: Vec<Mutation> = Vec::new();
///     let mut delete_row_mutation = Mutation::new();
///     delete_row_mutation.set_delete_from_row(Mutation_DeleteFromRow::new());
///     mutations.push(delete_row_mutation);
///
///     req.method.payload_mut().set_row_key(row_key);
///     req.method.payload_mut().set_mutations(RepeatedField::from_vec(mutations));
///
///     let response = req.execute(&get_auth_token("dummy_credentials_file_for_tests.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn mutate_row_doctest() {}
method!(MutateRow, MutateRowRequest, true);

/// ### `MutateRows`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate bigtable as bt;
/// extern crate serde_json;
/// extern crate protobuf;
///
/// use protobuf::RepeatedField;
///
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, MutateRows};
/// use bt::data::{Mutation, Mutation_DeleteFromRow};
/// use bt::bigtable::MutateRowsRequest_Entry;
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: MutateRows::new()
///                    };
///
///     let mut mutate_entries = Vec::new();
///     let mut mutate_entry = MutateRowsRequest_Entry::new();
///
///     let row_key = encode_str("r1");
///
///     let mut mutations: Vec<Mutation> = Vec::new();
///     let mut delete_row_mutation = Mutation::new();
///     delete_row_mutation.set_delete_from_row(Mutation_DeleteFromRow::new());
///
///     mutations.push(delete_row_mutation);
///     mutate_entry.set_mutations(RepeatedField::from_vec(mutations));
///     mutate_entry.set_row_key(row_key);
///     mutate_entries.push(mutate_entry);
///
///     req.method.payload_mut().set_entries(RepeatedField::from_vec(mutate_entries));
///
///     let response = req.execute(&get_auth_token("dummy_credentials_file_for_tests.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn mutate_rows_doctest() {}
method!(MutateRows, MutateRowsRequest, true);

/// ### `CheckAndMutateRow`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate bigtable as bt;
/// extern crate serde_json;
/// extern crate protobuf;
///
/// use protobuf::RepeatedField;
///
/// use bt::request::BTRequest;
/// use bt::utils::*;
/// use bt::method::{BigTable, CheckAndMutateRow};
/// use bt::data::{RowFilter, Mutation_DeleteFromRow, Mutation};
/// use bt::bigtable::MutateRowsRequest_Entry;
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: CheckAndMutateRow::new()
///                    };
///
///     let row_key = encode_str("r1");
///
///     let mut predicate_filter = RowFilter::new();
///     predicate_filter.set_pass_all_filter(true);
///
///     let mut mutations: Vec<Mutation> = Vec::new();
///     let mut delete_row_mutation = Mutation::new();
///     delete_row_mutation.set_delete_from_row(Mutation_DeleteFromRow::new());
///     mutations.push(delete_row_mutation);
///
///     req.method.payload_mut().set_row_key(row_key);
///     req.method.payload_mut().set_predicate_filter(predicate_filter);
///     req.method.payload_mut().set_true_mutations(RepeatedField::from_vec(mutations));
///
///     let response = req.execute(&get_auth_token("dummy_credentials_file_for_tests.json", true)?)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn check_and_mutate_row_doctes() {}
method!(CheckAndMutateRow, CheckAndMutateRowRequest, true);

/// ### `ReadWriteModifyRow`
///
/// ```
/// # #![allow(unused_imports)]
/// extern crate protobuf;
/// extern crate bigtable as bt;
/// extern crate serde_json;
///
/// use protobuf::RepeatedField;
///
/// use bt::request::BTRequest;
/// use bt::data::ReadModifyWriteRule;
/// use bt::utils::*;
/// use bt::method::{BigTable, ReadModifyWriteRow};
/// # use bt::error::BTErr;
///
/// fn main() {
/// # #[allow(dead_code)]
/// # fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///                              base: None,
///                              table: Default::default(),
///                              method: ReadModifyWriteRow::new()
///                   };
///
///     let token = get_auth_token("dummy_credentials_file_for_tests.json", true)?;
///
///     let mut rules: Vec<ReadModifyWriteRule> = Vec::new();
///     let mut rule = ReadModifyWriteRule::new();
///     rule.set_family_name(String::from("cf1"));
///     rule.set_column_qualifier(encode_str("r1"));
///     rule.set_append_value(encode_str("test_value"));
///
///     rules.push(rule);
///
///     req.method.payload_mut().set_row_key(encode_str("r1"));
///     req.method.payload_mut().set_rules(RepeatedField::from_vec(rules));
///
///     let response = req.execute(&token)?;
///     println!("{}", serde_json::to_string_pretty(&response)?);
/// # Ok(())
/// # }
/// }
/// ```
fn read_modify_write_doctest() {}
method!(ReadModifyWriteRow, ReadModifyWriteRowRequest, true);
