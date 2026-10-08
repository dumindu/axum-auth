use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::User;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
#[unique(user_id, device_hash)]
pub struct UserDevice {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub created_at: Timestamp,

    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub last_used_at: Timestamp,

    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub user_id: Uuid,

    #[schema(example = "iPhone 18 Pro")]
    pub device_name: String,

    #[serde(skip)]
    pub device_hash: Vec<u8>,

    #[serde(skip)]
    pub revoked_at: Option<Timestamp>,

    #[belongs_to(key = user_id, references = id)]
    #[serde(skip)]
    pub user: toasty::Deferred<User>,
}
