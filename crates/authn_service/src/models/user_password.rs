use jiff::Timestamp;
use uuid::Uuid;

use super::User;

#[derive(Debug, toasty::Model)]
pub struct UserPassword {
    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,

    #[key]
    pub user_id: Uuid,

    pub password_hash: String,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,
}
