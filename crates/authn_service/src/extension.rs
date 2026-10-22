use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub device_id: Uuid,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub family_id: Uuid,
    pub expires_in: u64,
}
