use jiff::Timestamp;

#[derive(Debug, toasty::Model)]
pub struct Registration {
    #[auto]
    pub created_at: Timestamp,

    pub verification_code_expires_at: Timestamp,
    pub verification_attempts: u8,
    pub verification_code_hash: Vec<u8>,
    pub password_hash: String,

    #[key]
    pub email: String,
}
