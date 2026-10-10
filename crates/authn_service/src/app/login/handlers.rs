use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use jiff::{SignedDuration, Timestamp};
use tracing::error;
use uuid::Uuid;

use super::payload::LoginRequest;
use crate::{
    app::{ValidatedJson, ValidationErrorResponse, cookies, password, sha_hashes},
    errors::{Error, Error::TimeGen, ErrorResponse},
    models::{AccessTokenResponse, RefreshToken, User, UserDevice, UserStatus},
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/v1/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (
            status = 200, description = "A successful login", body = AccessTokenResponse,
            headers(
                ("Set-Cookie" = String, description = "Secure HTTP-only device_id cookie"),
                ("Set-Cookie" = String, description = "Secure HTTP-only refresh_token session cookie; `Max-Age=604800`"),
            )
        ),
        (status = 400, description = "An invalid payload", body = ErrorResponse),
        (status = 401, description = "An invalid credentials", body = ErrorResponse),
        (status = 403, description = "A forbidden request"),
        (status = 422, description = "An unprocessable payload", body = ValidationErrorResponse),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn login(
    State(mut state): State<AppState>,
    mut jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = &payload.email.to_lowercase();

    let user = User::filter_by_email(email)
        .include(User::fields().password())
        .get(&mut state.db)
        .await
        .map_err(|err| {
            if err.is_record_not_found() {
                Error::InvalidCredentials
            } else {
                error!(target: "database", "failed to fetch: {err:?}");
                Error::DbFetch
            }
        })?;

    if user.status.ne(&UserStatus::Active) {
        return Err(Error::Forbidden);
    }

    let password_hash = &user.password.get().as_ref().ok_or(Error::InvalidGrant)?.password_hash;
    let pepper = state.secrets_conf.password_pepper.expose_str().as_bytes();
    password::verify(pepper, payload.password.as_bytes(), password_hash).await?;

    let device_id_hash_salt = state.secrets_conf.device_id_hash_salt.expose_str();

    let device_hash = jar.get("device_id").and_then(|cookie| {
        let device_id = Uuid::parse_str(cookie.value()).ok()?;
        Some(sha_hashes::device_hash(device_id_hash_salt, &device_id))
    });
    let user_device = match device_hash {
        Some(hash) => UserDevice::filter_by_user_id_and_device_hash(user.id, hash)
            .first()
            .exec(&mut state.db)
            .await
            .map_err(|err| {
                error!(target: "database", "failed to fetch: {err:?}");
                Error::DbFetch
            })?,
        None => None,
    };

    let mut tx = state.db.transaction().await.map_err(|err| {
        error!(target: "database", "failed to begin transaction: {err:?}");
        Error::DbTransactionBegin
    })?;

    let user_device = match user_device {
        Some(mut existing_device) => {
            existing_device.update().exec(&mut tx).await.map_err(|err| {
                error!(target: "database", "failed to update: {err:?}");
                Error::DbUpdate
            })?;
            // todo: invalidate active access/ refresh tokens of same device
            existing_device
        }
        None => {
            let device_id = Uuid::new_v4();
            let device_hash = sha_hashes::device_hash(device_id_hash_salt, &device_id);

            let new_device = toasty::create!(UserDevice {
                user_id: user.id,
                device_hash,
                device_name: &payload.device_name,
            })
            .exec(&mut tx)
            .await
            .map_err(|err| {
                error!(target: "database", "failed to insert: {err:?}");
                Error::DbInsert
            })?;

            jar = jar.add(cookies::device_id(&device_id));
            new_device
        }
    };

    let raw_refresh_token = blake3::hash(Uuid::new_v4().as_bytes()).to_string();
    let refresh_token_expires_at =
        Timestamp::now().checked_add(SignedDuration::from_hours(24 * 7)).map_err(|_| TimeGen)?;

    let refresh_token = toasty::create!(RefreshToken {
        expires_at: refresh_token_expires_at,
        user_id: user.id,
        device_id: user_device.id,
        token_hash: sha_hashes::refresh_token_hash(&raw_refresh_token),
    })
    .exec(&mut tx)
    .await
    .map_err(|err| {
        error!(target: "database", "failed to insert: {err:?}");
        Error::DbInsert
    })?;

    jar = jar.add(cookies::refresh_token(&refresh_token.token_family_id, &raw_refresh_token));

    tx.commit().await.map_err(|err| {
        error!(target: "database", "failed to commit transaction: {err:?}");
        Error::DbTransactionCommit
    })?;

    Ok(jar)
}
