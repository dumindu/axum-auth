use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;

use crate::{
    app::cookies,
    errors::{Error, ErrorResponse},
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
    State(state): State<AppState>,
    mut jar: CookieJar,
) -> Result<impl IntoResponse, Error> {
    // TODO
    jar = jar.add(cookies::refresh_token_expired(&state.cookies_conf));
    Ok(jar)
}
