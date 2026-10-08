use jiff::Timestamp;
use uuid::Uuid;

use super::User;

#[derive(Debug, toasty::Model)]
#[unique(provider, provider_sub_id)]
#[index(user_id)]
pub struct UserIdentity {
    #[auto]
    pub created_at: Timestamp,

    #[key]
    pub id: Uuid,

    pub user_id: Uuid,

    pub provider: String,

    pub provider_sub_id: String,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,
}
