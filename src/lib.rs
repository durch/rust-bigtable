#![feature(proc_macro)]

extern crate goauth;
extern crate smpl_jwt;
#[macro_use]
extern crate log;
extern crate curl;
extern crate curl_sys;

extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

extern crate pbuf;
//extern crate serde;
//extern crate serde_json;
//#[macro_use]
//extern crate serde_derive;

use goauth::auth::{JwtClaims, Token};
use goauth::scopes::Scope;
use goauth::get_token;
use smpl_jwt::{RSAKey, Jwt};

use curl::easy::{Easy, List};

use protobuf::Message;
use pbuf::read_one;

use std::io::prelude::*;
use std::io::{Read, BufRead};

use std::ffi::{CStr, CString};
use std::os::raw::{c_long};

extern crate solicit;
use solicit::http::client::CleartextConnector;
use solicit::http::client::tls::TlsConnector;
use solicit::client::SimpleClient;
use std::str;

pub mod error;

mod bigtable;
mod data;
mod status;
mod any;
mod wrappers;

use error::BTErr;
use bigtable::{ReadRowsRequest, ReadRowsResponse};
use grpc::iter::GrpcIterator;

pub const CURL_HTTP_VERSION_NONE: isize = 0;
pub const CURL_HTTP_VERSION_1_0: isize = 1;
pub const CURL_HTTP_VERSION_1_1: isize = 2;
pub const CURL_HTTP_VERSION_2_0: isize = 3;
pub const CURL_HTTP_VERSION_2: isize = 4;
pub const CURL_HTTP_VERSION_2TLS: isize = 5;
pub const CURL_HTTP_VERSION_2_PRIOR_KNOWLEDGE: isize = 6;

pub enum Http {
    None = CURL_HTTP_VERSION_NONE,
    Http1 = CURL_HTTP_VERSION_1_0,
    Http11 = CURL_HTTP_VERSION_1_1,
    Http20 = CURL_HTTP_VERSION_2_0,
    Http2 = CURL_HTTP_VERSION_2,
    Http2Tls = CURL_HTTP_VERSION_2TLS,
    Http2Prior = CURL_HTTP_VERSION_2_PRIOR_KNOWLEDGE
}

fn return_token() -> Result<Token, BTErr> {
    let token_url = "https://www.googleapis.com/oauth2/v4/token";
    let iss = "538580011331-compute@developer.gserviceaccount.com"; // https://developers.google.com/identity/protocols/OAuth2ServiceAccount
    let private_key_file = "pk.pem";

    let claims = JwtClaims::new(String::from(iss),
                             Scope::CloudPlatform,
                             String::from(token_url),
                             None, Some(60));
    let key = match RSAKey::from_pem(private_key_file) {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    };
    let jwt = Jwt::new(claims, key, None);
    Ok(get_token(&jwt, None)?)
}

//// Should make a post next :)
//#[derive(Serialize)]
//struct ReadRowsJson {
//    table_name: String
//}

fn https2_client() -> Result<(), BTErr> {
    // Connect to an HTTP/2 aware server
    let path = "pk.pem";
    let token = return_token()?;
    let root = "bigtableadmin.googleapis.com";
    let connector = TlsConnector::new(&root, &path);
    let mut client = SimpleClient::with_connector(connector).unwrap();
//    let response = client.get(b"/get", &[]).unwrap();
//    assert_eq!(response.stream_id, 1);
//    assert_eq!(response.status_code().unwrap(), 200);
//    // Dump the headers and the response body to stdout.
//    // They are returned as raw bytes for the user to do as they please.
//    // (Note: in general directly decoding assuming a utf8 encoding might not
//    // always work -- this is meant as a simple example that shows that the
//    // response is well formed.)
//    for header in response.headers.iter() {
//    println!("{}: {}",
//        str::from_utf8(header.name()).unwrap(),
//        str::from_utf8(header.value()).unwrap());
//    }
//println!("{}", str::from_utf8(&response.body).unwrap());

    let url = format!("/v2/projects/rustbigtable/instances?access_token={}", token.access_token());
    let b_url = url.as_bytes();
    let req_id1 = client.request(b"GET", &b_url, &[], None).unwrap();
    Ok(())
}

