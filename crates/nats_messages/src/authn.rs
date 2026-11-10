use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize)]
pub struct VerificationEmail {
    pub email: String,
    pub code: String,
    pub expires_at: i64,
}

#[derive(Archive, Deserialize, Serialize)]
pub struct NewDeviceEmail {
    pub email: String,
    pub device_name: String,
    pub ip_address: String,
    pub location: String,
    pub created_at: i64,
}
