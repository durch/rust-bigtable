use rustc_serialize::base64::{ToBase64, STANDARD};

use goauth::auth::{JwtClaims, Token};
use goauth::scopes::Scope;
use goauth::get_token;
use smpl_jwt::{RSAKey, Jwt};

use error::BTErr;

pub fn encode_str(str: &str) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(str.as_bytes().to_base64(STANDARD).as_bytes());
    v
}

pub fn get_auth_token(token_url: &str, iss: &str, pk: &str) -> Result<Token, BTErr> {
    let claims = JwtClaims::new(String::from(iss),
                             Scope::CloudPlatform,
                             String::from(token_url),
                             None, Some(60));
    let key = RSAKey::from_pem(pk)?;
    let jwt = Jwt::new(claims, key, None);
    Ok(get_token(&jwt, None)?)
}