use protobuf::Message;
use bigtable::*;

pub trait BigTable {
    type M: Message;

    fn payload(&self) -> &Self::M;
    fn payload_mut(&mut self) -> &mut Self::M;
    fn url_method(&self) -> &str;
    fn is_post(&self) -> bool;
}

macro_rules! method {
    ($name: ident, $proto: ty, $post: expr) => {
        pub struct $name {
            payload: $proto,
            url_method: String,
            is_post: bool
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

                $name { payload: Default::default(),
                        url_method: format!(":{}{}", first, rest),
                        is_post: $post}
            }
        }

        impl BigTable for $name {
            type M = $proto;

            fn payload(&self) -> &Self::M {
                &self.payload
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

/// ### ReadRows
///
/// ```
/// extern crate google_bigtable as bt;
///
/// use bt::request::BTRequest;
/// use bt::data::ReadModifyWriteRule;
/// use bt::utils::{encode_str, get_auth_token};
/// use bt::method::{BigTable, ReadRows};
/// # use bt::error::BTErr;
///
/// const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
/// const ISS: &'static str = "service_acc@developer.gserviceaccount.com";
/// const PK: &'static str = "random_rsa_for_tests";
///
/// fn main() {
/// # fn wrapper() -> Result<(), BTErr> {
///     let req = BTRequest {
///                          base: None,
///                          table: Default::default(),
///                          method: ReadRows::new()
///                    };
///     let response = req.execute(&get_auth_token(TOKEN_URL, ISS, PK)?);
/// # Ok(())
/// # }
/// }
/// ```
fn read_rows_doctest() {}
method!(ReadRows, ReadRowsRequest, true);
method!(SampleRowKeys, SampleRowKeysRequest, false);
method!(MutateRow, MutateRowRequest, true);
method!(MutateRows, MutateRowsRequest, true);
method!(CheckAndMutateRow, CheckAndMutateRowRequest, true);
/// ### ReadWriteModify
///
/// ```
/// extern crate protobuf;
/// extern crate google_bigtable as bt;
///
/// use protobuf::RepeatedField;
///
/// use bt::request::BTRequest;
/// use bt::data::ReadModifyWriteRule;
/// use bt::utils::{encode_str, get_auth_token};
/// use bt::method::{BigTable, ReadModifyWriteRow};
/// # use bt::error::BTErr;
///
/// const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
/// const ISS: &'static str = "service_acc@developer.gserviceaccount.com";
/// const PK: &'static str = "random_rsa_for_tests";
///
/// fn main() {
/// # fn wrapper() -> Result<(), BTErr> {
///     let mut req = BTRequest {
///                              base: None,
///                              table: Default::default(),
///                              method: ReadModifyWriteRow::new()
///                   };
///
///     let token = get_auth_token(TOKEN_URL, ISS, PK)?;
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
/// # Ok(())
/// # }
/// }
/// ```
fn read_modify_write_doctest() {}
method!(ReadModifyWriteRow, ReadModifyWriteRowRequest, true);