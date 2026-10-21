use std::collections::HashSet;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, SigningKey, pkcs8::DecodePrivateKey};
use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{JwtTokenConf, app::jwt_token};

pub const HEADER_ALG: &str = "EdDSA";
pub const HEADER_TYP: &str = "JWT";
pub const TOKEN_TYPE: &str = "Bearer";

#[derive(Serialize)]
pub struct Header {
    pub alg: &'static str,
    pub typ: &'static str,
}

#[derive(Serialize)]
pub struct Claims<'a> {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: &'a Uuid,

    #[serde(rename = "aud")]
    pub audience: HashSet<String>,

    #[serde(rename = "exp")]
    pub expires_at: usize,

    #[serde(rename = "nbf")]
    pub not_before: usize,

    #[serde(rename = "iat")]
    pub issued_at: usize,

    #[serde(rename = "jti")]
    pub id: &'a Uuid,

    #[serde(rename = "jtf")]
    pub family_id: &'a Uuid,

    #[serde(rename = "did")]
    pub device_id: &'a Uuid,
}

fn base64_url_json_encode<T: Serialize>(obj: &T) -> Result<String, anyhow::Error> {
    let json = serde_json::to_vec(obj)?;
    Ok(URL_SAFE_NO_PAD.encode(json))
}

pub fn generate_token_response(
    conf: &JwtTokenConf,
    user_id: &Uuid,
    device_id: &Uuid,
    token_id: &Uuid,
    token_family_id: &Uuid,
) -> Result<LoginResponse, anyhow::Error> {
    let now = Timestamp::now();
    let issued_at = now.as_second() as usize;

    let access_token_duration = conf.lifetime.0;
    let expires_in = access_token_duration.as_secs();

    let expires_at = now.checked_add(access_token_duration)?.as_second() as usize;

    let claims = jwt_token::Claims {
        issuer: conf.issuer.clone(),
        audience: conf.audience.0.clone(),
        subject: user_id,
        issued_at,
        not_before: issued_at,
        expires_at,
        id: token_id,
        family_id: token_family_id,
        device_id,
    };

    let signing_key =
        SigningKey::from_pkcs8_pem(&conf.private_key.expose_str().replace("\\n", "\n"))?;

    let header = Header { alg: HEADER_ALG, typ: HEADER_TYP };
    let encoded_header = base64_url_json_encode(&header)?;
    let encoded_claims = base64_url_json_encode(&claims)?;
    let signing_input = format!("{}.{}", encoded_header, encoded_claims);

    let signature: Signature = signing_key.sign(signing_input.as_bytes());
    let encoded_signature = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    let access_token = format!("{}.{}", signing_input, encoded_signature);

    Ok(LoginResponse::new(access_token, expires_in))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

impl LoginResponse {
    fn new(access_token: String, expires_in: i64) -> Self {
        Self { access_token, token_type: TOKEN_TYPE, expires_in }
    }
}
