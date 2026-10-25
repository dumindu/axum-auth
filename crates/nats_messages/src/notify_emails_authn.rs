use std::time::Duration;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize)]
pub struct Verification {
    pub email: String,
    pub code: String,
    pub expires_at: i64,
}

impl Verification {
    pub const ACK_TIMEOUT: Duration = Duration::from_secs(3);
    pub const CONSUMER: &'static str = "email_authn_verification_worker";
    pub const MAX_RETRIES: i64 = 5;
    pub const STREAM: &'static str = "notify";
    pub const SUBJECT: &'static str = "notify.email.authn.verification";
}

#[derive(Archive, Deserialize, Serialize)]
pub struct NewDevice {
    pub email: String,
    pub device_name: String,
    pub ip_address: String,
    pub location: String,
    pub created_at: i64,
}

impl NewDevice {
    pub const ACK_TIMEOUT: Duration = Duration::from_secs(3);
    pub const CONSUMER: &'static str = "email_authn_new_device_worker";
    pub const MAX_RETRIES: i64 = 5;
    pub const STREAM: &'static str = "notify";
    pub const SUBJECT: &'static str = "notify.email.authn.new_device";
}
