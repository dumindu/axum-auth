use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[index(user_id, device_id)]
pub struct SecurityAuditLog {
    #[auto]
    pub created_at: Timestamp,

    #[key]
    #[auto]
    pub id: Uuid,

    pub user_id: Option<Uuid>,

    pub device_id: Option<Uuid>,

    pub ip_address: String,

    pub user_agent: String,

    pub location: Option<String>,

    pub event_type: String,

    pub context: Option<String>,
}
