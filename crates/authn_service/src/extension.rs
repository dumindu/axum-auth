use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub token_id: Uuid,
    pub token_family_id: Uuid,
}
