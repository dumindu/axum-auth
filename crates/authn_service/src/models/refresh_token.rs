use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[unique(token_hash)]
#[index(token_family_id)]
pub struct RefreshToken {
    #[auto]
    pub created_at: Timestamp,

    pub expires_at: Timestamp,

    #[key]
    #[auto(uuid(v7))]
    pub id: Uuid,

    pub user_id: Uuid,

    pub device_id: Uuid,

    #[auto(uuid(v4))]
    pub token_family_id: Uuid,

    #[default(false)]
    pub is_revoked: bool,

    pub token_hash: Vec<u8>,
}
