use base64::{Engine as _, engine::general_purpose::STANDARD};
use goauth::auth::{JwtClaims, Token};
use goauth::credentials::Credentials;
use goauth::get_token;
use goauth::scopes::Scope;
use smpl_jwt::Jwt;
use std::str::FromStr;

use crate::error::BTErr;

pub fn encode_str(str: &str) -> Vec<u8> {
    STANDARD.encode(str).into_bytes()
}

pub fn get_auth_token(c: &str, fp: bool) -> Result<Token, BTErr> {
    let credentials = if fp {
        Credentials::from_file(c)?
    } else {
        Credentials::from_str(c)?
    };

    // AIDEV-NOTE: goauth 0.17 JwtClaims::new takes &[Scope] not &Scope
    let claims = JwtClaims::new(
        credentials.iss(),
        &[Scope::CloudPlatform],
        credentials.token_uri(),
        None,
        Some(60),
    );
    let jwt = Jwt::new(claims, credentials.rsa_key()?, None);
    Ok(get_token(&jwt, &credentials)?)
}

pub fn row_key_from_str(str: &str) -> Vec<u8> {
    let mut row_key = Vec::new();
    row_key.extend_from_slice(str.as_bytes());
    row_key
}