fn http2_client() -> Result<(), BTErr> {
    let token = return_token()?;
    let root = "bigtableadmin.googleapis.com";
    // Connect to an HTTP/2 aware server
    let connector = CleartextConnector::new(root);
    let mut client = SimpleClient::with_connector(connector).unwrap();
//    let response = client.get(b"/get", &[]).unwrap();
//    assert_eq!(response.stream_id, 1);
//    assert_eq!(response.status_code().unwrap(), 200);
    // Dump the headers and the response body to stdout.
    // They are returned as raw bytes for the user to do as they please.
    // (Note: in general directly decoding assuming a utf8 encoding might not
    // always work -- this is meant as a simple example that shows that the
    // response is well formed.)
//    for header in response.headers.iter() {
//    println!("{}: {}",
//        str::from_utf8(&header.0).unwrap(),
//        str::from_utf8(&header.1).unwrap());
//    }
//    println!("{}", str::from_utf8(&response.body).unwrap());
    // We can issue more requests after reading this one...
    // These calls block until the request itself is sent, but do not wait
    // for a response.
    let url = format!("/v2/projects/rustbigtable/instances?access_token={}", token.access_token());
    let b_url = url.as_bytes();
    let req_id1 = client.request(b"GET", &b_url, &[], None).unwrap();
//    let req_id2 = client.request(b"GET", b"/asdf", &[], None).unwrap();
    // Now we get a response for both requests... This does block.

//    let resp = client.get_response(req_id1).unwrap();
//    println!("{:?}", resp.body);
    Ok(())
}

pub fn list_instances() -> Result<ReadRowsResponse, BTErr> {
    let token = return_token()?;

    let mut response_data: Vec<u8> = vec!();
    let mut chunks: usize = 0;
    let mut data = ReadRowsRequest::new();

    let mut easy = Easy::new();
//    let url = &format!("https://bigtableadmin.googleapis.com/v2/projects/rustbigtable/instances?access_token={}", token.access_token());
    let url = "https://bigtable.googleapis.com/v2/projects/rustbigtable/instances/test-inst/tables/my-table:readRows";

    fn set_http2(easy: &mut Easy, http: Http) -> Result<(), BTErr> {
        Ok(easy.setopt_long(curl_sys::CURLOPT_HTTP_VERSION, http as c_long)?)
    }

//    set_http2(&mut easy, Http::Http2Prior)?;

    easy.url(url)?;

    easy.post(true)?;
    easy.post_field_size(data.compute_size() as u64)?;

    let mut list = List::new();
    let auth = format!("Authorization: {} {}", token.token_type(), token.access_token());
    list.append(&auth)?;
    list.append("Content-Type: application/octet-stream")?;
//    list.append("Accept: application/octet-stream")?;
//    list.append("Connection: Upgrade")?;
//    list.append("Upgrade: HTTP/2.0, SHTTP/1.3, IRC/6.9, RTA/x11")?;
    easy.http_headers(list)?;

    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(Message::write_to_bytes(&data).unwrap().len())
    //        Ok(b_data.read(buf).unwrap_or(0))
        })?;
        transfer.write_function(|data| {
//          response_data = String::from(std::str::from_utf8(data).expect("No response"));
            response_data.extend_from_slice(data);
            chunks += 1;
            Ok(data.len())
        })?;
        transfer.header_function(|header| {
            print!("header: {}", std::str::from_utf8(header).unwrap());
            true
        })?;
        transfer.perform()?;
    }

    println!("Bytes transfered: {}", response_data.len());
    println!("Response: {}", std::str::from_utf8(&response_data).unwrap());

    let mut slice = response_data.as_slice();
    Ok(protobuf::parse_from_bytes(&mut slice)?)
}

#[test]
fn test_get_token() {
    let easy = list_instances();
    match easy {
        Ok(x) => {
            println!("{}", x.get_chunks().len());
            for chunk in x.get_chunks() {
                println!("{:?}", chunk.get_family_name().get_value());
            }
        },
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_http2_con() {
    http2_client();
}
