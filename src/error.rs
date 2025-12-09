use curl::Error as curl_err;
use goauth::GoErr as go_err;
use protobuf::Error as pb_err;
use protobuf_json_mapping::PrintError as pb_json_err;
use serde_json::Error as serde_err;
use smpl_jwt::JwtErr as jwt_err;
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
    PBJsonErr(pb_json_err),
    JWTErr(jwt_err),
    UTF8Err(utf8_err),
    Unknown,
}

impl_from!(go_err, GOErr);
impl_from!(curl_err, CurlErr);
impl_from!(serde_err, SerdeErr);
impl_from!(pb_err, PBErr);
impl_from!(pb_json_err, PBJsonErr);
impl_from!(jwt_err, JWTErr);
impl_from!(utf8_err, UTF8Err);

impl std::fmt::Display for BTErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BTErr::GOErr(e) => e.fmt(f),
            BTErr::CurlErr(e) => e.fmt(f),
            BTErr::SerdeErr(e) => e.fmt(f),
            BTErr::PBErr(e) => e.fmt(f),
            BTErr::PBJsonErr(e) => e.fmt(f),
            BTErr::JWTErr(e) => e.fmt(f),
            BTErr::UTF8Err(e) => e.fmt(f),
            BTErr::Unknown => write!(f, "An unknown error has occurred"),
        }
    }
}

impl std::error::Error for BTErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BTErr::GOErr(e) => Some(e),
            BTErr::CurlErr(e) => Some(e),
            BTErr::SerdeErr(e) => Some(e),
            BTErr::PBErr(e) => Some(e),
            BTErr::PBJsonErr(e) => Some(e),
            BTErr::JWTErr(e) => Some(e),
            BTErr::UTF8Err(e) => Some(e),
            BTErr::Unknown => None,
        }
    }
}
