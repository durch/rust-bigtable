use curl::easy::{Easy, List};
use crate::error::BTErr;
use goauth::auth::Token;
use crate::method::{BigTable, ReadRows, UrlScope};
use protobuf_json_mapping;
use serde_json;
use serde_json::Value;
use std;
use std::io::Read;
use crate::support::Table;

pub struct BTRequest<'a, T: BigTable> {
    pub base: Option<&'a str>,
    pub table: Table,
    pub method: T,
}

impl<'a> Default for BTRequest<'a, ReadRows> {
    fn default() -> Self {
        BTRequest {
            base: None,
            table: Default::default(),
            method: ReadRows::new(),
        }
    }
}

impl<'a, T: BigTable> BTRequest<'a, T> {
    // AIDEV-NOTE: form_url handles both table-level and instance-level API methods
    pub fn form_url(&self) -> Result<String, BTErr> {
        let base = match self.base {
            Some(x) => x,
            None => "https://bigtable.googleapis.com/v2",
        };
        match self.method.url_scope() {
            UrlScope::Table => Ok(format!(
                "{}/projects/{}/instances/{}/tables/{}{}",
                base,
                self.table.instance.project.name,
                self.table.instance.name,
                self.table.name,
                self.method.url_method()
            )),
            UrlScope::Instance => Ok(format!(
                "{}/projects/{}/instances/{}{}",
                base,
                self.table.instance.project.name,
                self.table.instance.name,
                self.method.url_method()
            )),
        }
    }

    pub fn execute(&self, token: &Token) -> Result<Value, BTErr> {
        let mut response_data: Vec<u8> = Vec::new();
        let mut easy = Easy::new();

        // AIDEV-NOTE: protobuf 3.x uses print_to_string for JSON serialization
        let s_payload = protobuf_json_mapping::print_to_string(self.method.payload())?;
        let mut b_payload = s_payload.as_bytes();

        easy.url(&self.form_url()?)?;

        if self.method.is_post() {
            easy.post(true)?;
            easy.post_field_size(b_payload.len() as u64)?;
        }

        easy.http_headers(gen_headers(token)?)?;

        {
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| Ok(b_payload.read(buf).unwrap_or(0)))?;
            transfer.write_function(|response| {
                response_data.extend_from_slice(response);
                Ok(response.len())
            })?;
            transfer.header_function(|header| {
                debug!("header: {}", std::str::from_utf8(header).unwrap());
                true
            })?;
            transfer.perform()?;
        }

        let response_str = std::str::from_utf8(&response_data)?;

        debug!("Bytes transfered: {}", response_data.len());
        debug!("Response: {}", response_str);

        Ok(serde_json::from_str(response_str)?)
    }
}

fn gen_headers(token: &Token) -> Result<List, BTErr> {
    let mut list = List::new();
    let auth = format!(
        "Authorization: {} {}",
        token.token_type(),
        token.access_token()
    );
    list.append(&auth)?;
    list.append("Content-Type: application/json")?;
    Ok(list)
}
