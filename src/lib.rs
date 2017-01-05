#![allow(deprecated)]
#![allow(dead_code)]

extern crate goauth;
extern crate smpl_jwt;
#[macro_use]
extern crate log;
extern crate curl;
extern crate protobuf;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;

pub mod error;
pub mod method;
pub mod support;
pub mod request;
pub mod utils;

mod to_json;

// Generated mods
pub mod bigtable;
pub mod data;
pub mod status;
pub mod any;
pub mod wrappers;