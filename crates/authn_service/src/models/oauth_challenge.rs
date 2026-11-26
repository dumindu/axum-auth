use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct OauthChallenge {
    #[auto]
    pub created_at: Timestamp,

    pub expires_at: Timestamp,

    #[key]
    pub state: Uuid,

    pub nonce: Option<Uuid>,
    pub code_verifier: String,
    pub provider: String,
    pub flow_type: String,
}
