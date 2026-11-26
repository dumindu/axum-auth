use aws_lc_rs::{
    digest::{SHA256, digest},
    rand,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use uuid::Uuid;

pub fn device_hash(salt: &str, device_id: &Uuid) -> Vec<u8> {
    digest(&SHA256, &[salt.as_bytes(), device_id.as_bytes()].concat()).as_ref().to_vec()
}

pub fn refresh_token_hash(token: &str) -> Vec<u8> {
    digest(&SHA256, token.as_bytes()).as_ref().to_vec()
}

pub fn pkce_code_verifier_and_challenge() -> anyhow::Result<(String, String)> {
    let mut verifier_bytes = [0u8; 32];
    rand::fill(&mut verifier_bytes)?;

    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);
    let challenge = URL_SAFE_NO_PAD.encode(digest(&SHA256, verifier.as_bytes()).as_ref());

    Ok((verifier, challenge))
}
