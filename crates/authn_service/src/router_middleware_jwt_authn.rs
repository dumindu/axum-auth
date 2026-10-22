use std::{collections::HashSet, fmt::Debug};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, SigningKey, pkcs8::DecodePrivateKey};
use jiff::Timestamp;
use serde::{Deserialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    AppState, JwtTokenConf,
    app::jwt_token::{HEADER_ALG, HEADER_TYP, TOKEN_TYPE},
    errors::Error::{
        self, ExpiredToken, InvalidToken, InvalidTokenAudience, InvalidTokenIssuer,
        NotYetValidToken,
    },
    extension::{Token, User},
};

#[derive(Debug, Deserialize)]
pub struct DecodedHeader {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct DecodedClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: Uuid,

    #[serde(rename = "aud")]
    pub audience: HashSet<String>,

    #[serde(rename = "exp")]
    pub expires_at: u64,

    #[serde(rename = "nbf")]
    pub not_before: u64,

    #[serde(rename = "iat")]
    pub _issued_at: u64,

    #[serde(rename = "jti")]
    pub id: Uuid,

    #[serde(rename = "jtf")]
    pub family_id: Uuid,

    #[serde(rename = "did")]
    pub device_id: Uuid,
}

impl DecodedClaims {
    pub fn to_user(&self) -> User {
        User { id: self.subject, device_id: self.device_id }
    }

    pub fn to_token(&self, now: u64) -> Token {
        Token { id: self.id, family_id: self.family_id, expires_in: self.expires_at - now }
    }
}

fn base64_url_json_decode<T: DeserializeOwned>(encoded: &str) -> Result<T, anyhow::Error> {
    let json = URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(serde_json::from_slice(&json)?)
}

pub fn validate_token(conf: &JwtTokenConf, token: &str) -> Result<(User, Token), anyhow::Error> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        Err(ErrorKind::Invalid)?;
    }

    let encoded_header = parts[0];
    let encoded_claims = parts[1];
    let encoded_signature = parts[2];
    if encoded_header.is_empty() || encoded_claims.is_empty() || encoded_signature.is_empty() {
        Err(ErrorKind::Invalid)?;
    }

    let header: DecodedHeader = base64_url_json_decode(encoded_header)?;
    if header.alg != HEADER_ALG || header.typ != HEADER_TYP {
        Err(ErrorKind::Invalid)?;
    }

    let signature_bytes = URL_SAFE_NO_PAD.decode(encoded_signature)?;
    let signature = Signature::from_slice(&signature_bytes)?;

    let signing_key =
        SigningKey::from_pkcs8_pem(&conf.private_key.expose_str().replace("\\n", "\n"))?;
    let signing_input = format!("{}.{}", encoded_header, encoded_claims);
    signing_key.verify(signing_input.as_bytes(), &signature)?;

    let claims: DecodedClaims = base64_url_json_decode(encoded_claims)?;
    let now = Timestamp::now().as_second() as u64;

    if claims.issuer != conf.issuer {
        Err(ErrorKind::InvalidIssuer)?;
    }

    if claims.audience.is_disjoint(&conf.audience.0) {
        Err(ErrorKind::InvalidAudience)?;
    }

    if claims.not_before > now {
        Err(ErrorKind::NotYetValid)?;
    }

    if claims.expires_at < now {
        Err(ErrorKind::Expired)?;
    }

    Ok((claims.to_user(), claims.to_token(now)))
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("INVALID_TOKEN")]
    Invalid,
    #[error("INVALID_TOKEN_ISSUER")]
    InvalidIssuer,
    #[error("INVALID_TOKEN_AUDIENCE")]
    InvalidAudience,
    #[error("EXPIRED_TOKEN")]
    Expired,
    #[error("NOT_YT_VALID_TOKEN")]
    NotYetValid,
}

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use redis::AsyncTypedCommands;
use tracing::error;

use crate::{app::caches::token_blacklist_key, errors::Error::BlacklistedToken};

pub async fn authn_middleware(
    State(mut state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, Error> {
    let Some(auth_header) =
        request.headers().get(header::AUTHORIZATION).and_then(|header| header.to_str().ok())
    else {
        return Err(InvalidToken);
    };

    if !auth_header.starts_with(TOKEN_TYPE) {
        return Err(InvalidToken);
    }
    let token = &auth_header[7..];

    let path = request.uri().path();
    let need_token_info = matches!(path, "/v1/logout" | "/v1/refresh");

    let (user, token) = validate_token(&state.jwt_token_conf, token).map_err(|err| {
        if let Some(token_err) = err.downcast_ref::<ErrorKind>() {
            match token_err {
                ErrorKind::InvalidIssuer => InvalidTokenIssuer,
                ErrorKind::InvalidAudience => InvalidTokenAudience,
                ErrorKind::Expired => ExpiredToken,
                ErrorKind::NotYetValid => NotYetValidToken,
                _ => InvalidToken,
            }
        } else {
            InvalidToken
        }
    })?;

    let key = token_blacklist_key(&token.id);
    if state.cache.exists(&key).await.map_err(|err| {
        error!(target: "cache", "failed to exists: {key}: {err:?}");
        Error::CacheExists
    })? {
        return Err(BlacklistedToken);
    }

    request.extensions_mut().insert(user);
    if need_token_info {
        request.extensions_mut().insert(token);
    }

    Ok(next.run(request).await)
}
