use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{UserDevice, UserPassword};

#[derive(Debug, Model, Serialize, ToSchema)]
#[unique(email)]
pub struct User {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub created_at: Timestamp,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2027-01-01T00:00:00.123456Z"))]
    pub updated_at: Timestamp,

    #[key]
    #[auto(uuid(v4))]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    pub status: UserStatus,

    #[schema(example = "user@example.com")]
    pub email: String,

    #[has_one]
    #[serde(skip)]
    pub password: Deferred<Option<UserPassword>>,

    #[has_many]
    #[serde(skip)]
    pub devices: Vec<UserDevice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Embed, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[column(type = u8)]
pub enum UserStatus {
    #[column(variant = 0)]
    InProgress,
    #[column(variant = 1)]
    Active,
    #[column(variant = 2)]
    Disabled,
}
