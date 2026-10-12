use axum_extra::extract::cookie::{Cookie, SameSite::Strict};
use cookie::time::Duration;
use uuid::Uuid;

use crate::CookiesConf;

pub fn refresh_token<'a, 'b>(
    config: &CookiesConf,
    family_id: &'a Uuid,
    token: &'a str,
) -> Cookie<'b> {
    Cookie::build(("refresh_token", format!("{}:{}", family_id, token)))
        .http_only(true)
        .secure(false)
        .domain(config.domain.to_owned())
        .path("/v1/refresh")
        .max_age(Duration::days(config.refresh_token_lifetime_in_days))
        .same_site(Strict)
        .build()
}

pub fn device_id<'a, 'b>(config: &CookiesConf, device_id: &'a Uuid) -> Cookie<'b> {
    Cookie::build(("device_id", device_id.to_string()))
        .http_only(true)
        .secure(false)
        .domain(config.domain.to_owned())
        .path("/")
        .max_age(Duration::days(config.device_id_lifetime_in_days))
        .same_site(Strict)
        .build()
}
