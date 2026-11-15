use axum::{extract::State, http::StatusCode, response::IntoResponse};
use nats_messages::authn::VerificationEmail;
use tracing::error;

use super::payload::RegistrationRequest;
use crate::{
    app::{ValidatedJson, ValidationErrorResponse, password, verification::Verification},
    errors::{Error, ErrorResponse},
    models::{Registration, User, UserStatus},
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/v1/register",
    tag = "auth",
    request_body = RegistrationRequest,
    responses(
        (status = 201, description = "A successful registration"),
        (status = 400, description = "An invalid payload", body = ErrorResponse),
        (status = 403, description = "A forbidden request"),
        (status = 409, description = "A duplicate entity", body = ErrorResponse),
        (status = 422, description = "An unprocessable payload", body = ValidationErrorResponse),
        (status = 500, description = "An internal failure", body = ErrorResponse)
    )
)]
pub async fn register(
    State(mut state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RegistrationRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = &payload.email.to_ascii_lowercase();

    let user_status = User::filter_by_email(email)
        .select(User::fields().status())
        .first()
        .exec(&mut state.db)
        .await
        .map_err(|error| {
            error!(target: "db", %error, "db fetch failed");
            Error::DbFetch
        })?;

    match user_status {
        Some(UserStatus::Disabled) => return Err(Error::Forbidden),
        Some(_) => return Err(Error::DuplicateEntity),
        None => {}
    }

    let verification_attempts = Registration::filter_by_email(email)
        .select(Registration::fields().verification_attempts())
        .first()
        .exec(&mut state.db)
        .await
        .map_err(|error| {
            error!(target: "db", %error, "db fetch failed");
            Error::DbFetch
        })?;

    if let Some(attempts) = verification_attempts {
        if attempts >= Registration::MAX_ATTEMPTS {
            todo!("wait and try - email")
        }
        return Err(Error::DuplicateEntity);
    }

    let verification = Verification::new().map_err(|error| {
        error!(target: "time", %error, "time gen failed");
        Error::TimeGen
    })?;

    let nats_msg = VerificationEmail {
        email: email.to_owned(),
        code: verification.code,
        expires_at: verification.expires_at.as_second(),
    };
    let nats_msg = rkyv::to_bytes::<rkyv::rancor::Error>(&nats_msg).map_err(|error| {
        error!(target: "rkyv", %error, "rkyv serialize failed");
        Error::MsgGen
    })?;

    state.js.publish("authn.email.verification", nats_msg.to_vec().into()).await.map_err(
        |error| {
            error!(target: "nats", %error, "nats publish failed");
            Error::MsgPub
        },
    )?;

    let pepper = state.secrets_conf.password_pepper.expose_str().as_bytes();
    let password_hash = password::hash(pepper, payload.password.as_bytes()).await?;

    toasty::create!(Registration {
        email,
        password_hash,
        verification_code_hash: verification.hash,
        verification_code_expires_at: verification.expires_at,
    })
    .exec(&mut state.db)
    .await
    .map_err(|error| {
        error!(target: "db", %error, "db insert failed");
        Error::DbInsert
    })?;

    Ok(StatusCode::CREATED)
}
