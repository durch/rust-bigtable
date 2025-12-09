#![allow(deprecated)]
#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod method;
pub mod protos;
pub mod request;
pub mod support;
pub mod utils;
pub mod wraps;
