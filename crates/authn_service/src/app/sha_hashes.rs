use aws_lc_rs::digest::{SHA256, digest};
use uuid::Uuid;

pub fn device_hash(salt: &str, device_id: &Uuid) -> Vec<u8> {
    digest(&SHA256, &[salt.as_bytes(), device_id.as_bytes()].concat()).as_ref().to_vec()
}

pub fn refresh_token_hash(token: &str) -> Vec<u8> {
    digest(&SHA256, token.as_bytes()).as_ref().to_vec()
}
