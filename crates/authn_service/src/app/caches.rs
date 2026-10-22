use uuid::Uuid;

pub const VALUE_TRUE: &str = "1";

pub fn token_blacklist_key(id: &Uuid) -> String {
    format!("jwt:bl:{}", id)
}
