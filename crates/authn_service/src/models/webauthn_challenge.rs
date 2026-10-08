use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[index(expires_at)]
pub struct WebauthnChallenge {
    #[auto]
    pub created_at: Timestamp,

    pub expires_at: Timestamp,

    pub user_id: Option<Uuid>,

    pub device_id: Option<Uuid>,

    #[key]
    pub challenge_token: Vec<u8>,

    pub challenge_type: String,
}
