use curl::Error as curl_err;
use goauth::error::GOErr as go_err;
use protobuf::error::ProtobufError as pb_err;
use serde_json::Error as serde_err;
use smpl_jwt::error::JwtErr as jwt_err;
use std;
use std::str::Utf8Error as utf8_err;

macro_rules! impl_from {
    ($type_: ident, $enum_ty: ident) => {
        impl From<$type_> for BTErr {
            fn from(e: $type_) -> BTErr {
                BTErr::$enum_ty(e)
            }
        }
    };
}

#[derive(Debug)]
pub enum BTErr {
    GOErr(go_err),
    CurlErr(curl_err),
    SerdeErr(serde_err),
    PBErr(pb_err),
    JWTErr(jwt_err),
    UTF8Err(utf8_err),
    Unknown,
}

impl_from!(go_err, GOErr);
impl_from!(curl_err, CurlErr);
impl_from!(serde_err, SerdeErr);
impl_from!(pb_err, PBErr);
impl_from!(jwt_err, JWTErr);
impl_from!(utf8_err, UTF8Err);

impl std::fmt::Display for BTErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BTErr::GOErr(ref e) => e.fmt(f),
            BTErr::CurlErr(ref e) => e.fmt(f),
            BTErr::SerdeErr(ref e) => e.fmt(f),
            BTErr::PBErr(ref e) => e.fmt(f),
            BTErr::JWTErr(ref e) => e.fmt(f),
            BTErr::UTF8Err(ref e) => e.fmt(f),
            BTErr::Unknown => write!(f, "An unknown error has occured"),
        }
    }
}

impl std::error::Error for BTErr {
    fn description(&self) -> &str {
        match *self {
            BTErr::GOErr(ref e) => e.description(),
            BTErr::CurlErr(ref e) => e.description(),
            BTErr::SerdeErr(ref e) => e.description(),
            BTErr::PBErr(ref e) => e.description(),
            BTErr::JWTErr(ref e) => e.description(),
            BTErr::UTF8Err(ref e) => e.description(),
            BTErr::Unknown => "unknown error",
        }
    }
}
