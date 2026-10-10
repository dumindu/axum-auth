use anyhow::Error;
use blake3::hash;
use jiff::{Timestamp, ToSpan, Zoned};
use rand::{RngExt, distr::Alphanumeric};

const LENGTH: u8 = 6;
const LIFETIME_IN_MIN: i32 = 20;

pub struct Verification {
    pub code: String,
    pub hash: Vec<u8>,
    pub expires_at: Timestamp,
}

impl Verification {
    pub fn new() -> Result<Self, Error> {
        let mut rng = rand::rng();
        let code = (0..LENGTH)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>()
            .to_uppercase();
        let hash = hash(code.as_bytes()).to_hex().as_bytes().to_vec();
        let expires_at = Zoned::now().checked_add(LIFETIME_IN_MIN.minutes())?.timestamp();

        Ok(Self { code, hash, expires_at })
    }
}
