use axum::{
    extract::{Extension, State},
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use redis::AsyncTypedCommands;
use tracing::error;

use crate::{
    app::{
        caches::{VALUE_TRUE, token_blacklist_key},
        cookies,
    },
    errors::{Error, ErrorResponse},
    extension::{Token, User},
    models::{RefreshToken, UserDevice},
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/v1/logout",
    tag = "auth",
    security(
        ("jwt_token" = [])
    ),
    responses(
        (
            status = 200, description = "A successful logout",
            headers(
                ("Set-Cookie" = String, description = "refresh_token; Secure;HTTP-only;session cookie;Max-Age=-2days"),
            )
        ),
        (status = 401, description = "unauthorized request", body = ErrorResponse),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn logout(
    State(mut state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(token): Extension<Token>,
    mut jar: CookieJar,
) -> Result<impl IntoResponse, Error> {
    RefreshToken::update_by_access_token_id(token.id)
        .is_revoked(true)
        .exec(&mut state.db)
        .await
        .map_err(|err| {
            error!(target: "db", "failed to update: {err:?}");
            Error::DbUpdate
        })?;

    UserDevice::update_by_id(user.device_id).exec(&mut state.db).await.map_err(|err| {
        error!(target: "db", "failed to update: {err:?}");
        Error::DbUpdate
    })?;

    let key = token_blacklist_key(&token.id);
    state.cache.set_ex(&key, VALUE_TRUE, token.expires_in).await.map_err(|err| {
        error!(target: "cache", "failed to set: {key}: {err:?}");
        Error::CacheSet
    })?;

    jar = jar.add(cookies::refresh_token_expired(&state.cookies_conf));
    Ok(jar)
}
