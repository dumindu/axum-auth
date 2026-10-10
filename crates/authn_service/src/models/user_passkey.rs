use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::User;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
#[unique(credential_id)]
pub struct UserPasskey {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub created_at: Timestamp,

    #[update(Timestamp::now())]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub last_used_at: Timestamp,

    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub user_id: Uuid,

    #[serde(skip)]
    pub signature_counter: u8,

    #[serde(skip)]
    pub credential_id: Vec<u8>,

    #[serde(skip)]
    pub public_key: Vec<u8>,

    #[belongs_to(key = user_id, references = id)]
    #[serde(skip)]
    pub user: toasty::Deferred<User>,
}
