use axum_extra::extract::cookie::{Cookie, SameSite::Strict};
use cookie::time::Duration;
use uuid::Uuid;

pub fn refresh_token<'a, 'b>(family_id: &'a Uuid, token: &'a str) -> Cookie<'b> {
    Cookie::build(("refresh_token", format!("{}:{}", family_id, token)))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::days(7))
        .same_site(Strict)
        .build()
}

pub fn device_id<'a, 'b>(device_id: &'a Uuid) -> Cookie<'b> {
    Cookie::build(("device_id", device_id.to_string()))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::days(365))
        .same_site(Strict)
        .build()
}
